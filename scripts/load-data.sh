#!/bin/bash
# Load problems and clean database

set -e

PROBLEMS_FILE="${PROBLEMS_FILE:-factory/exports/problems_import.sql}"

echo "============================================"
echo "Loading Problem Data"
echo "============================================"
echo ""

# Check if file exists
if [ ! -f "$PROBLEMS_FILE" ]; then
    echo "ERROR: Problems file not found at $PROBLEMS_FILE"
    echo "Run the factory to generate problems first."
    exit 1
fi

# Check DATABASE_URL
if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: DATABASE_URL not set"
    echo "Source .env first or set DATABASE_URL"
    exit 1
fi

# Step 1: Load problems
echo "Step 1: Loading problems from SQL file..."
echo "File: $PROBLEMS_FILE"
echo ""

psql "$DATABASE_URL" < "$PROBLEMS_FILE"

echo ""
echo "✓ Problems loaded!"
echo ""

# Step 2: Remove duplicates
echo "Step 2: Removing duplicate problems..."
echo ""

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
echo "✓ Duplicates removed!"
echo ""
echo "============================================"
echo "✓ Database setup complete!"
echo "============================================"
