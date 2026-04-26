//! Compile a `ProblemSpec` (or one of its variants) into an executable `Plan`.
//!
//! Free samplers become enumerable slots; derived expressions become bytecode
//! programs; constraints become boolean programs. Anything we cannot encode
//! returns `CompileError::Unsupported`, signalling the caller to fall back
//! to legacy rejection sampling.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::gpu::bytecode::{Op, Program};
use crate::gpu::hoist::{expr_to_bytecode_input, try_hoist};

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("circular dependency: {0}")]
    Circular(String),
    #[error("invalid sampler '{0}'")]
    BadSampler(String),
}

/// Per-sampler enumerable domain.
#[derive(Debug, Clone)]
pub struct SamplerSlot {
    pub var_slot: u32,
    pub values: Vec<i32>,
}

/// Either populate a slot from a sampler index, or evaluate a derived program.
#[derive(Debug, Clone)]
pub enum EvalStep {
    Sampler { sampler_idx: u32, var_slot: u32 },
    Derived { var_slot: u32, program: Program },
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub var_names: Vec<String>,
    pub var_to_slot: BTreeMap<String, u32>,
    pub sampler_slots: Vec<SamplerSlot>,
    pub eval_steps: Vec<EvalStep>,
    /// Constraints that compile to integer bytecode and run on the enumerator
    /// kernel. Bool program — top of stack must be 1 to keep.
    pub constraints: Vec<Program>,
    /// Constraints that need the full SymEngine-resolved VarMap. Evaluated on
    /// CPU per surviving sampler tuple after `resolver::resolve_with_preset`.
    pub cpu_constraints: Vec<String>,
    /// Slots whose values define the GPU-side hash key — set to the sampler
    /// slots so we never collapse genuinely distinct problems whose derived
    /// or answer values happen to collide. Symmetric duplicates (same
    /// rendered question/answer from distinct sampler tuples) are caught by
    /// the post-render `DashSet` dedup in `enumerator::enumerate_one`.
    pub dedup_slots: Vec<u32>,
    pub total_combos: u64,
}

impl Plan {
    pub fn slot(&self, name: &str) -> Option<u32> {
        self.var_to_slot.get(name).copied()
    }
}

/// Build a Plan from variables + constraints.
///
/// Pipeline: try a symbolic hoist pass first (lowers `derivative`,
/// `expand`, `evaluate`, …). For each variable that hoists to integer
/// arithmetic in samplers, compile its symbolically-resolved form to
/// bytecode. Vars that don't hoist (e.g. still symbolic in `x`) are skipped
/// here — the GPU enumerator still gets its sampler tuple, and the CPU
/// render path recomputes them via `resolver::resolve_with_preset`.
///
/// Returns `Unsupported` if the answer var or any constraint cannot be
/// reduced to integer arithmetic — caller falls back to legacy rejection
/// sampling.
pub fn compile(
    variables: &BTreeMap<String, String>,
    constraints: &[String],
) -> Result<Plan, CompileError> {
    let order = topo_sort(variables)?;

    let mut var_names: Vec<String> = Vec::with_capacity(order.len());
    let mut var_to_slot: BTreeMap<String, u32> = BTreeMap::new();
    for (i, name) in order.iter().enumerate() {
        var_names.push(name.clone());
        var_to_slot.insert(name.clone(), i as u32);
    }

    // Symbolic hoist pass: best-effort, fall through if it raises.
    let hoist = try_hoist(variables).ok();

    let mut sampler_slots: Vec<SamplerSlot> = Vec::new();
    let mut eval_steps: Vec<EvalStep> = Vec::new();
    let mut total_combos: u64 = 1;

    for (i, name) in order.iter().enumerate() {
        let var_slot = i as u32;
        let definition = variables.get(name).expect("topo result has name");
        let def = definition.trim();

        if let Some(values) = compile_sampler(def)? {
            let cardinality = values.len() as u64;
            total_combos = total_combos.saturating_mul(cardinality.max(1));
            if total_combos > MAX_COMBOS {
                return Err(CompileError::Unsupported(format!(
                    "combos > {MAX_COMBOS} ({total_combos}) — tile in M3"
                )));
            }
            let sampler_idx = sampler_slots.len() as u32;
            sampler_slots.push(SamplerSlot { var_slot, values });
            eval_steps.push(EvalStep::Sampler {
                sampler_idx,
                var_slot,
            });
        } else if let Some(prog) = try_compile_var(name, def, &var_to_slot, hoist.as_ref()) {
            eval_steps.push(EvalStep::Derived {
                var_slot,
                program: prog,
            });
        } else {
            // Unhoistable var (e.g. `solve(eq, x)`, `integrate`, sqrt-valued).
            // Leave its slot at zero on the GPU; the CPU render path
            // recomputes it via `resolver::resolve_with_preset`.
            let mut p = Program::new();
            p.emit_const(0);
            p.finish();
            eval_steps.push(EvalStep::Derived {
                var_slot,
                program: p,
            });
        }
    }

    // Constraints split into GPU-bytecode and CPU-after-resolve. CPU
    // constraints typically touch unhoisted vars (e.g. `det(M) != 0` when M
    // is a matrix literal that the bytecode VM can't evaluate).
    let mut constraint_programs = Vec::new();
    let mut cpu_constraints = Vec::new();
    for c in constraints {
        match compile_constraint(c, &var_to_slot) {
            Ok(prog) => constraint_programs.push(prog),
            Err(_) => cpu_constraints.push(c.clone()),
        }
    }

    // GPU-side dedup hashes on the full sampler tuple. Symmetric duplicates
    // (e.g. quadratic_formula's (r1, r2) ↔ (r2, r1)) hash to *different*
    // GPU keys but resolve to identical (question, answer) post-render, where
    // the CPU DashSet collapses them. Hashing on sampler slots only guarantees
    // we never drop genuinely distinct problems whose derived/answer happen
    // to collide.
    let dedup_slots: Vec<u32> = sampler_slots.iter().map(|s| s.var_slot).collect();

    Ok(Plan {
        var_names,
        var_to_slot,
        sampler_slots,
        eval_steps,
        constraints: constraint_programs,
        cpu_constraints,
        dedup_slots,
        total_combos,
    })
}

