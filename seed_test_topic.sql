-- Seed script: Add "Test" topic with one problem per answer type
-- Run directly against local DB: psql -U locus -d locus -f seed_test_topic.sql

BEGIN;

-- Insert the Test topic
INSERT INTO topics (id, display_name, sort_order, enabled)
VALUES ('test', 'Test', 99, TRUE)
ON CONFLICT (id) DO NOTHING;

-- Insert subtopics (one per answer type)
INSERT INTO subtopics (topic_id, id, display_name, sort_order, enabled) VALUES
  ('test', 'expressions',   'Expressions',   1,  TRUE),
  ('test', 'numerics',      'Numerics',      2,  TRUE),
  ('test', 'sets',          'Sets',          3,  TRUE),
  ('test', 'tuples',        'Tuples',        4,  TRUE),
  ('test', 'lists',         'Lists',         5,  TRUE),
  ('test', 'intervals',     'Intervals',     6,  TRUE),
  ('test', 'inequalities',  'Inequalities',  7,  TRUE),
  ('test', 'equations',     'Equations',     8,  TRUE),
  ('test', 'booleans',      'Booleans',      9,  TRUE),
  ('test', 'words',         'Words',         10, TRUE),
  ('test', 'matrices',      'Matrices',      11, TRUE),
  ('test', 'multipart',     'Multipart',     12, TRUE)
ON CONFLICT (topic_id, id) DO NOTHING;

-- Insert 12 test problems, one per answer_type
INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed) VALUES
  -- expression
  ('Simplify $(x+1)^2$',
   'x**2 + 2*x + 1',
   1200, 'test', 'expressions', 'equivalent', 'expression', 'none'),

  -- numeric
  ('What is $12 \times 13$?',
   '156',
   1000, 'test', 'numerics', 'equivalent', 'numeric', 'none'),

  -- set
  ('Solve $x^2 - 5x + 6 = 0$',
   '2, 3',
   1300, 'test', 'sets', 'equivalent', 'set', 'none'),

  -- tuple
  ('Solve the system $x + y = 5,\ x - y = 1$',
   '3, 2',
   1300, 'test', 'tuples', 'equivalent', 'tuple', 'none'),

  -- list
  ('List the roots of $x^2 - 4$ in ascending order',
   '-2, 2',
   1200, 'test', 'lists', 'equivalent', 'list', 'none'),

  -- interval
  ('Solve $1 < x \le 7$',
   'open:1,closed:7',
   1200, 'test', 'intervals', 'equivalent', 'interval', 'none'),

  -- inequality
  ('Solve $x + 4 > 0$',
   'x > -4',
   1100, 'test', 'inequalities', 'equivalent', 'inequality', 'none'),

  -- equation
  ('Write the equation of a circle centered at $(3, 2)$ with radius $3$',
   '(x - 3)**2 + (y - 2)**2 = 9',
   1400, 'test', 'equations', 'equivalent', 'equation', 'none'),

  -- boolean
  ('Is $\pi$ rational? (true/false)',
   'false',
   900, 'test', 'booleans', 'equivalent', 'boolean', 'none'),

  -- word
  ('What type of extremum does $f(x) = x^2$ have at $x = 0$?',
   'minimum',
   1000, 'test', 'words', 'equivalent', 'word', 'none'),

  -- matrix
  ('Compute $I_2 \cdot \begin{bmatrix} 3 & 4 \\ 5 & 6 \end{bmatrix}$',
   '[[3, 4], [5, 6]]',
   1300, 'test', 'matrices', 'equivalent', 'matrix', 'none'),

  -- multi_part
  ('Find the center and radius of $(x - 5)^2 + (y + 4)^2 = 16$',
   'tuple:5,-4|||numeric:4',
   1400, 'test', 'multipart', 'equivalent', 'multi_part', 'none');

COMMIT;
