#!/bin/bash
# Load 5,376 production problems into database

set -e

PROBLEMS_FILE="${PROBLEMS_FILE:-factory/backend/exports/problems_import.sql}"

if [ ! -f "$PROBLEMS_FILE" ]; then
    echo "ERROR: Problems file not found at $PROBLEMS_FILE"
    echo "Run the factory to generate problems first."
    exit 1
fi

echo "Loading 5,376 problems into database..."
echo "File: $PROBLEMS_FILE"
echo ""

# Check if using Docker or direct PostgreSQL
if [ -n "$USE_DOCKER" ]; then
    # Load into Docker container
    CONTAINER="${CONTAINER:-locus-db}"
    echo "Loading into Docker container: $CONTAINER"
    docker exec -i $CONTAINER psql -U locus -d locus < "$PROBLEMS_FILE"
else
    # Load using DATABASE_URL
    if [ -z "$DATABASE_URL" ]; then
        echo "ERROR: DATABASE_URL not set"
        echo "Set it or use: USE_DOCKER=1 ./load-problems.sh"
        exit 1
    fi
    echo "Loading via DATABASE_URL"
    psql "$DATABASE_URL" < "$PROBLEMS_FILE"
fi

echo ""
echo "SUCCESS: Problems loaded!"
echo "Verify with: SELECT COUNT(*) FROM problems;"
