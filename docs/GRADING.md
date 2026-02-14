# Grading System Documentation

## Overview

The Locus grading system uses a sophisticated two-stage equivalence checking algorithm to determine if a user's mathematical answer is correct. The system is designed to handle various forms of mathematical expressions while supporting different grading modes that enforce specific answer formats.

**Key Features:**
- **Symbolic and Numerical Verification**: Combines exact symbolic algebra with numerical testing
- **Multiple Grading Modes**: Supports equivalent, factored, and expanded form checking
- **MathJSON Pipeline**: Structured AST-based input processing via MathLive
- **Cross-Platform**: Identical grading logic runs in both frontend (WASM) and backend (native)
- **SymEngine Integration**: Leverages powerful symbolic computation via FFI

**Architecture:**
```
User Input (MathLive) → MathJSON → SymEngine Expression → Two-Stage Check → Result
```

---

## Two-Stage Equivalence

The core of the grading system is the `are_equivalent()` function, which performs a two-stage check to determine if two expressions are mathematically equivalent.

### Stage 1: Symbolic Verification

The first stage uses exact symbolic algebra to check if two expressions are identical after expansion.

**Algorithm:**
1. Compute the difference: `diff = user_answer - expected_answer`
2. Expand the difference: `expanded = expand(diff)`
3. Check if the result is symbolically zero: `is_zero(expanded)`

**What it handles:**
- All polynomial equivalences (e.g., `(x+1)(x-1)` ≡ `x^2 - 1`)
- Rational expressions (e.g., `2/x + 3/x` ≡ `5/x`)
- Algebraic simplifications
- Exact symbolic identities

**Example:**
```rust
// User: (x+2)^2
// Expected: x^2 + 4*x + 4

let diff = parse("(x+2)^2") - parse("x^2 + 4*x + 4");
let expanded = expand(diff);  // Expands to 0
is_zero(expanded);  // true → Stage 1 passes
```

### Stage 2: Numerical Evaluation (Fallback)

If Stage 1 fails (the expanded difference is not symbolically zero), the system falls back to numerical testing. This is crucial for identities that SymEngine's `expand()` cannot simplify symbolically.

**Algorithm:**
1. Identify all free variables (symbols) in the expression
2. If no variables exist, evaluate the expression and check if `|result| < NUMERICAL_TOLERANCE`
3. If variables exist, substitute each with test values and evaluate
4. For each test point, verify that `|user_answer - expected_answer| < NUMERICAL_TOLERANCE`
5. If all test points pass, the answers are considered equivalent

**Test Points:**
```rust
const TEST_POINTS: &[f64] = &[0.7, 1.3, 2.1, -0.4, 0.31];
const NUMERICAL_TOLERANCE: f64 = 1e-6;
```

These points are specifically chosen to:
- Avoid common edge cases (0, 1, π/2, etc.)
- Include both positive and negative values
- Include non-integer values
- Provide sufficient coverage for most mathematical functions

**What it handles:**
- Trigonometric identities (e.g., `sin(x)^2 + cos(x)^2` ≡ `1`)
- Logarithmic equivalences (e.g., `log(x) + log(y)` ≡ `log(x*y)`)
- Transcendental functions (exp, ln, trig functions)
- Complex algebraic identities that don't expand to zero symbolically

**Example:**
```rust
// User: sin(x)^2 + cos(x)^2
// Expected: 1

// Stage 1: expand(sin(x)^2 + cos(x)^2 - 1) ≠ 0 (SymEngine can't simplify this)
// Stage 2: Numerical evaluation
for test_val in [0.7, 1.3, 2.1, -0.4, 0.31] {
    let user_val = sin(test_val)^2 + cos(test_val)^2;     // ≈ 1.0
    let expected_val = 1.0;
    assert!(abs(user_val - expected_val) < 1e-6);  // All pass → Correct!
}
```

