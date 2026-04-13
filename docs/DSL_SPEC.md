# LocusDSL Specification v1.0

Problem generation language for Locus. AI describes problems in structured YAML. Rust parser handles all computation, LaTeX formatting, validation, and grading.

**Core principle:** AI never writes code, LaTeX, or formatted output. AI describes mathematical relationships. Parser does everything else.

---

## 1. File Structure

Each `.yaml` file contains one or more problem definitions:

```yaml
# derivative-power-rule.yaml

topic: calculus/derivative_rules
difficulty: medium
calculator: none

variables:
  a: nonzero(-12, 12)
  n: integer(2, 7)
  f: a * x^n
  answer: derivative(f, x)

question: Find {derivative_of(f, x)}

answer: answer
mode: equivalent

solution:
  - Apply the power rule to {f}
  - {derivative_of(f, x)} = {answer}
```

---

## 2. Top-Level Fields

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `topic` | yes | `main/sub` | Closed enum, validated against known list |
| `difficulty` | yes | label or range | `very_easy`, `easy`, `medium`, `hard`, `very_hard`, `competition` or `1000-1400` |
| `calculator` | no | enum | `none` (default), `scientific`, `graphing` |
| `time` | no | integer | Expected solve time in seconds. Auto-derived from difficulty if omitted |
| `variables` | yes | map | Variable definitions and computations |
| `constraints` | no | list | Boolean conditions that must hold |
| `question` | yes | string | Problem text with `{var}` and `{display()}` refs |
| `answer` | yes | variable name | Points to the answer variable |
| `answer_type` | no | enum | Auto-inferred from answer value if omitted |
| `mode` | no | enum | `equivalent` (default), `factor`, `expand` |
| `solution` | no | list of strings | Step-by-step solution using `{var}` refs |
| `diagram` | no | map | Declarative diagram specification |
| `variants` | no | list | Multiple problem structures, parser picks randomly |

---

## 3. Difficulty Labels

| Label | ELO Range | Typical audience |
|-------|-----------|-----------------|
| `very_easy` | 800-1000 | First exposure |
| `easy` | 1000-1200 | Practice |
| `medium` | 1200-1400 | Proficiency |
| `hard` | 1400-1600 | Mastery |
| `very_hard` | 1600-1800 | Challenge |
| `competition` | 1800-2200 | Competition prep |

Parser picks uniformly within range.

---

## 4. Variables

Variables are the core of the DSL. Three kinds:

### 4.1 Sampled Variables (random generation)

```yaml
variables:
  a: integer(2, 10)          # random integer in [2, 10]
  b: integer(-5, 5)          # can be negative
  c: nonzero(-8, 8)          # integer, excludes 0
  r: decimal(0.1, 5.0, 1)    # decimal with 1 decimal place
  k: choice(2, 3, 5, 7)      # pick from explicit list
  sign: choice(1, -1)         # random sign
  trig: choice(sin, cos, tan) # pick a function
  base: prime(2, 20)          # random prime
```

**Sampler types:**

| Sampler | Syntax | Description |
|---------|--------|-------------|
| `integer` | `integer(lo, hi)` | Uniform integer in [lo, hi] |
| `nonzero` | `nonzero(lo, hi)` | Integer in [lo, hi], excludes 0 |
| `decimal` | `decimal(lo, hi, places)` | Decimal with fixed precision |
| `choice` | `choice(a, b, c, ...)` | Pick from list (numbers, symbols, or functions) |
| `prime` | `prime(lo, hi)` | Random prime in range |
| `rational` | `rational(lo, hi, max_denom)` | Random fraction, simplified |
| `angle` | `angle(lo, hi, step)` | Angle in degrees, snapped to step |
| `vector` | `vector(dim, lo, hi)` | Random integer vector |
| `matrix` | `matrix(rows, cols, lo, hi)` | Random integer matrix |

### 4.2 Derived Variables (computed)

```yaml
variables:
  a: integer(2, 8)
  b: integer(1, 5)
  f: a*x^2 + b*x          # expression built from samplers
  g: (x + a) * (x - b)    # another expression
  answer: f + g            # computed from other derived vars
```

Derived variables use **plain math notation** — same syntax SymEngine accepts:

| Operation | Syntax | Example |
|-----------|--------|---------|
| Add/subtract | `+`, `-` | `a + b`, `x - 3` |
| Multiply | `*` | `3*x`, `a*b` |
| Divide | `/` | `x/2`, `(a+b)/(c-d)` |
| Power | `^` | `x^2`, `e^x` |
| Negative | `-` (prefix) | `-a`, `-(x+1)` |
| Grouping | `()` | `(a+b)*(c-d)` |

### 4.3 Function Variables (math operations)

These invoke built-in math operations. Parser executes them via SymEngine.

```yaml
variables:
  f: a*x^2 + b*x + c
  answer: solve(f, x)           # solve f = 0 for x
  df: derivative(f, x)          # differentiate
  F: integral(f, x)             # indefinite integral
  val: evaluate(f, x, 3)        # plug in x = 3
  s: simplify(expr)             # simplify expression
  fac: factor(f)                # factor polynomial
  exp: expand(g)                # expand product
```

**Built-in math functions:**

#### Algebra
| Function | Syntax | Returns |
|----------|--------|---------|
| `solve` | `solve(expr, var)` | Solution set |
| `factor` | `factor(expr)` | Factored form |
| `expand` | `expand(expr)` | Expanded form |
| `simplify` | `simplify(expr)` | Simplified form |
| `evaluate` | `evaluate(expr, var, val)` | Numeric result |
| `gcd` | `gcd(a, b)` | Greatest common divisor |
| `lcm` | `lcm(a, b)` | Least common multiple |
| `abs` | `abs(x)` | Absolute value |
| `mod` | `mod(a, n)` | a mod n |

#### Calculus
| Function | Syntax | Returns |
|----------|--------|---------|
| `derivative` | `derivative(expr, var)` | Derivative expression |
| `nth_derivative` | `nth_derivative(expr, var, n)` | nth derivative |
| `integral` | `integral(expr, var)` | Antiderivative (no +C) |
| `definite_integral` | `definite_integral(expr, var, lo, hi)` | Numeric value |
| `limit` | `limit(expr, var, val)` | Limit value |
| `limit_left` | `limit_left(expr, var, val)` | Left-hand limit |
| `limit_right` | `limit_right(expr, var, val)` | Right-hand limit |
| `partial` | `partial(expr, var)` | Partial derivative |
| `gradient` | `gradient(expr, [x, y])` | Gradient vector |

#### Linear Algebra
| Function | Syntax | Returns |
|----------|--------|---------|
| `det` | `det(M)` | Determinant |
| `inverse` | `inverse(M)` | Matrix inverse |
| `transpose` | `transpose(M)` | Transpose |
| `eigenvalues` | `eigenvalues(M)` | Set of eigenvalues |
| `eigenvectors` | `eigenvectors(M)` | List of eigenvectors |
| `rank` | `rank(M)` | Rank |
| `nullity` | `nullity(M)` | Nullity |
| `rref` | `rref(M)` | Row-reduced echelon form |
| `mat_mult` | `mat_mult(A, B)` | Matrix product |
| `dot` | `dot(u, v)` | Dot product |
| `cross` | `cross(u, v)` | Cross product |
| `magnitude` | `magnitude(v)` | Vector magnitude |
| `normalize` | `normalize(v)` | Unit vector |

#### Trigonometry
| Function | Syntax | Returns |
|----------|--------|---------|
| `sin`, `cos`, `tan` | `sin(x)` | Trig value |
| `asin`, `acos`, `atan` | `asin(x)` | Inverse trig |
| `sec`, `csc`, `cot` | `sec(x)` | Reciprocal trig |

#### Utility
| Function | Syntax | Returns |
|----------|--------|---------|
| `tex` | `tex(expr)` | Force LaTeX rendering of expression |
| `round` | `round(expr, places)` | Rounded decimal |
| `floor` | `floor(x)` | Floor |
| `ceil` | `ceil(x)` | Ceiling |
| `max` | `max(a, b)` | Maximum |
| `min` | `min(a, b)` | Minimum |

### 4.4 Constants

Available without declaration:

| Constant | Value |
|----------|-------|
| `pi` | 3.14159... |
| `e` | 2.71828... |
| `inf` | Infinity |
| `i` | Imaginary unit |

---