/// Try to compile a derived variable to bytecode.
/// 1. If the raw expression is already arithmetic, compile directly.
/// 2. Else, look it up in the hoist table; if its symbolic form reduces to
///    integer arithmetic in sampler symbols, compile that.
fn try_compile_var(
    name: &str,
    def: &str,
    var_to_slot: &BTreeMap<String, u32>,
    hoist: Option<&crate::gpu::hoist::HoistResult>,
) -> Option<Program> {
    if let Ok(prog) = compile_expression(def, var_to_slot) {
        return Some(prog);
    }
    let hoist = hoist?;
    let h = hoist.by_var.get(name)?;
    if !h.free.is_subset(&hoist.sampler_names) {
        return None;
    }
    let lowered = expr_to_bytecode_input(&h.expr).ok()?;
    compile_expression(&lowered, var_to_slot).ok()
}

const MAX_COMBOS: u64 = 16_000_000;

/// Topological sort over variable dependency graph. Mirrors `resolver::topo_sort`
/// but returns owned strings (we own them in the Plan).
fn topo_sort(variables: &BTreeMap<String, String>) -> Result<Vec<String>, CompileError> {
    let var_names: HashSet<&str> = variables.keys().map(|s| s.as_str()).collect();
    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, def) in variables {
        let mut my_deps = Vec::new();
        let referenced = scan_identifiers(def);
        for other in &referenced {
            if other != name && var_names.contains(other.as_str()) {
                let s = variables.get_key_value(other).unwrap().0.as_str();
                my_deps.push(s);
            }
        }
        deps.insert(name.as_str(), my_deps);
    }

    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for (name, d) in &deps {
        in_degree.insert(name, d.len());
    }
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(&n, _)| n)
        .collect();
    queue.sort();

    let mut order: Vec<String> = Vec::new();
    while !queue.is_empty() {
        let name = queue.remove(0);
        order.push(name.to_string());
        for (other, other_deps) in &deps {
            if other_deps.contains(&name) {
                let deg = in_degree.get_mut(other).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(other);
                }
            }
        }
        queue.sort();
    }

    if order.len() != var_names.len() {
        let remaining: Vec<&str> = var_names
            .iter()
            .filter(|n| !order.iter().any(|o| o == **n))
            .copied()
            .collect();
        return Err(CompileError::Circular(remaining.join(", ")));
    }

    Ok(order)
}

