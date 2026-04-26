// Bytecode VM for combinatorial enumeration.
// One pipeline reused across all YAMLs; per-YAML data lives in storage buffers.
//
// Layout per dispatch:
//   - Each thread = one Cartesian-product index (mixed-radix decoded from sampler cardinalities).
//   - For each EvalStep: either copy a sampled value, or run a bytecode program against the slot array.
//   - For each constraint program: AND-reduce into `valid_flag`.
//   - Emit row + valid bit + state hash. CPU dedupes survivors (cross-tile path).
//
// Stack: 32 i32 slots per thread (function-scope). Slot array also 32 entries.
// Bytecode opcodes — see `bytecode.rs`. Keep numbering identical.

struct Cfg {
    n_samplers: u32,
    n_steps: u32,
    n_constraints: u32,
    n_vars: u32,
    n_dedup_slots: u32,
    total_combos: u32,
    _pad0: u32,
    _pad1: u32,
}

struct Step {
    kind: u32,         // 0 = sampler, 1 = derived (bytecode program)
    var_slot: u32,
    sampler_idx: u32,  // for kind==0
    program_off: u32,  // for kind==1: index into `code` (u32-pair offset, i.e. word offset / 2)
    program_pairs: u32,
    consts_off: u32,
    consts_count: u32,
    _pad0: u32,
}

