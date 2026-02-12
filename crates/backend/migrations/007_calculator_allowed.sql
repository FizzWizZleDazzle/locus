-- Add calculator level enum column to problems table
-- This allows the AI to restrict which calculators can be used for each problem

-- Add the column with a default value
ALTER TABLE problems
ADD COLUMN calculator_allowed VARCHAR(20) NOT NULL DEFAULT 'none';

-- Add check constraint to enforce valid values
-- Values form a hierarchy: none < scientific < graphing < cas
ALTER TABLE problems
ADD CONSTRAINT check_calculator_allowed
CHECK (calculator_allowed IN ('none', 'scientific', 'graphing', 'cas'));

-- Add index for filtering by calculator level
CREATE INDEX idx_problems_calculator_allowed
ON problems (calculator_allowed);

-- Add comment for documentation
COMMENT ON COLUMN problems.calculator_allowed IS 'Maximum calculator level allowed: none < scientific < graphing < cas. Higher levels implicitly include all lower capabilities.';