**Complete Implementation:**
```rust
fn are_equivalent<E: ExprEngine>(a: &E, b: &E) -> bool {
    // Stage 1: exact symbolic check
    let diff = a.sub(b);
    let expanded = diff.expand();
    if expanded.is_zero() {
        return true;
    }

    // Stage 2: numerical evaluation fallback
    let symbols = expanded.free_symbols();
    if symbols.is_empty() {
        // No free symbols but not symbolically zero — try evaluating
        if let Some(val) = expanded.to_float() {
            return val.abs() < NUMERICAL_TOLERANCE;
        }
        return false;
    }

    // Substitute each variable with test values and check
    for &test_val in TEST_POINTS {
        let mut subst = a.sub(b);
        for sym in &symbols {
            subst = subst.subs_float(sym, test_val);
        }
        match subst.to_float() {
            Some(val) if val.abs() < NUMERICAL_TOLERANCE => continue,
            Some(_) => return false,    // Non-zero at this point → not equivalent
            None => return false,       // Can't evaluate → can't confirm
        }
    }

    // All test points passed — expressions are equivalent (high confidence)
    true
}
```

---

## Grading Modes

The grading system supports three distinct modes that enforce different answer format requirements while checking mathematical correctness.

### 1. Equivalent Mode

**Purpose**: Accept any mathematically equivalent form of the answer.

**Check**: Only verifies mathematical equivalence (two-stage check).

**Use Cases**:
- General problem solving
- Problems where form doesn't matter
- Multiple valid solution formats

**Examples:**

| Expected Answer | User Answer | Result | Reason |
|----------------|-------------|---------|---------|
| `x^2 - 1` | `(x+1)(x-1)` | Correct | Equivalent |
| `x^2 - 1` | `x^2 - 1` | Correct | Identical |
| `2*x + 4` | `2(x + 2)` | Correct | Equivalent |
| `sin(x)^2 + cos(x)^2` | `1` | Correct | Trig identity |
| `x^2` | `x^3` | Incorrect | Not equivalent |

**Implementation:**
```rust
GradingMode::Equivalent => GradeResult::Correct,
```

### 2. Factor Mode

**Purpose**: Require the answer to be in factored form (not expanded).

**Checks**:
1. Mathematical equivalence (two-stage check)
2. Answer must NOT be in expanded form: `expand(user_answer) ≠ user_answer`

**Use Cases**:
- Factoring problems
- Requiring simplified factored expressions
- Difference of squares, sum/difference of cubes, etc.

**How it works**:
```rust
GradingMode::Factor => {
    // First check: Is the answer mathematically correct?
    if !are_equivalent(&user_expr, &answer_expr) {
        return GradeResult::Incorrect;
    }

    // Second check: Is it in factored form?
    // If expand(user) == user, they submitted an expanded form
    if user_expr.expand().equals(&user_expr) {
        GradeResult::Incorrect  // Already expanded → reject
    } else {
        GradeResult::Correct    // Not expanded → factored form
    }
}
```

**Examples:**

| Expected Answer | User Answer | Result | Reason |
|----------------|-------------|---------|---------|
| `(x+1)(x-1)` | `(x+1)(x-1)` | Correct | Factored form |
| `(x+1)(x-1)` | `(x-1)(x+1)` | Correct | Equivalent factored form |
| `(x+1)(x-1)` | `x^2 - 1` | Incorrect | Expanded form rejected |
| `(x+2)(x+3)` | `x^2 + 5*x + 6` | Incorrect | Expanded form rejected |
| `(x+1)^2` | `(x+1)(x+1)` | Correct | Both are factored |
| `(x+1)(x-1)` | `x^2 + 2*x` | Incorrect | Not equivalent |

**Edge Cases:**
- `x` is considered "expanded" (expand(x) == x), so Factor mode works correctly for linear expressions
- Constants like `5` are expanded, but rarely used in Factor mode problems
- `(x+1)^2` is NOT considered expanded (expands to `x^2 + 2*x + 1`)

### 3. Expand Mode

**Purpose**: Require the answer to be in fully expanded form.

**Checks**:
1. Mathematical equivalence (two-stage check)
2. Answer MUST be in expanded form: `expand(user_answer) == user_answer`

