#!/bin/bash
# Remove duplicate problems from database

set -e

if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: DATABASE_URL not set"
    echo "Usage: DATABASE_URL='postgresql://...' ./remove-duplicates.sh"
    exit 1
fi

echo "Removing duplicate problems..."

psql "$DATABASE_URL" << 'EOF'
-- Show duplicates before deletion
SELECT question_latex, COUNT(*) as count
FROM problems
GROUP BY question_latex
HAVING COUNT(*) > 1
ORDER BY count DESC
LIMIT 10;

-- Delete duplicates (keep the one with lowest id)
DELETE FROM problems a USING problems b
WHERE a.id > b.id AND a.question_latex = b.question_latex;

-- Show final count
SELECT COUNT(*) as total_problems, COUNT(DISTINCT question_latex) as unique_questions FROM problems;
EOF

echo ""
echo "SUCCESS: Duplicates removed!"
