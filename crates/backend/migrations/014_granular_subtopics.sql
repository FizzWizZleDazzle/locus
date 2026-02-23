-- Migration: Replace subtopics with more granular ones + add differential_equations topic
-- Old problems keep their subtopic strings; they just won't match new subtopic filters.

-- Delete all existing subtopics
DELETE FROM subtopics;

-- Add differential_equations topic (between calculus and multivariable_calculus)
INSERT INTO topics (id, display_name, sort_order, enabled) VALUES
    ('differential_equations', 'Differential Equations', 7, TRUE);

-- Bump sort_order for topics after calculus to make room
UPDATE topics SET sort_order = 8 WHERE id = 'multivariable_calculus';
UPDATE topics SET sort_order = 9 WHERE id = 'linear_algebra';

-- Arithmetic (10)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('arithmetic', 'addition', 'Addition', 1, TRUE),
    ('arithmetic', 'subtraction', 'Subtraction', 2, TRUE),
    ('arithmetic', 'multiplication', 'Multiplication', 3, TRUE),
    ('arithmetic', 'long_division', 'Long Division', 4, TRUE),
    ('arithmetic', 'fractions', 'Fractions', 5, TRUE),
    ('arithmetic', 'mixed_numbers', 'Mixed Numbers', 6, TRUE),
    ('arithmetic', 'decimals', 'Decimals', 7, TRUE),
    ('arithmetic', 'percentages', 'Percentages', 8, TRUE),
    ('arithmetic', 'order_of_operations', 'Order of Operations', 9, TRUE),
    ('arithmetic', 'ratios_proportions', 'Ratios & Proportions', 10, TRUE);

-- Algebra 1 (14)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('algebra1', 'one_step_equations', 'One-Step Equations', 1, TRUE),
    ('algebra1', 'two_step_equations', 'Two-Step Equations', 2, TRUE),
    ('algebra1', 'multi_step_equations', 'Multi-Step Equations', 3, TRUE),
    ('algebra1', 'linear_inequalities', 'Linear Inequalities', 4, TRUE),
    ('algebra1', 'compound_inequalities', 'Compound Inequalities', 5, TRUE),
    ('algebra1', 'slope_and_intercept', 'Slope & Intercept', 6, TRUE),
    ('algebra1', 'graphing_lines', 'Graphing Lines', 7, TRUE),
    ('algebra1', 'systems_substitution', 'Systems: Substitution', 8, TRUE),
    ('algebra1', 'systems_elimination', 'Systems: Elimination', 9, TRUE),
    ('algebra1', 'exponent_rules', 'Exponent Rules', 10, TRUE),
    ('algebra1', 'polynomial_operations', 'Polynomial Operations', 11, TRUE),
    ('algebra1', 'factoring_gcf', 'Factoring: GCF', 12, TRUE),
    ('algebra1', 'factoring_trinomials', 'Factoring Trinomials', 13, TRUE),
    ('algebra1', 'quadratic_formula', 'Quadratic Formula', 14, TRUE);

-- Geometry (12)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('geometry', 'angle_relationships', 'Angle Relationships', 1, TRUE),
    ('geometry', 'triangle_properties', 'Triangle Properties', 2, TRUE),
    ('geometry', 'triangle_congruence', 'Triangle Congruence', 3, TRUE),
    ('geometry', 'similar_triangles', 'Similar Triangles', 4, TRUE),
    ('geometry', 'circle_theorems', 'Circle Theorems', 5, TRUE),
    ('geometry', 'arc_length_sectors', 'Arc Length & Sectors', 6, TRUE),
    ('geometry', 'area_of_polygons', 'Area of Polygons', 7, TRUE),
    ('geometry', 'perimeter', 'Perimeter', 8, TRUE),
    ('geometry', 'surface_area', 'Surface Area', 9, TRUE),
    ('geometry', 'volume', 'Volume', 10, TRUE),
    ('geometry', 'pythagorean_theorem', 'Pythagorean Theorem', 11, TRUE),
    ('geometry', 'right_triangle_trig', 'Right Triangle Trig', 12, TRUE);

-- Algebra 2 (12)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('algebra2', 'complex_number_operations', 'Complex Number Operations', 1, TRUE),
    ('algebra2', 'complex_number_equations', 'Complex Number Equations', 2, TRUE),
    ('algebra2', 'rational_expressions', 'Rational Expressions', 3, TRUE),
    ('algebra2', 'rational_equations', 'Rational Equations', 4, TRUE),
    ('algebra2', 'radical_expressions', 'Radical Expressions', 5, TRUE),
    ('algebra2', 'radical_equations', 'Radical Equations', 6, TRUE),
    ('algebra2', 'exponential_growth_decay', 'Exponential Growth & Decay', 7, TRUE),
    ('algebra2', 'exponential_equations', 'Exponential Equations', 8, TRUE),
    ('algebra2', 'logarithm_properties', 'Logarithm Properties', 9, TRUE),
    ('algebra2', 'logarithmic_equations', 'Logarithmic Equations', 10, TRUE),
    ('algebra2', 'arithmetic_sequences', 'Arithmetic Sequences', 11, TRUE),
    ('algebra2', 'geometric_sequences', 'Geometric Sequences', 12, TRUE);