## 5. Constraints

Boolean conditions. Parser resamples variables until all constraints are satisfied (max 1000 attempts, then error).

```yaml
constraints:
  - b != 0                    # no division by zero
  - gcd(a, b) == 1            # reduced fraction
  - answer > 0                # positive answer
  - det(M) != 0               # invertible matrix
  - discriminant(f) >= 0      # real roots
  - is_integer(answer)         # whole number result
  - answer != a                # answer different from input
  - magnitude(v) > 0           # nonzero vector
```

**Built-in constraint helpers:**

| Helper | Description |
|--------|-------------|
| `is_integer(x)` | x is a whole number |
| `is_rational(x)` | x is a ratio of integers |
| `is_positive(x)` | x > 0 |
| `is_negative(x)` | x < 0 |
| `is_real(x)` | x has no imaginary part |
| `discriminant(f)` | b^2 - 4ac for quadratic f |
| `distinct(a, b, c)` | all values different |

---

## 6. Question Text

Plain English with `{var}` references and `{display()}` functions. **No LaTeX.**

```yaml
question: Find {derivative_of(f, x)}
question: Solve {equation(f, 0)} for x
question: Evaluate {definite_integral_of(f, x, 0, 1)}
question: What is {det_of(M)}?
question: Factor {f}
question: Simplify {f}
```

### 6.1 Variable References

`{var_name}` — parser evaluates variable, converts to LaTeX, wraps in `$...$`.

```yaml
# If f = 3*x^2 + 5*x
question: Factor {f}
# → "Factor $3x^{2} + 5x$"
```

### 6.2 Display Functions

Formatted math notation. Parser generates appropriate LaTeX structure.

| Display Function | Output |
|------------------|--------|
| `{f}` | Inline math: `$3x^2 + 5x$` |
| `{display(f)}` | Display math: `$$3x^2 + 5x$$` |
| `{derivative_of(f, x)}` | `$\frac{d}{dx}\left[3x^2 + 5x\right]$` |
| `{nth_derivative_of(f, x, 2)}` | `$\frac{d^2}{dx^2}\left[...\right]$` |
| `{partial_of(f, x)}` | `$\frac{\partial}{\partial x}\left[...\right]$` |
| `{integral_of(f, x)}` | `$\int 3x^2 + 5x \, dx$` |
| `{definite_integral_of(f, x, a, b)}` | `$\int_{a}^{b} ... \, dx$` |
| `{limit_of(f, x, val)}` | `$\lim_{x \to val} ...$` |
| `{sum_of(f, n, lo, hi)}` | `$\sum_{n=lo}^{hi} ...$` |
| `{product_of(f, n, lo, hi)}` | `$\prod_{n=lo}^{hi} ...$` |
| `{equation(f, rhs)}` | `$3x^2 + 5x = 0$` |
| `{system(eq1, eq2)}` | `$$\begin{cases} ... \\ ... \end{cases}$$` |
| `{matrix_of(M)}` | `$\begin{pmatrix} ... \end{pmatrix}$` |
| `{augmented_matrix(M, b)}` | `$\left[\begin{array}{cc|c}...\end{array}\right]$` |
| `{det_of(M)}` | `$\det\begin{pmatrix}...\end{pmatrix}$` |
| `{vec(v)}` | `$\vec{v}$` |
| `{norm(v)}` | `$\|\vec{v}\|$` |
| `{angle(A)}` | `$\angle A$` |
| `{set_of(s)}` | `$\{1, 2, 3\}$` |
| `{interval(type1, a, type2, b)}` | `$(a, b]$` etc. |
| `{abs_of(x)}` | `$|x|$` |
| `{cases(cond1, val1, cond2, val2)}` | `$$\begin{cases}...\end{cases}$$` |
| `{binomial(n, k)}` | `$\binom{n}{k}$` |

### 6.3 Multi-line Questions

Use `|` for multi-line:

```yaml
question: |
  Solve the system:
  {system(eq1, eq2, eq3)}
  Enter your answer as an ordered triple.
```

---

## 7. Answer Types

Parser auto-infers from answer value when `answer_type` is omitted:

| Inferred from | Type | Example value |
|---------------|------|---------------|
| Single number | `numeric` | `42`, `3.14`, `-7` |
| Expression with variable | `expression` | `3*x + 5` |
| Comma-separated pair | `tuple` | `3, -5` |
| Comma-separated 3+ | `list` | `1, 2, 3` |
| Curly brace set | `set` | `{1, 2, 3}` |
| Interval result | `interval` | `open:0, closed:5` |
| Matrix result | `matrix` | `[[1, 2], [3, 4]]` |
| true/false/yes/no | `boolean` | `true` |
| Text string | `word` | `linearly dependent` |
| Equation | `equation` | `x^2 + y^2 = 4` |
| Multi-expression | `multi_part` | parts separated by `|||` |

Override when needed:

```yaml
answer: result
answer_type: set    # force set even if auto-detect differs
```

---

## 8. Solution Steps

Same `{var}` and `{display()}` system as question. Each item is one step:

```yaml
solution:
  - Identify the function: {f}
  - Apply the power rule: {derivative_of(f, x)} = {df}
  - Simplify: {answer}
```

Parser renders each step with proper LaTeX. Steps joined with `\n` in `solution_latex` field.

---

## 9. Constraints Block

```yaml
constraints:
  - b != 0
  - gcd(a, b) == 1
  - answer > 0
  - discriminant(f) >= 0
  - is_integer(answer)
  - distinct(root1, root2)
  - det(M) != 0
```

Constraints support:
- Comparison: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Boolean: `and`, `or`, `not`
- Function calls: any math function from section 4.3
- Variable references

Parser resamples (up to 1000 tries) until all pass. Fails with clear error if unsatisfiable.

---

## 10. Variants

Multiple problem structures in one file:

```yaml
topic: algebra1/factoring_gcf
difficulty: easy

variants:
  - name: two_terms
    variables:
      gcf: choice(2, 3, 4, 5)
      a: gcf * integer(1, 4)
      b: gcf * integer(1, 4)
      f: a*x + b
      answer: factor(f)
    question: Factor out the GCF of {f}
    mode: factor
    solution:
      - The GCF of {a} and {b} is {gcf}
      - Factor: {answer}

  - name: three_terms
    variables:
      gcf: choice(2, 3, 5)
      a: gcf * integer(1, 3)
      b: gcf * integer(1, 3)
      c: gcf * integer(1, 3)
      f: a*x^2 + b*x + c
      answer: factor(f)
    question: Factor out the GCF of {f}
    mode: factor
    solution:
      - The GCF of {a}, {b}, and {c} is {gcf}
      - Factor: {answer}
```

Parser picks a random variant per generation. Each variant inherits top-level fields (`topic`, `difficulty`, etc.) but can override them.

---

## 11. Diagrams

Declarative SVG generation. AI describes what to draw. Parser generates rendering code and compiles to SVG.

### Rendering Engines

| Engine | Used for | Integration |
|--------|----------|-------------|
| **Typst + cetz** | Geometry, coord planes, graphs, number lines, force diagrams, field lines | In-process via `typst` Rust crate (~100ms) |
| **circuitikz** (LaTeX) | Circuit diagrams | CLI: `pdflatex` → `pdf2svg` (~1-2s) |

Parser translates diagram YAML → Typst/cetz markup (or circuitikz for circuits) → compiles to SVG → compresses via `compress_svg()`.

AI never sees Typst or LaTeX. AI only writes the diagram YAML block.

### 11.1 Coordinate Plane

```yaml
diagram:
  type: coordinate_plane
  x_range: [-5, 5]
  y_range: [-3, 7]
  grid: true
  elements:
    - line: {slope: m, intercept: b, color: blue, label: f}
    - line: {slope: m2, intercept: b2, color: red, label: g}
    - point: {x: 3, y: val, label: P, color: black}
    - shade: {between: [f, g], from: 0, to: 3, color: lightblue}
    - asymptote: {x: 2, style: dashed}
    - arrow: {from: [0, 0], to: [3, 4], label: v}
```

Rendered via: Typst + cetz `plot` module.

### 11.2 Triangle

```yaml
diagram:
  type: triangle
  vertices: [A, B, C]
  sides:
    AB: 5
    BC: x
    AC: 8
  angles:
    A: 30
  marks:
    BC: unknown     # "?" label
    AB: tick        # congruence tick mark
  right_angle: C
```

