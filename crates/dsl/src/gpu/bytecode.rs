//! Bytecode VM for derived-variable and constraint evaluation.
//!
//! Stack machine over `i32`. One pipeline (WGSL) and one Rust interpreter
//! both consume the same encoding, so CPU fallback and GPU execution are
//! semantically identical.
//!
//! Boolean ops produce 0 / 1 i32 values. Constraints are programs that
//! evaluate to a single boolean on top of the stack.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Op {
    Halt = 0,
    LoadConst = 1,
    LoadVar = 2,
    Add = 3,
    Sub = 4,
    Mul = 5,
    DivTrunc = 6,
    ModFloor = 7,
    Neg = 8,
    Abs = 9,
    Pow = 10,
    Eq = 11,
    Neq = 12,
    Lt = 13,
    Le = 14,
    Gt = 15,
    Ge = 16,
    And = 17,
    Or = 18,
    Not = 19,
    Min = 20,
    Max = 21,
}

impl Op {
    pub fn from_u32(x: u32) -> Option<Op> {
        Some(match x {
            0 => Op::Halt,
            1 => Op::LoadConst,
            2 => Op::LoadVar,
            3 => Op::Add,
            4 => Op::Sub,
            5 => Op::Mul,
            6 => Op::DivTrunc,
            7 => Op::ModFloor,
            8 => Op::Neg,
            9 => Op::Abs,
            10 => Op::Pow,
            11 => Op::Eq,
            12 => Op::Neq,
            13 => Op::Lt,
            14 => Op::Le,
            15 => Op::Gt,
            16 => Op::Ge,
            17 => Op::And,
            18 => Op::Or,
            19 => Op::Not,
            20 => Op::Min,
            21 => Op::Max,
            _ => return None,
        })
    }
}

/// Compact bytecode program.
///
/// Layout: alternating `(op, arg)` u32 pairs. `arg` is used for `LoadConst`
/// (constant index) and `LoadVar` (slot index); zero for other ops.
#[derive(Debug, Clone, Default)]
pub struct Program {
    pub code: Vec<u32>,
    pub consts: Vec<i32>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, op: Op) {
        self.code.push(op as u32);
        self.code.push(0);
    }

    pub fn emit_const(&mut self, value: i32) {
        let idx = self
            .consts
            .iter()
            .position(|&v| v == value)
            .unwrap_or_else(|| {
                self.consts.push(value);
                self.consts.len() - 1
            });
        self.code.push(Op::LoadConst as u32);
        self.code.push(idx as u32);
    }

    pub fn emit_var(&mut self, slot: u32) {
        self.code.push(Op::LoadVar as u32);
        self.code.push(slot);
    }

    pub fn finish(&mut self) {
        self.emit(Op::Halt);
    }

    pub fn len(&self) -> usize {
        self.code.len() / 2
    }
}

/// Evaluate a program with given variable slots. Returns top of stack.
///
/// Same semantics as the WGSL VM. Used by CPU fallback and tests.
pub fn run(program: &Program, vars: &[i32]) -> Result<i32, &'static str> {
    let mut stack: [i32; 32] = [0; 32];
    let mut sp: usize = 0;
    let mut ip: usize = 0;

    while ip < program.code.len() {
        let op = Op::from_u32(program.code[ip]).ok_or("bad opcode")?;
        let arg = program.code[ip + 1];
        ip += 2;

        match op {
            Op::Halt => break,
            Op::LoadConst => {
                if sp >= 32 {
                    return Err("stack overflow");
                }
                stack[sp] = *program.consts.get(arg as usize).ok_or("const oob")?;
                sp += 1;
            }
            Op::LoadVar => {
                if sp >= 32 {
                    return Err("stack overflow");
                }
                stack[sp] = *vars.get(arg as usize).ok_or("var oob")?;
                sp += 1;
            }
            _ => {
                // Unary / binary / boolean ops
                if sp == 0 {
                    return Err("stack underflow");
                }
                match op {
                    Op::Neg => stack[sp - 1] = stack[sp - 1].wrapping_neg(),
                    Op::Abs => stack[sp - 1] = stack[sp - 1].wrapping_abs(),
                    Op::Not => stack[sp - 1] = if stack[sp - 1] == 0 { 1 } else { 0 },
                    _ => {
                        if sp < 2 {
                            return Err("stack underflow");
                        }
                        let b = stack[sp - 1];
                        let a = stack[sp - 2];
                        sp -= 1;
                        stack[sp - 1] = match op {
                            Op::Add => a.wrapping_add(b),
                            Op::Sub => a.wrapping_sub(b),
                            Op::Mul => a.wrapping_mul(b),
                            Op::DivTrunc => {
                                if b == 0 {
                                    return Err("division by zero");
                                }
                                a.wrapping_div(b)
                            }
                            Op::ModFloor => {
                                if b == 0 {
                                    return Err("modulo by zero");
                                }
                                a.rem_euclid(b)
                            }
                            Op::Pow => {
                                if b < 0 || b > 16 {
                                    return Err("pow exponent out of range");
                                }
                                let mut acc = 1i32;
                                let mut base = a;
                                let mut e = b as u32;
                                while e > 0 {
                                    if e & 1 == 1 {
                                        acc = acc.wrapping_mul(base);
                                    }
                                    base = base.wrapping_mul(base);
                                    e >>= 1;
                                }
                                acc
                            }
                            Op::Eq => i32::from(a == b),
                            Op::Neq => i32::from(a != b),
                            Op::Lt => i32::from(a < b),
                            Op::Le => i32::from(a <= b),
                            Op::Gt => i32::from(a > b),
                            Op::Ge => i32::from(a >= b),
                            Op::And => i32::from(a != 0 && b != 0),
                            Op::Or => i32::from(a != 0 || b != 0),
                            Op::Min => a.min(b),
                            Op::Max => a.max(b),
                            _ => unreachable!(),
                        };
                    }
                }
            }
        }
    }

    if sp == 0 {
        Err("empty stack at halt")
    } else {
        Ok(stack[sp - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_load() {
        let mut p = Program::new();
        p.emit_const(42);
        p.finish();
        assert_eq!(run(&p, &[]).unwrap(), 42);
    }

    #[test]
    fn add_vars() {
        let mut p = Program::new();
        p.emit_var(0);
        p.emit_var(1);
        p.emit(Op::Add);
        p.finish();
        assert_eq!(run(&p, &[3, 5]).unwrap(), 8);
    }

    #[test]
    fn mod_floor_negative() {
        let mut p = Program::new();
        p.emit_const(-7);
        p.emit_const(3);
        p.emit(Op::ModFloor);
        p.finish();
        assert_eq!(run(&p, &[]).unwrap(), 2);
    }

    #[test]
    fn pow_op() {
        let mut p = Program::new();
        p.emit_var(0);
        p.emit_const(3);
        p.emit(Op::Pow);
        p.finish();
        assert_eq!(run(&p, &[2]).unwrap(), 8);
    }

    #[test]
    fn boolean_chain() {
        // (a > 0) and (b != 0)
        let mut p = Program::new();
        p.emit_var(0);
        p.emit_const(0);
        p.emit(Op::Gt);
        p.emit_var(1);
        p.emit_const(0);
        p.emit(Op::Neq);
        p.emit(Op::And);
        p.finish();
        assert_eq!(run(&p, &[5, 3]).unwrap(), 1);
        assert_eq!(run(&p, &[5, 0]).unwrap(), 0);
        assert_eq!(run(&p, &[-1, 3]).unwrap(), 0);
    }
}