**Use Cases**:
- Polynomial expansion exercises
- "Simplify and expand" problems
- Requiring standard form

**How it works**:
```rust
GradingMode::Expand => {
    // First check: Is the answer mathematically correct?
    if !are_equivalent(&user_expr, &answer_expr) {
        return GradeResult::Incorrect;
    }

    // Second check: Is it in expanded form?
    // If expand(user) != user, they submitted an unexpanded form
    if user_expr.expand().equals(&user_expr) {
        GradeResult::Correct    // Already expanded → correct
    } else {
        GradeResult::Incorrect  // Not expanded → reject
    }
}
```

**Examples:**

| Expected Answer | User Answer | Result | Reason |
|----------------|-------------|---------|---------|
| `x^2 + 2*x + 1` | `x^2 + 2*x + 1` | Correct | Expanded form |
| `x^2 + 2*x + 1` | `2*x + x^2 + 1` | Correct | Equivalent expanded |
| `x^2 + 2*x + 1` | `(x+1)^2` | Incorrect | Factored form rejected |
| `x^2 + 2*x + 1` | `(x+1)(x+1)` | Incorrect | Factored form rejected |
| `x^2 - 1` | `(x+1)(x-1)` | Incorrect | Factored form rejected |
| `x^2 + 2*x + 1` | `x^2 + 3*x` | Incorrect | Not equivalent |

**Edge Cases:**
- `x + 2` is in expanded form (expand(x+2) == x+2)
- Constants and variables alone are expanded
- Trig functions: `sin(x)` is considered expanded

---

## MathJSON Pipeline

### Overview

The system uses MathLive's MathJSON format as the primary input method, providing a structured AST representation of mathematical expressions instead of raw LaTeX parsing.

**Flow:**
```
User types in MathLive → MathJSON AST → SymEngine-compatible string → SymEngine parse → Expression
```

### MathJSON Format

MathJSON represents mathematical expressions as JSON trees with typed nodes:

**Node Types:**

1. **Number**: `{"num": "42"}`
2. **Symbol**: `{"sym": "x"}`
3. **String** (constants): `{"str": "Pi"}` → `pi`
4. **Function**: `{"fn": "FunctionName", "args": [...]}`

### Conversion Examples

**Simple Expression:**
```json
// x + 2
{
  "fn": "Add",
  "args": [
    {"sym": "x"},
    {"num": "2"}
  ]
}
```
→ Converts to: `(x+2)`

**Nested Expression:**
```json
// (x+2)(x+3)
{
  "fn": "Multiply",
  "args": [
    {"fn": "Add", "args": [{"sym": "x"}, {"num": "2"}]},
    {"fn": "Add", "args": [{"sym": "x"}, {"num": "3"}]}
  ]
}
```
→ Converts to: `((x+2)*(x+3))`

**Power/Exponent:**
```json
// x^2
{
  "fn": "Power",
  "args": [
    {"sym": "x"},
    {"num": "2"}
  ]
}
```
→ Converts to: `(x)^(2)`

**Trigonometric:**
```json
// sin(x)^2 + cos(x)^2
{
  "fn": "Add",
  "args": [
    {"fn": "Power", "args": [{"fn": "Sin", "args": [{"sym": "x"}]}, {"num": "2"}]},
    {"fn": "Power", "args": [{"fn": "Cos", "args": [{"sym": "x"}]}, {"num": "2"}]}
  ]
}
```
→ Converts to: `((sin(x))^(2)+(cos(x))^(2))`

### Supported Functions

**Arithmetic:**
- `Add` → `+` (supports chaining: x+y+z)
- `Subtract` → `-` (binary) or `-(expr)` (unary)
- `Negate` → `-(expr)`
- `Multiply` → `*` (supports chaining)
- `Divide` → `/`

**Powers and Roots:**
- `Power` → `^` (special case: exponent 0.5 or 1/2 → `sqrt`)
- `Sqrt` → `sqrt()`
- `Root` → `x^(1/n)`