fn scan_identifiers(s: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let ident = &s[start..i];
            // Skip operator words and reserved samplers
            if !matches!(
                ident,
                "and" | "or" | "not" | "if" | "else" | "true" | "false"
            ) {
                out.insert(ident.to_string());
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Attempt to parse a sampler definition. Returns `Some(values)` if it's an
/// enumerable sampler we support; `None` if it's a derived expression; `Err`
/// if it's an unsupported sampler shape (continuous, vector, matrix, …).
fn compile_sampler(def: &str) -> Result<Option<Vec<i32>>, CompileError> {
    let paren = match def.find('(') {
        Some(p) => p,
        None => return Ok(None),
    };
    if !def.ends_with(')') {
        return Ok(None);
    }
    let name = &def[..paren];
    let args_str = &def[paren + 1..def.len() - 1];
    let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

    match name {
        "integer" => {
            let (lo, hi) = parse_pair(&args, name)?;
            if hi - lo + 1 > MAX_COMBOS as i64 {
                return Err(CompileError::Unsupported(format!(
                    "integer range too wide: {def}"
                )));
            }
            Ok(Some((lo..=hi).map(|v| v as i32).collect()))
        }
        "nonzero" => {
            let (lo, hi) = parse_pair(&args, name)?;
            Ok(Some(
                (lo..=hi).filter(|&v| v != 0).map(|v| v as i32).collect(),
            ))
        }
        "choice" => {
            let mut vals = Vec::with_capacity(args.len());
            for a in &args {
                let v: i32 = a
                    .parse()
                    .map_err(|_| CompileError::Unsupported(format!("choice non-int '{a}'")))?;
                vals.push(v);
            }
            Ok(Some(vals))
        }
        "prime" => {
            let (lo, hi) = parse_pair(&args, name)?;
            let lo = lo.max(2);
            let primes: Vec<i32> = (lo..=hi)
                .filter(|&n| is_prime(n as u64))
                .map(|v| v as i32)
                .collect();
            if primes.is_empty() {
                return Err(CompileError::Unsupported(format!("no primes in {def}")));
            }
            Ok(Some(primes))
        }
        "decimal" | "rational" | "vector" | "matrix" | "angle" => Err(CompileError::Unsupported(
            format!("M1 doesn't support sampler '{name}'"),
        )),
        _ => Ok(None), // not a sampler; treat as derived
    }
}

fn parse_pair(args: &[&str], name: &str) -> Result<(i64, i64), CompileError> {
    if args.len() != 2 {
        return Err(CompileError::BadSampler(format!("{name} needs 2 args")));
    }
    let lo: i64 = args[0]
        .parse()
        .map_err(|_| CompileError::BadSampler(format!("{name} lo: {}", args[0])))?;
    let hi: i64 = args[1]
        .parse()
        .map_err(|_| CompileError::BadSampler(format!("{name} hi: {}", args[1])))?;
    if lo > hi {
        return Err(CompileError::BadSampler(format!("{name} empty range")));
    }
    Ok((lo, hi))
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: u64 = 5;
    while i.saturating_mul(i) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// ----- expression parser -----

fn compile_expression(
    src: &str,
    var_to_slot: &BTreeMap<String, u32>,
) -> Result<Program, CompileError> {
    let mut p = Parser::new(src);
    let mut prog = Program::new();
    p.parse_or(&mut prog, var_to_slot)?;
    p.skip_ws();
    if !p.at_end() {
        return Err(CompileError::Unsupported(format!(
            "trailing input '{}' in '{src}'",
            &src[p.pos..]
        )));
    }
    prog.finish();
    Ok(prog)
}

fn compile_constraint(
    src: &str,
    var_to_slot: &BTreeMap<String, u32>,
) -> Result<Program, CompileError> {
    let mut p = Parser::new(src);
    let mut prog = Program::new();
    p.parse_or(&mut prog, var_to_slot)?;
    p.skip_ws();
    if !p.at_end() {
        return Err(CompileError::Unsupported(format!(
            "trailing constraint '{}' in '{src}'",
            &src[p.pos..]
        )));
    }
    prog.finish();
    Ok(prog)
}

struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Try to consume a literal string; on match advance and return true.
    fn eat(&mut self, lit: &str) -> bool {
        if self.src[self.pos..].starts_with(lit) {
            self.pos += lit.len();
            true
        } else {
            false
        }
    }

    /// Match a word boundary keyword like `and`/`or`. Requires non-alpha after.
    fn eat_keyword(&mut self, kw: &str) -> bool {
        let s = &self.src[self.pos..];
        if s.starts_with(kw) {
            let next = s.as_bytes().get(kw.len()).copied();
            let boundary = next.is_none_or(|c| !(c.is_ascii_alphanumeric() || c == b'_'));
            if boundary {
                self.pos += kw.len();
                return true;
            }
        }
        false
    }

    fn parse_or(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.parse_and(prog, slots)?;
        loop {
            self.skip_ws();
            if self.eat_keyword("or") {
                self.parse_and(prog, slots)?;
                prog.emit(Op::Or);
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_and(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.parse_compare(prog, slots)?;
        loop {
            self.skip_ws();
            if self.eat_keyword("and") {
                self.parse_compare(prog, slots)?;
                prog.emit(Op::And);
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_compare(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.parse_add(prog, slots)?;
        self.skip_ws();
        // Order: try 2-char ops first
        let op = if self.eat("==") {
            Some(Op::Eq)
        } else if self.eat("!=") {
            Some(Op::Neq)
        } else if self.eat("<=") {
            Some(Op::Le)
        } else if self.eat(">=") {
            Some(Op::Ge)
        } else if self.eat("<") {
            Some(Op::Lt)
        } else if self.eat(">") {
            Some(Op::Gt)
        } else {
            None
        };
        if let Some(op) = op {
            self.parse_add(prog, slots)?;
            prog.emit(op);
        }
        Ok(())
    }

    fn parse_add(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.parse_mul(prog, slots)?;
        loop {
            self.skip_ws();
            if self.eat("+") {
                self.parse_mul(prog, slots)?;
                prog.emit(Op::Add);
            } else if self.peek() == Some(b'-') {
                // distinguish from unary minus that's part of next term;
                // if previous was a value, this is binary.
                self.pos += 1;
                self.parse_mul(prog, slots)?;
                prog.emit(Op::Sub);
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_mul(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.parse_unary(prog, slots)?;
        loop {
            self.skip_ws();
            if self.eat("*") {
                self.parse_unary(prog, slots)?;
                prog.emit(Op::Mul);
            } else if self.eat("/") {
                self.parse_unary(prog, slots)?;
                prog.emit(Op::DivTrunc);
            } else if self.eat("%") {
                self.parse_unary(prog, slots)?;
                prog.emit(Op::ModFloor);
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_unary(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.skip_ws();
        if self.eat("-") {
            self.parse_unary(prog, slots)?;
            prog.emit(Op::Neg);
            return Ok(());
        }
        if self.eat("+") {
            return self.parse_unary(prog, slots);
        }
        self.parse_pow(prog, slots)
    }

    fn parse_pow(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.parse_primary(prog, slots)?;
        self.skip_ws();
        if self.eat("^") {
            // right-assoc: parse another unary
            self.parse_unary(prog, slots)?;
            prog.emit(Op::Pow);
        }
        Ok(())
    }

    fn parse_primary(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        self.skip_ws();
        let c = self.peek().ok_or_else(|| {
            CompileError::Unsupported(format!("unexpected end in '{}'", self.src))
        })?;
        if c == b'(' {
            self.pos += 1;
            self.parse_or(prog, slots)?;
            self.skip_ws();
            if !self.eat(")") {
                return Err(CompileError::Unsupported(format!(
                    "missing ')' in '{}'",
                    self.src
                )));
            }
            return Ok(());
        }
        if c.is_ascii_digit() {
            return self.parse_number(prog);
        }
        if c.is_ascii_alphabetic() || c == b'_' {
            return self.parse_ident(prog, slots);
        }
        Err(CompileError::Unsupported(format!(
            "bad char '{}' in '{}'",
            c as char, self.src
        )))
    }

    fn parse_number(&mut self, prog: &mut Program) -> Result<(), CompileError> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else {
                break;
            }
        }
        // Reject decimals — M1 ints only
        if self.peek() == Some(b'.') {
            return Err(CompileError::Unsupported(format!(
                "float literal in '{}'",
                self.src
            )));
        }
        let s = &self.src[start..self.pos];
        let v: i64 = s
            .parse()
            .map_err(|_| CompileError::Unsupported(format!("bad int '{s}'")))?;
        if !(i32::MIN as i64..=i32::MAX as i64).contains(&v) {
            return Err(CompileError::Unsupported(format!("int out of range '{s}'")));
        }
        prog.emit_const(v as i32);
        Ok(())
    }

    fn parse_ident(
        &mut self,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let name = &self.src[start..self.pos];

        // Boolean literals
        if name == "true" {
            prog.emit_const(1);
            return Ok(());
        }
        if name == "false" {
            prog.emit_const(0);
            return Ok(());
        }
        if name == "not" {
            self.parse_unary(prog, slots)?;
            prog.emit(Op::Not);
            return Ok(());
        }

        // Function call?
        self.skip_ws();
        if self.peek() == Some(b'(') {
            return self.parse_call(name, prog, slots);
        }

        let slot = slots.get(name).ok_or_else(|| {
            CompileError::Unsupported(format!("unknown ident '{name}' in '{}'", self.src))
        })?;
        prog.emit_var(*slot);
        Ok(())
    }

    fn parse_call(
        &mut self,
        name: &str,
        prog: &mut Program,
        slots: &BTreeMap<String, u32>,
    ) -> Result<(), CompileError> {
        // Eat '('
        self.pos += 1;
        let n_args_expected = match name {
            "abs" => 1,
            "min" | "max" => 2,
            _ => {
                return Err(CompileError::Unsupported(format!(
                    "unsupported function '{name}'"
                )));
            }
        };
        let mut args_seen = 0;
        loop {
            self.skip_ws();
            if self.peek() == Some(b')') {
                break;
            }
            if args_seen > 0 {
                if !self.eat(",") {
                    return Err(CompileError::Unsupported(format!(
                        "expected ',' in call to {name}"
                    )));
                }
            }
            self.parse_or(prog, slots)?;
            args_seen += 1;
            if args_seen > n_args_expected {
                return Err(CompileError::Unsupported(format!(
                    "too many args to {name}"
                )));
            }
        }
        if !self.eat(")") {
            return Err(CompileError::Unsupported(format!(
                "missing ')' in {name}(...)"
            )));
        }
        if args_seen != n_args_expected {
            return Err(CompileError::Unsupported(format!(
                "{name} expects {n_args_expected} args, got {args_seen}"
            )));
        }
        match name {
            "abs" => prog.emit(Op::Abs),
            "min" => prog.emit(Op::Min),
            "max" => prog.emit(Op::Max),
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu::bytecode::run;

    fn make_slots(names: &[&str]) -> BTreeMap<String, u32> {
        names
            .iter()
            .enumerate()
            .map(|(i, n)| ((*n).to_string(), i as u32))
            .collect()
    }

    #[test]
    fn arith() {
        let slots = make_slots(&["a", "b"]);
        let prog = compile_expression("a*b + 3", &slots).unwrap();
        assert_eq!(run(&prog, &[2, 5]).unwrap(), 13);
    }

    #[test]
    fn parens_and_neg() {
        let slots = make_slots(&["x0", "r"]);
        let prog = compile_expression("-(x0 + r)", &slots).unwrap();
        assert_eq!(run(&prog, &[1, 4]).unwrap(), -5);
    }

    #[test]
    fn power() {
        let slots = make_slots(&["x"]);
        let prog = compile_expression("x^3 + 2*x", &slots).unwrap();
        assert_eq!(run(&prog, &[3]).unwrap(), 33);
    }

    #[test]
    fn constraint_neq_and_lt() {
        let slots = make_slots(&["a", "b"]);
        let prog = compile_constraint("a != b and a < 10", &slots).unwrap();
        assert_eq!(run(&prog, &[3, 4]).unwrap(), 1);
        assert_eq!(run(&prog, &[5, 5]).unwrap(), 0);
        assert_eq!(run(&prog, &[20, 4]).unwrap(), 0);
    }

    #[test]
    fn samplers_recognized() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 3)".into());
        vars.insert("b".into(), "nonzero(-2, 2)".into());
        vars.insert("c".into(), "a + b".into());
        let plan = compile(&vars, &[]).unwrap();
        assert_eq!(plan.sampler_slots.len(), 2);
        assert_eq!(plan.sampler_slots[0].values, vec![1, 2, 3]);
        assert_eq!(plan.sampler_slots[1].values, vec![-2, -1, 1, 2]);
        // total combos = 3 * 4 = 12
        assert_eq!(plan.total_combos, 12);
    }

    #[test]
    fn unsupported_function() {
        let slots = make_slots(&["a"]);
        let r = compile_expression("derivative(a*x, x)", &slots);
        assert!(matches!(r, Err(CompileError::Unsupported(_))));
    }
}