-- Precalculus (14)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('precalculus', 'domain_and_range', 'Domain & Range', 1, TRUE),
    ('precalculus', 'function_composition', 'Function Composition', 2, TRUE),
    ('precalculus', 'inverse_functions', 'Inverse Functions', 3, TRUE),
    ('precalculus', 'transformations', 'Transformations', 4, TRUE),
    ('precalculus', 'unit_circle', 'Unit Circle', 5, TRUE),
    ('precalculus', 'graphing_trig', 'Graphing Trig', 6, TRUE),
    ('precalculus', 'trig_identities', 'Trig Identities', 7, TRUE),
    ('precalculus', 'sum_difference_formulas', 'Sum & Difference Formulas', 8, TRUE),
    ('precalculus', 'inverse_trig_functions', 'Inverse Trig Functions', 9, TRUE),
    ('precalculus', 'law_of_sines_cosines', 'Law of Sines & Cosines', 10, TRUE),
    ('precalculus', 'polar_coordinates', 'Polar Coordinates', 11, TRUE),
    ('precalculus', 'polar_curves', 'Polar Curves', 12, TRUE),
    ('precalculus', 'vector_operations', 'Vector Operations', 13, TRUE),
    ('precalculus', 'dot_cross_product', 'Dot & Cross Product', 14, TRUE);

-- Calculus (16)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('calculus', 'limits_algebraic', 'Limits: Algebraic', 1, TRUE),
    ('calculus', 'limits_at_infinity', 'Limits at Infinity', 2, TRUE),
    ('calculus', 'continuity', 'Continuity', 3, TRUE),
    ('calculus', 'derivative_rules', 'Derivative Rules', 4, TRUE),
    ('calculus', 'chain_rule', 'Chain Rule', 5, TRUE),
    ('calculus', 'implicit_differentiation', 'Implicit Differentiation', 6, TRUE),
    ('calculus', 'related_rates', 'Related Rates', 7, TRUE),
    ('calculus', 'curve_sketching', 'Curve Sketching', 8, TRUE),
    ('calculus', 'optimization', 'Optimization', 9, TRUE),
    ('calculus', 'lhopitals_rule', 'L''Hopital''s Rule', 10, TRUE),
    ('calculus', 'antiderivatives', 'Antiderivatives', 11, TRUE),
    ('calculus', 'u_substitution', 'U-Substitution', 12, TRUE),
    ('calculus', 'integration_by_parts', 'Integration by Parts', 13, TRUE),
    ('calculus', 'definite_integrals', 'Definite Integrals', 14, TRUE),
    ('calculus', 'area_between_curves', 'Area Between Curves', 15, TRUE),
    ('calculus', 'volumes_of_revolution', 'Volumes of Revolution', 16, TRUE);

-- Differential Equations (10)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('differential_equations', 'separable_equations', 'Separable Equations', 1, TRUE),
    ('differential_equations', 'first_order_linear', 'First-Order Linear', 2, TRUE),
    ('differential_equations', 'exact_equations', 'Exact Equations', 3, TRUE),
    ('differential_equations', 'homogeneous_equations', 'Homogeneous Equations', 4, TRUE),
    ('differential_equations', 'second_order_constant', 'Second-Order Constant Coefficient', 5, TRUE),
    ('differential_equations', 'characteristic_equation', 'Characteristic Equation', 6, TRUE),
    ('differential_equations', 'undetermined_coefficients', 'Undetermined Coefficients', 7, TRUE),
    ('differential_equations', 'variation_of_parameters', 'Variation of Parameters', 8, TRUE),
    ('differential_equations', 'laplace_transforms', 'Laplace Transforms', 9, TRUE),
    ('differential_equations', 'systems_of_odes', 'Systems of ODEs', 10, TRUE);

-- Multivariable Calculus (10)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('multivariable_calculus', 'partial_derivatives', 'Partial Derivatives', 1, TRUE),
    ('multivariable_calculus', 'gradient', 'Gradient', 2, TRUE),
    ('multivariable_calculus', 'directional_derivatives', 'Directional Derivatives', 3, TRUE),
    ('multivariable_calculus', 'lagrange_multipliers', 'Lagrange Multipliers', 4, TRUE),
    ('multivariable_calculus', 'double_integrals', 'Double Integrals', 5, TRUE),
    ('multivariable_calculus', 'triple_integrals', 'Triple Integrals', 6, TRUE),
    ('multivariable_calculus', 'change_of_variables', 'Change of Variables', 7, TRUE),
    ('multivariable_calculus', 'line_integrals', 'Line Integrals', 8, TRUE),
    ('multivariable_calculus', 'greens_theorem', 'Green''s Theorem', 9, TRUE),
    ('multivariable_calculus', 'stokes_divergence', 'Stokes'' & Divergence Theorem', 10, TRUE);

-- Linear Algebra (10)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('linear_algebra', 'row_reduction', 'Row Reduction', 1, TRUE),
    ('linear_algebra', 'matrix_arithmetic', 'Matrix Arithmetic', 2, TRUE),
    ('linear_algebra', 'matrix_inverses', 'Matrix Inverses', 3, TRUE),
    ('linear_algebra', 'determinants', 'Determinants', 4, TRUE),
    ('linear_algebra', 'vector_spaces', 'Vector Spaces', 5, TRUE),
    ('linear_algebra', 'subspaces', 'Subspaces', 6, TRUE),
    ('linear_algebra', 'linear_independence', 'Linear Independence', 7, TRUE),
    ('linear_algebra', 'eigenvalues', 'Eigenvalues', 8, TRUE),
    ('linear_algebra', 'diagonalization', 'Diagonalization', 9, TRUE),
    ('linear_algebra', 'linear_transformations', 'Linear Transformations', 10, TRUE);
