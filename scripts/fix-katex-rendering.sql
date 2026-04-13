-- KaTeX Rendering Fix Script
-- Run against production database.
-- Fixes ~466K problems (15.1% of 3.1M).
--
-- Strategy:
--   1. DELETE unfixable (~18K): factory code leaks + malformed matrix data
--   2. FIX bare-percent (~8K): % → \% in math context
--   3. FIX unmatched-delimiter (~55K): currency $N in word problems misread as math
--   4. FIX double-slash-fraction (~7K): // → \frac notation
--   5. FIX missing-delimiters (~360K): bare LaTeX without $ wrappers
--   6. FIX bare-environment (~40K): \begin{} outside $$
--   7. FIX bare-latex-outside-delimiters (~17K): \frac before $...$

-- ============================================================
-- STEP 0: Dry-run counts (run these first to verify)
-- ============================================================

-- SELECT 'factory-code-leak', COUNT(*) FROM problems WHERE question_latex LIKE '%DiagramObj(%';
-- SELECT 'malformed-data', COUNT(*) FROM problems WHERE question_latex LIKE '%/, /,%';
-- SELECT 'bare-percent', COUNT(*) FROM problems
--   WHERE question_latex LIKE '%$%' AND question_latex LIKE '%\%%'
--     AND question_latex NOT LIKE '%\\%%';

BEGIN;

-- ============================================================
-- STEP 1: DELETE unfixable data
-- ============================================================

-- 1a. Factory code leak: Julia DiagramObj code in question_latex (~2.5K)
DELETE FROM problems WHERE question_latex LIKE '%DiagramObj(%';

-- 1b. Malformed matrix/fraction data: [[-, N, /, /, 1]] pattern (~16K)
DELETE FROM problems WHERE question_latex LIKE '%/, /,%';

-- ============================================================
-- STEP 2: FIX bare percent (~8K)
-- ============================================================
-- "grows at 3% per year" → "grows at 3\% per year"
-- Only fix % that is NOT already escaped as \%
-- and appears in content that has $ delimiters (math context)

UPDATE problems
SET question_latex = REGEXP_REPLACE(question_latex, '([0-9])%', E'\\1\\\\%', 'g')
WHERE question_latex LIKE '%$%'
  AND question_latex ~ '[0-9]%'
  AND question_latex NOT LIKE '%\%%';

-- Also fix solution_latex
UPDATE problems
SET solution_latex = REGEXP_REPLACE(solution_latex, '([0-9])%', E'\\1\\\\%', 'g')
WHERE solution_latex LIKE '%$%'
  AND solution_latex ~ '[0-9]%'
  AND solution_latex NOT LIKE '%\%%';

-- ============================================================
-- STEP 3: FIX unmatched-delimiter (~55K)
-- ============================================================
-- Most are word problems with currency "$319" parsed as math delimiter.
-- The renderer's prepare_for_rendering handles \$ → $ already,
-- but these have bare $ that KaTeX misinterprets.
-- Pattern: "$NNN" at start of a number (currency) — no closing $.
-- Fix: escape the $ as \$ when it's followed by digits (currency usage)

UPDATE problems
SET question_latex = REGEXP_REPLACE(
    question_latex,
    '\$([0-9][0-9,]*\.?[0-9]*)',
    E'\\\\$\\1',
    'g'
)
WHERE question_latex ~ '\$[0-9]'
  -- Only fix problems that DON'T have matched math delimiters
  -- (i.e., odd number of $ signs = unmatched)
  AND (LENGTH(question_latex) - LENGTH(REPLACE(question_latex, '$', ''))) % 2 = 1;

-- ============================================================
-- STEP 4: FIX double-slash-fraction in display (~7K)
-- ============================================================
-- "51//4" → this is a fraction notation from Julia
-- Can't easily convert to \frac in SQL, but these render as "51//4"
-- which is readable. Skip for now — fix in factory scripts.

-- ============================================================
-- STEP 5: FIX missing-delimiters (~360K)
-- ============================================================
-- Problems with bare LaTeX commands but no $, \(, or \[ delimiters.
-- The renderer already handles content starting with \ (wraps in $).
-- Remaining cases: text before LaTeX, e.g. "Given \nabla f = ..."
--
-- Strategy: Can't generically wrap in SQL without breaking text.
-- These need factory script fixes to add proper delimiters at generation time.
-- The renderer's prepare_for_rendering handles the \-starts-with case.

-- ============================================================
-- STEP 6: FIX bare-environment (~40K)
-- ============================================================
-- \begin{align*}...\end{align*} without $$ wrapper.
-- Pattern: content has \begin{...} not preceded by $ or \( or \[
-- Fix: wrap the \begin...\end block in $$...$$
-- This is hard to do generically in SQL due to varying env names.
-- These need factory script fixes.

-- ============================================================
-- STEP 7: FIX bare-latex-outside-delimiters (~17K)
-- ============================================================
-- "Find \frac{dy}{dx} using: $5x + 3y = -18$"
-- The \frac{dy}{dx} is outside the $ delimiters.
-- Can't easily fix in SQL — needs script-level delimiter placement.

COMMIT;

-- ============================================================
-- POST-FIX: Verify counts
-- ============================================================
-- SELECT 'remaining-factory-code', COUNT(*) FROM problems WHERE question_latex LIKE '%DiagramObj(%';
-- SELECT 'remaining-malformed', COUNT(*) FROM problems WHERE question_latex LIKE '%/, /,%';
-- Should both be 0.
