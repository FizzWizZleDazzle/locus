# PER-TOPIC PLAYBOOK

For each topic, the playbook lists the one or two specific tools/structures that prevent the most common mistakes for THAT topic. When you start a problem, look up your topic in this list before drafting.

## arithmetic/*

Pure number problems. Pick clean inputs, compute the answer with `compute`. No CAS gymnastics needed.

- `arithmetic/percentages` — pick `base` from {100, 200, 250, 400, 500} and `rate` from {10, 15, 20, 25, 50}. Answer = `base*(1 + rate/100)` for increase, `base*(1 - rate/100)` for decrease.
- `arithmetic/fractions` — pick numerator and denominator separately as integers, take LCD. Use `compute` to verify the simplified result.
- `arithmetic/order_of_operations` — pick the answer first, then design an expression that produces it.

## algebra1/*

- `algebra1/quadratic_formula` — pick roots first, then build coefficients with `polynomial_from_roots([r1, r2])`. Answer is the SET of roots, `answer_type: set`. Use `quadratic_roots(a, b, c)` to verify.
- `algebra1/linear_inequalities` — use `solve_linear_inequality(a, b, c, d, op)`. The answer is the inequality (`x < N` or `x > N`), not the boundary value. Set `answer_type: inequality`. Don't write `answer: x_val` and hope.
- `algebra1/compound_inequalities` — return both bounds as a tuple: `answer: lo, hi` with `answer_type: tuple`.
- `algebra1/factoring_gcf` — pick GCF and per-term integer coefficients, build the expression as `gcf*c1*x + gcf*c2`, answer is the factored form.
- `algebra1/factoring_trinomials` — pick roots r1, r2, build `(x - r1)*(x - r2)`, expand for the question. Use `polynomial_from_roots`.
- `algebra1/graphing_lines` / `slope_and_intercept` — pick slope `m` and a point (x1, y1), compute intercept `b = y1 - m*x1`. Add a `coordinate_plane` diagram.
- `algebra1/systems_*` — pick `(x_sol, y_sol)` first, build coefficients to reproduce them. Constraint: `a1*b2 != a2*b1` for unique solution.
- `algebra1/multi_step_equations` / `two_step_equations` / `one_step_equations` — pick the answer, work backward through the operations.
- `algebra1/exponent_rules` — pick base + exponents, compute via `compute`. Answer is a single integer or simple expression.
- `algebra1/polynomial_operations` — write the operands as polynomials with small integer coefficients. Use `expand` for products.

## algebra2/*

- `algebra2/exponential_equations` — pick base and integer exponent, compute RHS = base^exp. Answer is the exponent.
- `algebra2/logarithmic_equations` / `logarithm_properties` — pick base from {2, 3, 5, 10}, pick integer exponent. Argument = base^exponent. Answer = exponent. Use `compute("log(arg, base)")` to verify.
- `algebra2/exponential_growth_decay` — pick `principal`, `rate`, integer `years`. Use `compute` for the final amount, round if needed.
- `algebra2/arithmetic_sequences` — pick `a1` and common difference `d`, compute the n-th term as `a1 + (n-1)*d`.
- `algebra2/geometric_sequences` — pick `a1`, ratio `r`, integer `n`. Compute `S_n = a1*(r^n - 1)/(r - 1)`.
- `algebra2/complex_number_*` — pick real/imaginary parts of inputs, compute the result as `(ar + br) + (ai + bi)*i`. Use `i` (SymEngine recognizes it).
- `algebra2/radical_equations` / `radical_expressions` — pick perfect-square radicand. Answer is the radical's exact value via `compute("sqrt(N)")`.
- `algebra2/rational_equations` / `rational_expressions` — pick the value first, build numerator/denominator. Watch for excluded values.

## calculus/*

- `calculus/derivative_rules` — pick a polynomial with `a*x^n + b*x^m` form. Answer = `differentiate(f, x)`.
- `calculus/chain_rule` — write `f = (inner)^outer_exp`. Use `differentiate` on the composed form.
- `calculus/implicit_differentiation` — pick `y(x)` first, compute `dy/dx` via implicit rule. Answer is the derivative expression.
- `calculus/continuity` — pick the boundary value and one piece's value at the boundary. Compute the parameter that makes the other piece match. Don't introduce orphan variables; the answer IS the value the discontinuous piece must take. Use `evaluate_at` to verify both pieces give the same number.
- `calculus/limits_at_infinity` — for rational functions with same-degree numerator/denominator, answer = ratio of leading coefficients. Build numerator/denominator with the SAME `m` for the leading exponent so this rule applies.
- `calculus/lhopitals_rule` — design so direct substitution gives 0/0. Answer = limit of derivatives at the point. Use `differentiate` and `evaluate_at`.
- `calculus/antiderivatives` / `definite_integrals` — pick the antiderivative F first, set f = differentiate(F). Use `integrate(expr, var)` for indefinite, `integrate(expr, var, lo, hi)` for definite.
- `calculus/u_substitution` — pick u(x) and the antiderivative G(u). Compose to get the integrand. Answer = G(u(b)) - G(u(a)).
- `calculus/integration_by_parts` — design as `u*dv` with both u and v polynomial-friendly. Answer = `u*v - integral(v*du)`.
- `calculus/area_between_curves` — pick two functions with clean intersection points. Use `definite_integral`.
- `calculus/volumes_of_revolution` — disk/shell/washer: pick the outer-radius function, compute via definite_integral of `pi*r^2`.
- `calculus/curve_sketching` — pick a polynomial; answer is critical points or inflection points. Use `solve(differentiate(f, x), x)`.
- `calculus/optimization` — pick the optimal value first; design the objective so its critical point is at that value.
- `calculus/related_rates` — pick the unknown rate; design the geometric setup so dy/dt or dx/dt resolves to a clean number.

