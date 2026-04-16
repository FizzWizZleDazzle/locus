// GPU compute shader for parallel variable sampling + constraint checking
// Each invocation generates one complete set of random variables and checks constraints

struct SamplerDef {
    kind: u32,     // 0=integer, 1=nonzero, 2=choice, 3=derived_add, 4=derived_mul, 5=derived_sub
    lo: i32,
    hi: i32,
    dep_a: u32,    // index of dependency variable A (for derived)
    dep_b: u32,    // index of dependency variable B (for derived)
    extra: i32,    // extra param (choice count, constant multiplier, etc.)
}

struct ConstraintDef {
    kind: u32,     // 0=neq, 1=lt, 2=gt, 3=gte, 4=lte
    var_a: u32,    // index of left variable
    var_b: u32,    // index of right variable (or 0xFFFFFFFF for constant)
    constant: i32, // constant value for comparison (when var_b = 0xFFFFFFFF)
}

struct Config {
    num_vars: u32,
    num_constraints: u32,
    num_invocations: u32,
    seed: u32,
}

@group(0) @binding(0) var<uniform> config: Config;
@group(0) @binding(1) var<storage, read> samplers: array<SamplerDef>;
@group(0) @binding(2) var<storage, read> constraints: array<ConstraintDef>;
@group(0) @binding(3) var<storage, read> choices: array<i32>; // flattened choice values
@group(0) @binding(4) var<storage, read_write> results: array<i32>; // output: num_vars * num_invocations
@group(0) @binding(5) var<storage, read_write> valid: array<u32>; // 1 if constraints pass, 0 otherwise

// PCG random number generator
fn pcg(state: ptr<function, u32>) -> u32 {
    let old = *state;
    *state = old * 747796405u + 2891336453u;
    let word = ((old >> ((old >> 28u) + 4u)) ^ old) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand_range(state: ptr<function, u32>, lo: i32, hi: i32) -> i32 {
    let range = u32(hi - lo + 1);
    let r = pcg(state) % range;
    return lo + i32(r);
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    if (idx >= config.num_invocations) {
        return;
    }

    // Initialize RNG with unique seed per invocation
    var rng_state = config.seed ^ (idx * 1000003u);
    // Warm up RNG
    _ = pcg(&rng_state);
    _ = pcg(&rng_state);

    let base = idx * config.num_vars;

    // Sample all variables in order
    for (var v = 0u; v < config.num_vars; v++) {
        let s = samplers[v];
        var value: i32 = 0;

        switch (s.kind) {
            // integer(lo, hi)
            case 0u: {
                value = rand_range(&rng_state, s.lo, s.hi);
            }
            // nonzero(lo, hi)
            case 1u: {
                for (var attempt = 0u; attempt < 100u; attempt++) {
                    value = rand_range(&rng_state, s.lo, s.hi);
                    if (value != 0) { break; }
                }
            }
            // choice — index into choices array
            case 2u: {
                let choice_idx = rand_range(&rng_state, 0, s.hi); // hi = count-1
                value = choices[u32(s.lo) + u32(choice_idx)]; // lo = offset into choices
            }
            // derived: a + b
            case 3u: {
                value = results[base + s.dep_a] + results[base + s.dep_b];
            }
            // derived: a * b
            case 4u: {
                value = results[base + s.dep_a] * results[base + s.dep_b];
            }
            // derived: a - b
            case 5u: {
                value = results[base + s.dep_a] - results[base + s.dep_b];
            }
            // derived: a * constant
            case 6u: {
                value = results[base + s.dep_a] * s.extra;
            }
            // derived: copy of another var
            case 7u: {
                value = results[base + s.dep_a];
            }
            default: {
                value = 0;
            }
        }

        results[base + v] = value;
    }

    // Check constraints
    var all_pass = true;
    for (var c = 0u; c < config.num_constraints; c++) {
        let ct = constraints[c];
        let a_val = results[base + ct.var_a];
        var b_val: i32;
        if (ct.var_b == 0xFFFFFFFFu) {
            b_val = ct.constant;
        } else {
            b_val = results[base + ct.var_b];
        }

        switch (ct.kind) {
            case 0u: { all_pass = all_pass && (a_val != b_val); } // neq
            case 1u: { all_pass = all_pass && (a_val < b_val); }  // lt
            case 2u: { all_pass = all_pass && (a_val > b_val); }  // gt
            case 3u: { all_pass = all_pass && (a_val >= b_val); } // gte
            case 4u: { all_pass = all_pass && (a_val <= b_val); } // lte
            default: {}
        }
    }

    valid[idx] = select(0u, 1u, all_pass);
}