Rendered via: Typst + cetz. Parser computes vertex positions from side lengths + angles using law of cosines, draws with `cetz.draw`.

### 11.3 Circle

```yaml
diagram:
  type: circle
  center: O
  radius: r
  elements:
    - chord: {from: A, to: B, label: "8"}
    - arc: {from: A, to: B, label: theta}
    - radius: {to: A, label: r}
    - tangent: {at: A, label: t}
    - central_angle: {vertex: O, sides: [A, B], label: "60"}
    - inscribed_angle: {vertex: C, sides: [A, B], label: alpha}
```

Rendered via: Typst + cetz. Parser places points on circle at computed angles.

### 11.4 Polygon

```yaml
diagram:
  type: polygon
  vertices: [A, B, C, D]
  sides:
    AB: 6
    CD: 6
  angles:
    A: 90
  parallel: [[AB, CD]]
  labels:
    center: "Area = ?"
```

Rendered via: Typst + cetz.

### 11.5 Number Line

```yaml
diagram:
  type: number_line
  range: [-5, 5]
  elements:
    - point: {at: 3, label: a, style: filled}
    - point: {at: -2, label: b, style: open}
    - segment: {from: -2, to: 3, color: blue}
    - arrow: {from: 3, direction: right, color: red}
```

Rendered via: Typst + cetz. Simple axis with tick marks + elements.

### 11.6 Graph of Function

```yaml
diagram:
  type: function_graph
  functions:
    - expr: f
      color: blue
      label: "f(x)"
    - expr: g
      color: red
      label: "g(x)"
  x_range: [-2*pi, 2*pi]
  y_range: [-3, 3]
  features:
    - zero: {of: f, label: true}
    - maximum: {of: f, label: true}
    - inflection: {of: f, style: dot}
```

Rendered via: Typst + cetz `plot`. Parser evaluates `f` at sample points, plots curve, marks features.

### 11.7 Force Diagram (Physics C: Mechanics)

```yaml
diagram:
  type: force_diagram
  object: block      # block, sphere, point, beam
  surface: incline   # flat, incline, none
  incline_angle: theta
  forces:
    - gravity: {magnitude: mg, label: "mg"}
    - normal: {label: "N"}
    - friction: {direction: up_incline, label: "f"}
    - applied: {angle: 30, magnitude: F, label: "F"}
```

Rendered via: Typst + cetz. Parser draws object shape, surface, then force arrows at correct angles with labels.

### 11.8 Field Lines (Physics C: E&M)

```yaml
diagram:
  type: field
  field_type: electric    # electric, magnetic
  sources:
    - charge: {value: q, position: [-2, 0], label: "+q"}
    - charge: {value: -q, position: [2, 0], label: "-q"}
  show_lines: true
  show_equipotential: false
  region: [-5, 5, -5, 5]
```

Rendered via: Typst + cetz. Parser computes field line trajectories numerically from source positions and charge values.

### 11.9 Circuit (Physics C: E&M)

```yaml
diagram:
  type: circuit
  elements:
    - battery: {voltage: V, between: [A, B]}
    - resistor: {resistance: R1, between: [B, C], label: "R1"}
    - resistor: {resistance: R2, between: [C, D], label: "R2"}
    - capacitor: {capacitance: C1, between: [D, A], label: "C1"}
    - wire: {from: A, to: B}
  layout: series    # series, parallel, bridge, wheatstone
```

Rendered via: **circuitikz** (LaTeX). Parser generates `.tex` with circuitikz commands → `pdflatex` → `pdf2svg` → compress. Used instead of Typst because circuitikz has the most mature circuit component library (resistors, capacitors, inductors, op-amps, transistors, voltage/current sources, grounds, switches).

Node positions auto-computed from `layout` + `between` declarations. AI never specifies coordinates.

### 11.10 Diagram Element Reference

**Colors:** `black`, `blue`, `red`, `green`, `orange`, `purple`, `gray`, `lightblue`, `lightgreen`

**Styles:** `solid`, `dashed`, `dotted`, `thick`

**Point styles:** `filled`, `open`, `cross`, `square`

**Labels:** Any `{var}` reference or quoted string. Parser renders via Typst math mode (or circuitikz labels for circuits).

