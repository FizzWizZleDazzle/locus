-- Fix broken answer key formats from factory Python/SymPy notation

-- 1. Fix set answers with Python list notation: {['r = -2', 'r = -1 + 5i', ...]}
-- Convert to: {-2, -1+5*I, -1-5*I} (SymEngine format)
UPDATE problems
SET answer_key = regexp_replace(
    regexp_replace(
        regexp_replace(
            regexp_replace(
                regexp_replace(
                    answer_key,
                    E'\\{\\[''', '{', 'g'  -- Strip {[' → {
                ),
                E'''\\]\\}', '}', 'g'      -- Strip ']} → }
            ),
            E''',\\s*''', ', ', 'g'        -- Strip ', ' between quoted items
        ),
        E'[a-z]\\s*=\\s*', '', 'g'        -- Strip "r = " or "x = " prefixes
    ),
    E'([0-9])i\\b', E'\\1*I', 'g'         -- Convert lowercase i to SymEngine I
)
WHERE answer_type = 'set'
  AND answer_key LIKE E'%[''%';

-- Also handle remaining j/i complex notation in sets
UPDATE problems
SET answer_key = regexp_replace(
    answer_key,
    E'([0-9])i\\b', E'\\1*I', 'g'
)
WHERE answer_type = 'set'
  AND answer_key ~ E'[0-9]i\\b';

-- 2. Delete broken list-of-Matrix problems (can't be graded correctly)
DELETE FROM problems
WHERE answer_type = 'list'
  AND answer_key LIKE '%Matrix(%';
