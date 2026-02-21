-- Add answer_type column to problems table
ALTER TABLE problems ADD COLUMN answer_type VARCHAR(20) NOT NULL DEFAULT 'expression';

ALTER TABLE problems ADD CONSTRAINT check_answer_type
  CHECK (answer_type IN ('expression','numeric','set','tuple','list',
    'interval','inequality','equation','boolean','word','matrix','multi_part'));

CREATE INDEX idx_problems_answer_type ON problems(answer_type);