**All coordinates and positions are computed by the parser** from the declarative spec. AI specifies relationships (side lengths, angles, connections), not pixel positions.

---

## 12. Topics (Closed Enum)

### Current

```
arithmetic/{addition, subtraction, multiplication, long_division, fractions,
            decimals, mixed_numbers, order_of_operations, percentages,
            ratios_proportions}

algebra1/{one_step_equations, two_step_equations, multi_step_equations,
          linear_inequalities, compound_inequalities, exponent_rules,
          polynomial_operations, factoring_gcf, factoring_trinomials,
          quadratic_formula, graphing_lines, slope_and_intercept,
          systems_substitution, systems_elimination}

algebra2/{complex_number_operations, complex_number_equations,
          exponential_equations, exponential_growth_decay,
          logarithm_properties, logarithmic_equations,
          radical_expressions, radical_equations,
          rational_expressions, rational_equations,
          arithmetic_sequences, geometric_sequences}

geometry/{angle_relationships, triangle_properties, triangle_congruence,
          similar_triangles, pythagorean_theorem, right_triangle_trig,
          perimeter, area_of_polygons, circle_theorems, arc_length_sectors,
          surface_area, volume, coordinate_geometry}

precalculus/{domain_and_range, function_composition, inverse_functions,
             transformations, unit_circle, graphing_trig, trig_identities,
             sum_difference_formulas, inverse_trig_functions,
             law_of_sines_cosines, vector_operations, dot_cross_product,
             polar_coordinates, polar_curves}

calculus/{continuity, lhopitals_rule, limits_at_infinity,
          derivative_rules, chain_rule, implicit_differentiation,
          related_rates, curve_sketching, optimization,
          antiderivatives, u_substitution, integration_by_parts,
          definite_integrals, area_between_curves, volumes_of_revolution}

multivariable_calculus/{partial_derivatives, gradient, directional_derivatives,
                        lagrange_multipliers, double_integrals, triple_integrals,
                        change_of_variables, line_integrals, greens_theorem,
                        stokes_divergence}

linear_algebra/{matrix_arithmetic, matrix_inverses, determinants,
                row_reduction, eigenvalues, diagonalization,
                vector_spaces, subspaces, linear_independence,
                linear_transformations}

differential_equations/{separable_equations, first_order_linear, exact_equations,
                        homogeneous_equations, second_order_constant,
                        characteristic_equation, undetermined_coefficients,
                        variation_of_parameters, laplace_transforms,
                        systems_of_odes}
```

### Planned (Physics C)

```
physics_mechanics/{kinematics_1d, kinematics_2d, projectile_motion,
                   newton_laws, friction, circular_motion,
                   work_energy, conservation_energy,
                   momentum, collisions, impulse,
                   rotational_kinematics, rotational_dynamics,
                   torque, angular_momentum,
                   simple_harmonic_motion, pendulum,
                   gravitation, orbits}

physics_em/{coulombs_law, electric_field, electric_potential,
            gauss_law, capacitance, dielectrics,
            dc_circuits, rc_circuits, kirchhoff_laws,
            magnetic_field, biot_savart, ampere_law,
            faraday_law, inductance, lenz_law,
            ac_circuits, maxwell_equations,
            electromagnetic_waves}
```

---

## 13. Physics Extensions

Physics problems need units and physical constants.

### 13.1 Units

```yaml
variables:
  m: decimal(1, 10, 1) kg
  v: decimal(0, 30, 1) m/s
  F: m * a
  KE: 0.5 * m * v^2
  answer: KE
  answer_unit: J
```

Parser tracks units, validates dimensional consistency, formats with proper unit symbols.

**Supported units:**

| Quantity | Units |
|----------|-------|
| Length | `m`, `cm`, `mm`, `km` |
| Mass | `kg`, `g` |
| Time | `s`, `ms`, `min`, `hr` |
| Velocity | `m/s`, `km/hr` |
| Acceleration | `m/s^2` |
| Force | `N` |
| Energy | `J`, `eV` |
| Power | `W` |
| Charge | `C` |
| Voltage | `V` |
| Current | `A` |
| Resistance | `ohm` |
| Capacitance | `F`, `uF`, `nF`, `pF` |
| Magnetic field | `T` |
| Inductance | `H` |
| Angle | `deg`, `rad` |