**Exponential and Logarithmic:**
- `Exp` → `exp()`
- `Ln` → `log()` (natural log in SymEngine)
- `Log` → `log(x, base)` (default base 10 if not specified)

**Trigonometric:**
- `Sin` → `sin()`
- `Cos` → `cos()`
- `Tan` → `tan()`
- `Arcsin` → `asin()`
- `Arccos` → `acos()`
- `Arctan` → `atan()`
- `Sec` → `sec()`
- `Csc` → `csc()`
- `Cot` → `cot()`

**Other:**
- `Abs` → `abs()`

**Constants:**
- `{"str": "Pi"}` → `pi`
- `{"str": "E"}` or `{"str": "ExponentialE"}` → `E`

### Implementation

The conversion is handled by `/home/artur/Locus/crates/common/src/mathjson.rs`:

```rust
pub fn convert_mathjson_to_plain(json_str: &str) -> Result<String, String> {
    let json: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    node_to_symengine(&json)
}
```

**Key Design Decisions:**
- Extra parentheses ensure correct precedence: `((x+2)*(x+3))`
- Square root optimization: `x^0.5` and `x^(1/2)` → `sqrt(x)`
- Natural log: MathJSON `Ln` → SymEngine `log()` (base e)
- Unary minus: `{"fn": "Subtract", "args": [x]}` → `-(x)`

---

## Edge Cases and Special Handling

### Trigonometric Identities

**Challenge**: SymEngine's `expand()` cannot simplify most trig identities symbolically.

**Solution**: Stage 2 numerical evaluation catches these cases.

**Examples:**

| User Answer | Expected | Stage 1 | Stage 2 | Result |
|------------|----------|---------|---------|--------|
| `sin(x)^2 + cos(x)^2` | `1` | Not zero | All test points pass | Correct |
| `tan(x)` | `sin(x)/cos(x)` | Not zero | All test points pass | Correct |
| `sin(2*x)` | `2*sin(x)*cos(x)` | Not zero | All test points pass | Correct |
| `cos(2*x)` | `cos(x)^2 - sin(x)^2` | Not zero | All test points pass | Correct |

### Logarithmic Equivalences

**Examples:**

| User Answer | Expected | How It's Checked |
|------------|----------|------------------|
| `log(x) + log(y)` | `log(x*y)` | Stage 2 (numerical) |
| `log(x^2)` | `2*log(x)` | Stage 2 (numerical) |
| `log(x/y)` | `log(x) - log(y)` | Stage 2 (numerical) |

**Note**: SymEngine's `expand()` may simplify some log operations, but numerical fallback ensures all cases work.

### Special Values