## differential_equations/*

- `differential_equations/separable_equations` — pick the solution `y(x)` first, write `dy/dx` and the equation. Initial condition picks the constant.
- `differential_equations/first_order_linear` — pick mu(x) and y(x) backward; build the equation.
- `differential_equations/characteristic_equation` — pick roots r1, r2 of the characteristic poly. Build `a = -(r1+r2)`, `b = r1*r2`, equation `r^2 + a*r + b`.
- `differential_equations/homogeneous_equations` / `second_order_constant` — pick the two roots, build the ODE coefficients.
- `differential_equations/undetermined_coefficients` — pick the particular solution, derive the forcing term from it.
- `differential_equations/variation_of_parameters` — design the homogeneous solution + a clean particular solution.
- `differential_equations/exact_equations` — pick a potential function F(x, y), the ODE is `dF/dx*dx + dF/dy*dy = 0`.
- `differential_equations/laplace_transforms` — pick the time-domain function with a known transform table entry. Answer is the transform.
- `differential_equations/systems_of_odes` — use `matrix_with_eigenvalues(λ1, λ2)` to build the coefficient matrix. The eigenvalues ARE the answer. Never construct the matrix entries first.

## geometry/*

Always include a `diagram:` block when the topic is visual.

- `geometry/triangle_*` / `pythagorean_theorem` / `right_triangle_trig` — `type: triangle` diagram. Pick the answer first; for trig, pick angle from {30, 45, 60} and use `compute` for exact ratios.
- `geometry/circle_*` / `arc_length_sectors` — `type: circle` diagram. Pick radius and central angle in degrees; convert to radians inside `compute` for arc length.
- `geometry/perimeter` / `area_of_polygons` — pick side lengths or radii. Use `type: polygon` for non-rectangular shapes.
- `geometry/surface_area` / `volume` — pick dimensions. Use `compute` for cylinders, cones, spheres in terms of pi.
- `geometry/similar_triangles` — pick scale factor, build proportional sides.
- `geometry/triangle_congruence` — answer is the criterion (`SSS`, `SAS`, `ASA`, `AAS`, `HL`). `answer_type: word`.
- `geometry/angle_relationships` — pick one angle, derive the partner via supplementary/complementary/vertical relationships.

## linear_algebra/*

- `linear_algebra/determinants` — pick small integer entries, compute det via `compute("a*d - b*c")` for 2x2 or expand for 3x3.
- `linear_algebra/eigenvalues` / `diagonalization` — use `matrix_with_eigenvalues(λ1, λ2)` to build the matrix.
- `linear_algebra/matrix_arithmetic` / `matrix_inverses` — pick small integer matrices. For inverse, design so determinant divides evenly.
- `linear_algebra/row_reduction` — pick an invertible matrix; the answer is the RREF (which is identity or close).
- `linear_algebra/linear_independence` / `linear_transformations` / `vector_spaces` / `subspaces` — answer is often boolean or word (`yes`, `no`, `dependent`, `independent`).

## multivariable_calculus/*

- `multivariable_calculus/partial_derivatives` — write `f(x, y)` as a sum of polynomial terms. Use `differentiate` for each partial.
- `multivariable_calculus/gradient` — answer is a tuple `(∂f/∂x, ∂f/∂y)`. Use `differentiate` and `evaluate_at` to substitute the point.
- `multivariable_calculus/directional_derivatives` — gradient · unit vector. Pre-compute the unit vector to avoid messy radicals.
- `multivariable_calculus/double_integrals` / `triple_integrals` — pick the antiderivative chain first. Use nested `integrate` calls.
- `multivariable_calculus/line_integrals` / `greens_theorem` — pick the parametrization and the field; verify with `compute`.
- `multivariable_calculus/lagrange_multipliers` — pick the optimal point on the constraint, design the objective so its critical point lands there.
- `multivariable_calculus/change_of_variables` — pick the Jacobian determinant; design the transformation backward.

## precalculus/*

- `precalculus/unit_circle` — answer is an exact trig value at a standard angle. Use `compute("sin(pi/6)")` etc.
- `precalculus/trig_identities` / `sum_difference_formulas` — pick the answer; derive the equivalent expression. Use `compute` for verification.
- `precalculus/inverse_trig_functions` — answer is an angle (radians or degrees, be consistent).
- `precalculus/graphing_trig` — pick amplitude, period, phase, vertical shift. Add a `function_graph` diagram.
- `precalculus/polar_coordinates` / `polar_curves` — pick r(theta), evaluate at standard angles.
- `precalculus/vector_operations` / `dot_cross_product` — pick small integer vectors. Cross product is 3D; dot is any dimension.
- `precalculus/function_composition` / `inverse_functions` — pick f(x), compose / invert symbolically.
- `precalculus/domain_and_range` — answer is an interval or union; use `answer_type: tuple` for boundary pairs.
- `precalculus/transformations` — pick parent function and shifts, answer is the transformed expression.
- `precalculus/law_of_sines_cosines` — pick triangle sides/angles such that the unknown comes out clean. `compute` for verification.