### 13.2 Physical Constants

Available without declaration:

| Constant | Symbol | Value |
|----------|--------|-------|
| `g` | g | 9.8 m/s^2 |
| `G` | G | 6.674e-11 N*m^2/kg^2 |
| `k_e` | k_e | 8.99e9 N*m^2/C^2 |
| `epsilon_0` | epsilon_0 | 8.854e-12 F/m |
| `mu_0` | mu_0 | 4*pi*1e-7 T*m/A |
| `c` | c | 3e8 m/s |
| `e_charge` | e | 1.6e-19 C |
| `m_electron` | m_e | 9.11e-31 kg |
| `m_proton` | m_p | 1.67e-27 kg |

---

## 14. Validation Pipeline

Parser validates in layers:

### Layer 1: Structural
- YAML well-formed
- Required fields present
- Field types correct
- Topic in known list
- `answer_type` in known enum

### Layer 2: Variable Resolution
- All variable references resolve
- No circular dependencies
- Sampler ranges valid (lo < hi)
- Function arguments match expected arity

### Layer 3: Constraint Satisfaction
- All constraints satisfiable within 1000 samples
- No contradictory constraints
- No division by zero possible

### Layer 4: Mathematical
- All expressions parse in SymEngine
- Answer evaluates to a value
- Answer matches declared `answer_type`

### Layer 5: Grading Round-Trip
- `grade_answer(answer_key, answer_key)` returns `Correct`
- Uses same grading mode declared in `mode`

### Layer 6: Rendering
- All `{var}` references in question/solution resolve
- All `{display()}` functions have valid arguments
- Generated LaTeX passes `validate_katex()`
- Diagram spec valid (if present)

**Any layer failure → reject with clear error message pointing to the exact field and problem.**

---

## 15. Output Format

Parser produces a JSON object matching the existing `Problem` database schema:

```json
{
  "question_latex": "Factor $3x^{2} + 5x$",
  "answer_key": "x*(3*x + 5)",
  "solution_latex": "The GCF is $x$\nFactor: $x(3x + 5)$",
  "difficulty": 1150,
  "main_topic": "algebra1",
  "subtopic": "factoring_gcf",
  "grading_mode": "factor",
  "answer_type": "expression",
  "calculator_allowed": "none",
  "question_image": "",
  "time_limit_seconds": null
}
```

`question_image` contains compressed SVG if `diagram` was specified.

---

## 16. Complete Examples

### Example 1: Basic Arithmetic

```yaml
topic: arithmetic/fractions
difficulty: easy

variables:
  a: integer(1, 9)
  b: integer(2, 9)
  c: integer(1, 9)
  d: integer(2, 9)
  answer: a/b + c/d

constraints:
  - gcd(a, b) == 1
  - gcd(c, d) == 1
  - b != d

question: Simplify {a}/{b} + {c}/{d}

answer: answer
mode: equivalent

solution:
  - Find common denominator: {b} and {d} have LCD {lcm(b, d)}
  - Add: {answer}
```

### Example 2: Calculus Optimization

```yaml
topic: calculus/optimization
difficulty: hard

variables:
  a: nonzero(1, 6)
  b: nonzero(1, 8)
  f: a*x^3 - b*x
  df: derivative(f, x)
  critical: solve(df, x)
  answer: evaluate(f, x, min(critical))

constraints:
  - is_real(critical)
  - distinct(elements(critical))

question: |
  Find the minimum value of {f} on the real line.

answer: answer

solution:
  - Find critical points: {derivative_of(f, x)} = {df} = 0
  - Solve: x = {critical}
  - Evaluate {f} at critical points
  - Minimum value: {answer}
```

### Example 3: Linear Algebra with Diagram

```yaml
topic: linear_algebra/eigenvalues
difficulty: medium

variables:
  M: matrix(2, 2, -5, 5)
  eigenvals: eigenvalues(M)
  answer: eigenvals

constraints:
  - det(M) != 0
  - is_real(eigenvals)
  - distinct(elements(eigenvals))

question: Find all eigenvalues of {matrix_of(M)}

answer: answer
answer_type: set

solution:
  - Characteristic equation: {equation(det_expr(M - lambda*I), 0)}
  - Solve for lambda: {answer}
```

