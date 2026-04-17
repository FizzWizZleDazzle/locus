-- Physics learning platform tables
-- Separate from math tables to avoid coupling

-- ============================================================================
-- Topic catalogue
-- ============================================================================

CREATE TABLE physics_topics (
    id VARCHAR(50) PRIMARY KEY,
    display_name VARCHAR(100) NOT NULL,
    sort_order INT NOT NULL,
    enabled BOOLEAN DEFAULT TRUE
);

CREATE TABLE physics_subtopics (
    topic_id VARCHAR(50) REFERENCES physics_topics(id) ON DELETE CASCADE,
    id VARCHAR(50),
    display_name VARCHAR(100) NOT NULL,
    sort_order INT NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    PRIMARY KEY (topic_id, id)
);

-- Seed initial topics
INSERT INTO physics_topics VALUES
    ('mechanics', 'Mechanics', 1, true),
    ('waves', 'Waves & Oscillations', 2, true),
    ('electricity', 'Electricity & Circuits', 3, false);

INSERT INTO physics_subtopics VALUES
    ('mechanics', 'projectile_motion', 'Projectile Motion', 1, true),
    ('mechanics', 'inclined_plane', 'Inclined Planes', 2, true),
    ('mechanics', 'collisions', 'Collisions', 3, true),
    ('mechanics', 'springs', 'Springs & Hooke''s Law', 4, true),
    ('mechanics', 'friction', 'Friction', 5, true),
    ('mechanics', 'circular_motion', 'Circular Motion', 6, true),
    ('waves', 'pendulum', 'Pendulums', 1, true),
    ('waves', 'spring_oscillation', 'Spring Oscillation', 2, true);

-- ============================================================================
-- Physics problems
-- ============================================================================

CREATE TABLE physics_problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL DEFAULT '',
    description_latex TEXT NOT NULL,
    difficulty INT NOT NULL,
    physics_topic VARCHAR(50) NOT NULL,
    physics_subtopic VARCHAR(50) NOT NULL,

    -- Rapier2D scene definition (bodies, forces, constraints, materials)
    scene_definition JSONB NOT NULL,

    -- Adjustable parameters (sliders exposed to the student)
    parameters JSONB NOT NULL DEFAULT '[]',

    -- Interactive challenge stages (Predict-Test-Reflect flow)
    challenge_stages JSONB NOT NULL DEFAULT '[]',

    -- Post-solve exploration prompts
    what_if_prompts JSONB NOT NULL DEFAULT '[]',

    -- Common errors catalogue for the Reflection stage
    common_errors JSONB NOT NULL DEFAULT '[]',

    -- Numeric answer specification with tolerances
    answer_spec JSONB NOT NULL,

    -- Optional diagram image
    question_image TEXT NOT NULL DEFAULT '',

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_physics_problems_topic ON physics_problems(physics_topic);
CREATE INDEX idx_physics_problems_subtopic ON physics_problems(physics_topic, physics_subtopic);
CREATE INDEX idx_physics_problems_difficulty ON physics_problems(difficulty);

-- ============================================================================
-- Physics attempts (tracks HOW the student solved, not just IF)
-- ============================================================================

CREATE TABLE physics_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    problem_id UUID REFERENCES physics_problems(id) ON DELETE CASCADE NOT NULL,

    -- What the student submitted
    user_answers JSONB NOT NULL,
    is_correct BOOLEAN NOT NULL,

    -- Process metrics (used for scoring)
    hints_used INT NOT NULL DEFAULT 0,
    fbd_attempts INT NOT NULL DEFAULT 0,
    prediction_accuracy FLOAT,
    stages_completed INT NOT NULL DEFAULT 0,
    what_ifs_explored INT NOT NULL DEFAULT 0,
    errors_identified JSONB DEFAULT '[]',

    -- Score breakdown
    score_correctness INT NOT NULL DEFAULT 0,
    score_process INT NOT NULL DEFAULT 0,
    score_prediction INT NOT NULL DEFAULT 0,
    score_independence INT NOT NULL DEFAULT 0,
    score_exploration INT NOT NULL DEFAULT 0,

    -- Session metadata
    parameters_used JSONB,
    time_taken_ms INT,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_physics_attempts_user ON physics_attempts(user_id, created_at DESC);
CREATE INDEX idx_physics_attempts_problem ON physics_attempts(problem_id);

-- ============================================================================
-- Per-topic progress (mastery-based, no ELO for physics)
-- ============================================================================

CREATE TABLE physics_user_progress (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    topic VARCHAR(50) NOT NULL,
    problems_attempted INT DEFAULT 0,
    problems_solved INT DEFAULT 0,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, topic)
);