**Constants**:
- `pi` (π)
- `E` (Euler's number)
- Numeric constants: `1`, `2.5`, etc.

**Zero Expressions**:
```rust
// User: 0
// Expected: x - x
// Stage 1: expand(0 - (x-x)) = expand(0) = 0 YES
```

**Constant Expressions (No Variables)**:
```rust
// User: 2 + 3
// Expected: 5
// Stage 1: expand((2+3) - 5) = expand(0) = 0 YES
```

### Division and Fractions

**MathJSON Division**:
```json
// x/2
{"fn": "Divide", "args": [{"sym": "x"}, {"num": "2"}]}
```
→ `(x)/(2)`

**SymEngine Handling**:
- `x/2 + x/3` expands to `5*x/6` (rational simplification)
- `(x+1)/(x-1)` remains as division (not expanded)

### Negative Numbers and Subtraction

**Unary Minus**:
```json
{"fn": "Subtract", "args": [{"sym": "x"}]}
```
→ `-(x)`

**Binary Subtraction**:
```json
{"fn": "Subtract", "args": [{"sym": "x"}, {"num": "2"}]}
```
→ `(x-2)`

**Negate Function**:
```json
{"fn": "Negate", "args": [{"sym": "x"}]}
```
→ `-(x)`

---

## Test Points Selection

### The Test Points

```rust
const TEST_POINTS: &[f64] = &[0.7, 1.3, 2.1, -0.4, 0.31];
```

### Why These Values?

**Avoiding Edge Cases:**
- **NOT 0**: Many expressions are undefined or special at x=0 (e.g., `1/x`, `log(x)`)
- **NOT 1**: Identities like `x^n` collapse to 1 when x=1
- **NOT π/2 ≈ 1.57**: Trig functions have special values (sin(π/2)=1, cos(π/2)=0)
- **NOT integers**: Polynomial differences might cancel at integer points

**Coverage:**
- **Positive values**: 0.7, 1.3, 2.1, 0.31 (different scales)
- **Negative value**: -0.4 (tests sign handling)
- **Non-integer values**: All are decimals (better for general expressions)
- **Different magnitudes**: 0.31 (small), 2.1 (larger), spread across [−0.5, 2.5]

### How Many Test Points?

**Five points** provides a balance:
- **Too few** (1-2): May miss non-equivalent expressions that coincidentally match
- **Just right** (5): High confidence for most mathematical functions
- **Too many** (10+): Diminishing returns, slower evaluation

**Statistical Confidence:**
For two random non-equivalent expressions to match at all 5 test points is extremely unlikely (probability ≈ 0 for continuous functions).

### Limitations

**Pathological Cases:**
Carefully constructed adversarial expressions could theoretically match at these exact 5 points while being non-equivalent. However:
- This is astronomically unlikely for genuine student answers
- Would require knowledge of the exact test points
- Not a practical concern for educational use

**Future Improvements:**
- Randomized test points per problem (prevents gaming)
- Adaptive number of test points based on expression complexity
- Interval arithmetic for rigorous verification

---

## Numerical Tolerance

### The Constant

```rust
const NUMERICAL_TOLERANCE: f64 = 1e-6;
```

**Value**: 0.000001 (one millionth)

### Why 1e-6?

**Precision Balance:**
- **Too strict** (e.g., 1e-12): Floating-point rounding errors cause false negatives
- **Just right** (1e-6): Distinguishes computational noise from genuine differences
- **Too loose** (e.g., 1e-3): May accept incorrect answers

**Floating-Point Context:**
- IEEE 754 double precision: ~15-17 decimal digits of precision
- Trigonometric functions: 1e-15 typical error
- Multiple operations compound errors: 1e-6 provides safety margin

### How It's Used

**Constant Expressions (No Variables):**
```rust
if symbols.is_empty() {
    if let Some(val) = expanded.to_float() {
        return val.abs() < NUMERICAL_TOLERANCE;  // |val| < 1e-6
    }
}
```

**Symbolic Expressions (With Variables):**
```rust
for &test_val in TEST_POINTS {
    let mut subst = a.sub(b);
    for sym in &symbols {
        subst = subst.subs_float(sym, test_val);
    }
    match subst.to_float() {
        Some(val) if val.abs() < NUMERICAL_TOLERANCE => continue,  // |val| < 1e-6
        Some(_) => return false,    // Difference exceeds tolerance
        None => return false,
    }
}
```

### Examples

**Accepted (Within Tolerance):**
- `|0.0000001| < 1e-6` YES
- `|0.0000009| < 1e-6` YES
- `|sin(x)^2 + cos(x)^2 - 1| ≈ 1e-16` YES (floating-point noise)

**Rejected (Exceeds Tolerance):**
- `|0.00001| > 1e-6` NO
- `|0.1| > 1e-6` NO
- `|x^2 - x^3| = |0.7^2 - 0.7^3|` = 0.147 NO (at x=0.7)

---

## Common Patterns

### Polynomial Expansion

**Problem**: Expand `(x+1)(x-1)`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `x^2 - 1` | Expand | Correct | Expanded form |
| `(x+1)(x-1)` | Expand | Incorrect | Factored form |
| `x^2 - 1` | Equivalent | Correct | Mathematically correct |
| `(x+1)(x-1)` | Equivalent | Correct | Mathematically correct |

### Polynomial Factoring

**Problem**: Factor `x^2 - 4`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `(x+2)(x-2)` | Factor | Correct | Factored form |
| `(x-2)(x+2)` | Factor | Correct | Equivalent factored form |
| `x^2 - 4` | Factor | Incorrect | Expanded form |
| `(x+2)(x-2)` | Equivalent | Correct | Mathematically correct |
| `x^2 - 4` | Equivalent | Correct | Mathematically correct |

### Simplification

**Problem**: Simplify `2x + 3x`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `5*x` | Equivalent | Correct | Simplified |
| `5x` | Equivalent | Correct | Same (implicit multiplication) |
| `x*5` | Equivalent | Correct | Commutative |
| `3x + 2x` | Equivalent | Correct | Equivalent unsimplified |
| `6x` | Equivalent | Incorrect | Wrong value |

### Trigonometric Identities

**Problem**: Simplify `sin(x)^2 + cos(x)^2`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `1` | Equivalent | Correct | Trig identity (Stage 2) |
| `cos(x)^2 + sin(x)^2` | Equivalent | Correct | Same (commutative) |
| `1 + 0*x` | Equivalent | Correct | Equivalent to 1 |
| `0` | Equivalent | Incorrect | Wrong value |

**Problem**: Simplify `tan(x)`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `sin(x)/cos(x)` | Equivalent | Correct | Definition (Stage 2) |
| `tan(x)` | Equivalent | Correct | Identical |
| `1/cot(x)` | Equivalent | Correct | Reciprocal identity (Stage 2) |

### Logarithms

**Problem**: Simplify `log(x) + log(y)`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `log(x*y)` | Equivalent | Correct | Log property (Stage 2) |
| `log(y) + log(x)` | Equivalent | Correct | Commutative |
| `log(x + y)` | Equivalent | Incorrect | Wrong property |

### Fractions

**Problem**: Simplify `1/2 + 1/3`

| User Answer | Mode | Result | Reason |
|------------|------|--------|--------|
| `5/6` | Equivalent | Correct | Common denominator |
| `0.833333` | Equivalent | Correct | Decimal (within tolerance) |
| `3/6 + 2/6` | Equivalent | Correct | Equivalent unsimplified |
| `1/5` | Equivalent | Incorrect | Wrong value |

---

## Troubleshooting

### Why Was My Correct Answer Rejected?

#### 1. Wrong Grading Mode

**Symptom**: Answer is mathematically correct but marked wrong.

**Cause**: You submitted the correct value but in the wrong format.

**Examples:**
- **Factor mode**: You submitted `x^2 - 1` instead of `(x+1)(x-1)`
- **Expand mode**: You submitted `(x+1)^2` instead of `x^2 + 2*x + 1`

**Solution**: Check the problem's grading mode requirement and format your answer accordingly.

#### 2. Parsing Errors

**Symptom**: "Could not parse your answer" error.

**Cause**: Invalid mathematical syntax in your input.

**Common Mistakes:**
- Missing operators: `2x` should be `2*x` (MathLive usually handles this)
- Unmatched parentheses: `(x+1` missing closing `)`
- Invalid function names: `sine(x)` instead of `sin(x)`
- Mixed notation: Combining incompatible symbols

**Solution**: Use MathLive's built-in editor, which prevents most syntax errors.

#### 3. Floating-Point Precision

**Symptom**: Decimal answers are marked incorrect despite looking correct.

**Cause**: Your decimal approximation differs from the exact answer by more than `1e-6`.

**Example:**
- Expected: `1/3`
- You entered: `0.33` (exact value: 0.333333...)
- Difference: `|0.33 - 0.333333| = 0.003333` > `1e-6` NO

**Solution:**
- Enter exact fractions when possible: `1/3` instead of `0.33`
- Use more decimal places: `0.333333` instead of `0.33`
- Let SymEngine handle the computation: `1/3` is exact

#### 4. Symbolic vs. Numerical Limitations

**Symptom**: Complex trig/log identities are rejected despite being correct.

**Cause**: The test points failed to verify equivalence (rare).

**Example** (hypothetical edge case):
```
Expected: arcsin(x) + arccos(x)
User: pi/2
```
While mathematically correct, this specific identity might not be in SymEngine's simplification rules.

**Solution**: This is rare but possible. Contact instructor if you believe your answer is correct.

#### 5. Expression Form Ambiguity

**Symptom**: In Factor mode, your factored answer is rejected.

**Cause**: SymEngine considers your answer "already expanded."

**Example:**
- You entered: `x` (for `x^2 - x = x(x-1)`)
- SymEngine: `expand(x) == x`, so it's "expanded"
- Factor mode rejects it

**Solution**: Include the full factorization: `x(x-1)` instead of just `x`.

---

## Implementation Reference

### Core Functions

**Main grading function** (`/home/artur/Locus/crates/common/src/grader.rs`):
```rust
pub fn check_answer<E: ExprEngine>(
    user_input: &str,
    answer_key: &str,
    mode: GradingMode
) -> GradeResult {
    let user_expr = match E::parse(user_input) {
        Ok(e) => e,
        Err(_) => return GradeResult::Invalid("Could not parse your answer".into()),
    };

    let answer_expr = match E::parse(answer_key) {
        Ok(e) => e,
        Err(_) => return GradeResult::Error("Invalid answer key".into()),
    };

    if !are_equivalent(&user_expr, &answer_expr) {
        return GradeResult::Incorrect;
    }

    match mode {
        GradingMode::Equivalent => GradeResult::Correct,
        GradingMode::Factor => {
            if user_expr.expand().equals(&user_expr) {
                GradeResult::Incorrect  // Already expanded
            } else {
                GradeResult::Correct
            }
        }
        GradingMode::Expand => {
            if user_expr.expand().equals(&user_expr) {
                GradeResult::Correct    // Already expanded
            } else {
                GradeResult::Incorrect  // Not expanded
            }
        }
    }
}
```

**Convenience wrapper**:
```rust
pub fn check_answer_expr(
    user_input: &str,
    answer_key: &str,
    mode: GradingMode
) -> GradeResult {
    check_answer::<Expr>(user_input, answer_key, mode)
}
```

### ExprEngine Trait

Abstraction for symbolic operations (implemented by SymEngine `Expr`):

```rust
pub trait ExprEngine: Sized {
    type Error: std::fmt::Display;

    fn parse(input: &str) -> Result<Self, Self::Error>;
    fn expand(&self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn equals(&self, other: &Self) -> bool;
    fn is_zero(&self) -> bool;
    fn free_symbols(&self) -> Vec<String>;
    fn subs_float(&self, var_name: &str, val: f64) -> Self;
    fn to_float(&self) -> Option<f64>;
}
```

### GradeResult Enum

```rust
pub enum GradeResult {
    Correct,
    Incorrect,
    Invalid(String),  // Parse error
    Error(String),    // System error (bad answer key)
}
```

---

## Summary

The Locus grading system provides:

1. **Two-Stage Equivalence**: Symbolic algebra (Stage 1) + numerical testing (Stage 2)
2. **Three Grading Modes**: Equivalent, Factor, Expand
3. **Robust MathJSON Pipeline**: Structured AST input from MathLive
4. **High Accuracy**: 5 carefully chosen test points with 1e-6 tolerance
5. **Cross-Platform**: Identical logic in WASM (frontend) and native (backend)

**Key Strengths:**
- Handles polynomial, trig, log, and transcendental functions
- Enforces answer format requirements (Factor/Expand modes)
- Minimizes false negatives through numerical fallback
- Structured input reduces parsing errors

**Known Limitations:**
- Numerical stage can theoretically accept non-equivalent expressions (extremely rare)
- Some exotic identities may not be recognized
- Requires proper MathJSON formatting from MathLive

**Files:**
- `/home/artur/Locus/crates/common/src/grader.rs` - Core grading logic
- `/home/artur/Locus/crates/common/src/mathjson.rs` - MathJSON converter
- `/home/artur/Locus/crates/common/src/symengine.rs` - SymEngine FFI bindings
- `/home/artur/Locus/crates/common/src/lib.rs` - GradingMode enum definition