### Example 4: Physics Mechanics

```yaml
topic: physics_mechanics/work_energy
difficulty: medium

variables:
  m: decimal(1, 10, 1) kg
  v0: decimal(2, 15, 1) m/s
  v1: decimal(0, 10, 1) m/s
  KE0: 0.5 * m * v0^2
  KE1: 0.5 * m * v1^2
  answer: KE1 - KE0
  answer_unit: J

constraints:
  - v0 > v1

question: |
  A {m} kg block moving at {v0} m/s slows to {v1} m/s.
  What is the work done on the block?

answer: answer

diagram:
  type: force_diagram
  object: block
  surface: flat
  forces:
    - gravity: {magnitude: m*g, label: "mg"}
    - normal: {label: "N"}
    - friction: {direction: left, label: "f"}
    - velocity: {direction: right, magnitude: v0, label: "v"}

solution:
  - Work-energy theorem: W = change in KE
  - KE initial = {KE0} J
  - KE final = {KE1} J
  - W = {KE1} - {KE0} = {answer} J
```

### Example 5: Geometry with Diagram

```yaml
topic: geometry/pythagorean_theorem
difficulty: easy

variables:
  a: choice(3, 5, 6, 7, 8)
  b: choice(4, 8, 9, 12, 15)
  c: sqrt(a^2 + b^2)
  answer: c

constraints:
  - is_integer(c)
  - a < b

question: |
  Find the hypotenuse of a right triangle with legs {a} and {b}.

answer: answer

diagram:
  type: triangle
  vertices: [A, B, C]
  right_angle: C
  sides:
    AC: a
    BC: b
    AB: "?"
  labels:
    AB: "c = ?"

solution:
  - Pythagorean theorem: a^2 + b^2 = c^2
  - {a}^2 + {b}^2 = c^2
  - c = {answer}
```

### Example 6: Systems with Variants

```yaml
topic: algebra1/systems_elimination
difficulty: medium

variants:
  - name: two_var
    variables:
      a1: nonzero(-5, 5)
      b1: nonzero(-5, 5)
      a2: nonzero(-5, 5)
      b2: nonzero(-5, 5)
      sol_x: integer(-8, 8)
      sol_y: integer(-8, 8)
      c1: a1*sol_x + b1*sol_y
      c2: a2*sol_x + b2*sol_y
      eq1: equation(a1*x + b1*y, c1)
      eq2: equation(a2*x + b2*y, c2)
      answer: sol_x, sol_y
    constraints:
      - det([[a1, b1], [a2, b2]]) != 0
    question: |
      Solve the system using elimination:
      {system(eq1, eq2)}
    answer: answer
    answer_type: tuple
    solution:
      - Multiply equations to align coefficients
      - Eliminate one variable
      - x = {sol_x}, y = {sol_y}

  - name: three_var
    variables:
      sol_x: integer(-5, 5)
      sol_y: integer(-5, 5)
      sol_z: integer(-5, 5)
      # ... similar pattern with 3 equations
    difficulty: hard
    question: |
      Solve the system:
      {system(eq1, eq2, eq3)}
    answer: sol_x, sol_y, sol_z
    answer_type: tuple
```

---

## 17. Error Messages

Parser errors are structured for the generate-validate-repair loop:

```json
{
  "errors": [
    {
      "layer": "variable_resolution",
      "field": "variables.answer",
      "message": "Function 'derivativ' not found. Did you mean 'derivative'?",
      "line": 8
    },
    {
      "layer": "constraint",
      "field": "constraints[0]",
      "message": "Constraint 'b != 0' unsatisfiable: b is sampled from integer(0, 5) which always includes 0. Use nonzero(1, 5) instead.",
      "line": 12
    }
  ]
}
```

If fed back to the AI, these messages are specific enough to auto-correct on retry.

---

## 18. File Naming Convention

```
{topic}_{subtopic}_{difficulty}.yaml

# Examples:
calculus_derivative_rules_medium.yaml
algebra1_factoring_gcf_easy.yaml
physics_mechanics_work_energy_hard.yaml
```

Standard `.yaml` extension → IDE syntax highlighting, linting, schema validation work out of box.

Multiple difficulty levels per file via variants or separate files.