struct ConstraintRange {
    program_off: u32,
    program_pairs: u32,
    consts_off: u32,
    consts_count: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> radixes: array<u32>;
@group(0) @binding(2) var<storage, read> sampler_offsets: array<u32>;
@group(0) @binding(3) var<storage, read> sampler_values: array<i32>;
@group(0) @binding(4) var<storage, read> steps: array<Step>;
@group(0) @binding(5) var<storage, read> code: array<u32>;
@group(0) @binding(6) var<storage, read> consts_buf: array<i32>;
@group(0) @binding(7) var<storage, read> constraint_ranges: array<ConstraintRange>;
@group(0) @binding(8) var<storage, read> dedup_slot_idx: array<u32>;
@group(0) @binding(9) var<storage, read_write> out_rows: array<i32>;
@group(0) @binding(10) var<storage, read_write> out_valid: array<u32>;
@group(0) @binding(11) var<storage, read_write> out_hash: array<u32>;

// Opcodes — must match Rust `Op` discriminants.
const OP_HALT: u32 = 0u;
const OP_LOAD_CONST: u32 = 1u;
const OP_LOAD_VAR: u32 = 2u;
const OP_ADD: u32 = 3u;
const OP_SUB: u32 = 4u;
const OP_MUL: u32 = 5u;
const OP_DIV_TRUNC: u32 = 6u;
const OP_MOD_FLOOR: u32 = 7u;
const OP_NEG: u32 = 8u;
const OP_ABS: u32 = 9u;
const OP_POW: u32 = 10u;
const OP_EQ: u32 = 11u;
const OP_NEQ: u32 = 12u;
const OP_LT: u32 = 13u;
const OP_LE: u32 = 14u;
const OP_GT: u32 = 15u;
const OP_GE: u32 = 16u;
const OP_AND: u32 = 17u;
const OP_OR: u32 = 18u;
const OP_NOT: u32 = 19u;
const OP_MIN: u32 = 20u;
const OP_MAX: u32 = 21u;

// Floor-mod matching Rust `rem_euclid`. WGSL `%` truncates toward zero.
fn mod_floor(a: i32, b: i32) -> i32 {
    let r = a % b;
    var out = r;
    if (r < 0 && b > 0) || (r > 0 && b < 0) {
        out = r + b;
    }
    return out;
}

fn ipow(base: i32, exp_in: i32) -> i32 {
    var acc: i32 = 1;
    var b: i32 = base;
    var e: u32 = u32(max(exp_in, 0));
    loop {
        if e == 0u { break; }
        if (e & 1u) == 1u {
            acc = acc * b;
        }
        b = b * b;
        e = e >> 1u;
    }
    return acc;
}

// Run bytecode program. Returns top-of-stack value (or 0 on fail).
fn run(prog_off: u32, prog_pairs: u32, c_off: u32, slots: ptr<function, array<i32, 32>>) -> i32 {
    var stack: array<i32, 32>;
    var sp: i32 = 0;
    var ip: u32 = 0u;

    loop {
        if ip >= prog_pairs { break; }
        let off = prog_off + ip * 2u;
        let op = code[off];
        let arg = code[off + 1u];
        ip = ip + 1u;

        if op == OP_HALT {
            break;
        } else if op == OP_LOAD_CONST {
            stack[sp] = consts_buf[c_off + arg];
            sp = sp + 1;
        } else if op == OP_LOAD_VAR {
            stack[sp] = (*slots)[arg];
            sp = sp + 1;
        } else if op == OP_NEG {
            stack[sp - 1] = -stack[sp - 1];
        } else if op == OP_ABS {
            stack[sp - 1] = abs(stack[sp - 1]);
        } else if op == OP_NOT {
            stack[sp - 1] = select(0, 1, stack[sp - 1] == 0);
        } else {
            let b = stack[sp - 1];
            let a = stack[sp - 2];
            sp = sp - 1;
            var v: i32 = 0;
            if op == OP_ADD { v = a + b; }
            else if op == OP_SUB { v = a - b; }
            else if op == OP_MUL { v = a * b; }
            else if op == OP_DIV_TRUNC {
                if b == 0 { v = 0; } else { v = a / b; }
            }
            else if op == OP_MOD_FLOOR {
                if b == 0 { v = 0; } else { v = mod_floor(a, b); }
            }
            else if op == OP_POW { v = ipow(a, b); }
            else if op == OP_EQ { v = select(0, 1, a == b); }
            else if op == OP_NEQ { v = select(0, 1, a != b); }
            else if op == OP_LT { v = select(0, 1, a < b); }
            else if op == OP_LE { v = select(0, 1, a <= b); }
            else if op == OP_GT { v = select(0, 1, a > b); }
            else if op == OP_GE { v = select(0, 1, a >= b); }
            else if op == OP_AND { v = select(0, 1, a != 0 && b != 0); }
            else if op == OP_OR { v = select(0, 1, a != 0 || b != 0); }
            else if op == OP_MIN { v = min(a, b); }
            else if op == OP_MAX { v = max(a, b); }
            stack[sp - 1] = v;
        }
    }

    if sp <= 0 { return 0; }
    return stack[sp - 1];
}

// xxh3-ish: cheap 32-bit mixing on a single u32 stream. Good enough for dedup
// keying within one dispatch (collision-free for tens of thousands of survivors).
fn mix32(seed: u32, x: u32) -> u32 {
    var h = seed ^ x;
    h = h ^ (h >> 16u);
    h = h * 0x85ebca6bu;
    h = h ^ (h >> 13u);
    h = h * 0xc2b2ae35u;
    h = h ^ (h >> 16u);
    return h;
}

@compute @workgroup_size(256)
fn enumerate(@builtin(global_invocation_id) gid: vec3<u32>) {
    let tid = gid.x;
    if tid >= cfg.total_combos { return; }

    // Mixed-radix decode
    var sampler_idx: array<u32, 16>;
    var t = tid;
    for (var i: u32 = 0u; i < cfg.n_samplers; i = i + 1u) {
        sampler_idx[i] = t % radixes[i];
        t = t / radixes[i];
    }

    // Slot array (function scope)
    var slots: array<i32, 32>;
    for (var i: i32 = 0; i < 32; i = i + 1) { slots[i] = 0; }

    // Evaluate steps in order
    for (var i: u32 = 0u; i < cfg.n_steps; i = i + 1u) {
        let s = steps[i];
        if s.kind == 0u {
            let off = sampler_offsets[s.sampler_idx];
            slots[s.var_slot] = sampler_values[off + sampler_idx[s.sampler_idx]];
        } else {
            slots[s.var_slot] = run(s.program_off, s.program_pairs, s.consts_off, &slots);
        }
    }

    // Constraints
    var ok: u32 = 1u;
    for (var i: u32 = 0u; i < cfg.n_constraints; i = i + 1u) {
        let r = constraint_ranges[i];
        let v = run(r.program_off, r.program_pairs, r.consts_off, &slots);
        if v == 0 { ok = 0u; }
    }

    // Hash dedup-slots into a 32-bit key
    var h: u32 = 0x9e3779b9u;
    for (var i: u32 = 0u; i < cfg.n_dedup_slots; i = i + 1u) {
        let slot = dedup_slot_idx[i];
        let v = u32(slots[slot]);
        h = mix32(h, v);
    }

    let base = tid * cfg.n_vars;
    for (var i: u32 = 0u; i < cfg.n_vars; i = i + 1u) {
        out_rows[base + i] = slots[i];
    }
    out_valid[tid] = ok;
    out_hash[tid] = h;
}
