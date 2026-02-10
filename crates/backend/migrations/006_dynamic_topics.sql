-- Migration: Dynamic Topics System
-- Creates topics and subtopics tables to replace hardcoded MainTopic enum

-- Topics table
CREATE TABLE IF NOT EXISTS topics (
    id VARCHAR(50) PRIMARY KEY,
    display_name VARCHAR(100) NOT NULL,
    sort_order INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

-- Subtopics table
CREATE TABLE IF NOT EXISTS subtopics (
    topic_id VARCHAR(50) NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    id VARCHAR(50) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    sort_order INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (topic_id, id)
);

-- Seed existing topics
INSERT INTO topics (id, display_name, sort_order, enabled) VALUES
    ('arithmetic', 'Arithmetic', 1, TRUE),
    ('algebra1', 'Algebra 1', 2, TRUE),
    ('geometry', 'Geometry', 3, TRUE),
    ('algebra2', 'Algebra 2', 4, TRUE),
    ('precalculus', 'Precalculus', 5, TRUE),
    ('calculus', 'Calculus', 6, TRUE),
    ('multivariable_calculus', 'Multivariable Calculus', 7, TRUE),
    ('linear_algebra', 'Linear Algebra', 8, TRUE);

-- Seed Arithmetic subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('arithmetic', 'addition_subtraction', 'Addition Subtraction', 1, TRUE),
    ('arithmetic', 'multiplication_division', 'Multiplication Division', 2, TRUE),
    ('arithmetic', 'fractions', 'Fractions', 3, TRUE),
    ('arithmetic', 'decimals', 'Decimals', 4, TRUE),
    ('arithmetic', 'percentages', 'Percentages', 5, TRUE),
    ('arithmetic', 'order_of_operations', 'Order Of Operations', 6, TRUE);

-- Seed Algebra 1 subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('algebra1', 'linear_equations', 'Linear Equations', 1, TRUE),
    ('algebra1', 'inequalities', 'Inequalities', 2, TRUE),
    ('algebra1', 'graphing_lines', 'Graphing Lines', 3, TRUE),
    ('algebra1', 'systems_of_equations', 'Systems Of Equations', 4, TRUE),
    ('algebra1', 'exponents', 'Exponents', 5, TRUE),
    ('algebra1', 'polynomials', 'Polynomials', 6, TRUE),
    ('algebra1', 'factoring', 'Factoring', 7, TRUE),
    ('algebra1', 'quadratic_equations', 'Quadratic Equations', 8, TRUE);

-- Seed Geometry subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('geometry', 'angles', 'Angles', 1, TRUE),
    ('geometry', 'triangles', 'Triangles', 2, TRUE),
    ('geometry', 'circles', 'Circles', 3, TRUE),
    ('geometry', 'area_perimeter', 'Area Perimeter', 4, TRUE),
    ('geometry', 'volume_surface_area', 'Volume Surface Area', 5, TRUE),
    ('geometry', 'pythagorean_theorem', 'Pythagorean Theorem', 6, TRUE),
    ('geometry', 'trigonometry_basics', 'Trigonometry Basics', 7, TRUE);

-- Seed Algebra 2 subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('algebra2', 'complex_numbers', 'Complex Numbers', 1, TRUE),
    ('algebra2', 'rational_expressions', 'Rational Expressions', 2, TRUE),
    ('algebra2', 'radical_expressions', 'Radical Expressions', 3, TRUE),
    ('algebra2', 'exponential_functions', 'Exponential Functions', 4, TRUE),
    ('algebra2', 'logarithms', 'Logarithms', 5, TRUE),
    ('algebra2', 'sequences_series', 'Sequences Series', 6, TRUE),
    ('algebra2', 'conic_sections', 'Conic Sections', 7, TRUE);

-- Seed Precalculus subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('precalculus', 'functions', 'Functions', 1, TRUE),
    ('precalculus', 'trigonometric_functions', 'Trigonometric Functions', 2, TRUE),
    ('precalculus', 'trigonometric_identities', 'Trigonometric Identities', 3, TRUE),
    ('precalculus', 'inverse_trig', 'Inverse Trig', 4, TRUE),
    ('precalculus', 'polar_coordinates', 'Polar Coordinates', 5, TRUE),
    ('precalculus', 'vectors', 'Vectors', 6, TRUE),
    ('precalculus', 'matrices', 'Matrices', 7, TRUE);

-- Seed Calculus subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('calculus', 'limits', 'Limits', 1, TRUE),
    ('calculus', 'derivatives', 'Derivatives', 2, TRUE),
    ('calculus', 'integration', 'Integration', 3, TRUE),
    ('calculus', 'applications_of_derivatives', 'Applications Of Derivatives', 4, TRUE),
    ('calculus', 'applications_of_integration', 'Applications Of Integration', 5, TRUE);

-- Seed Multivariable Calculus subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('multivariable_calculus', 'partial_derivatives', 'Partial Derivatives', 1, TRUE),
    ('multivariable_calculus', 'multiple_integrals', 'Multiple Integrals', 2, TRUE),
    ('multivariable_calculus', 'vector_calculus', 'Vector Calculus', 3, TRUE),
    ('multivariable_calculus', 'line_integrals', 'Line Integrals', 4, TRUE),
    ('multivariable_calculus', 'surface_integrals', 'Surface Integrals', 5, TRUE);

-- Seed Linear Algebra subtopics
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
    ('linear_algebra', 'matrix_operations', 'Matrix Operations', 1, TRUE),
    ('linear_algebra', 'determinants', 'Determinants', 2, TRUE),
    ('linear_algebra', 'vector_spaces', 'Vector Spaces', 3, TRUE),
    ('linear_algebra', 'eigenvalues_eigenvectors', 'Eigenvalues Eigenvectors', 4, TRUE),
    ('linear_algebra', 'linear_transformations', 'Linear Transformations', 5, TRUE);
