# Topic Refactoring Status

## Completed ✓

1. **Database Schema** - Dynamic ELO table
   - Created `user_topic_elo` table (one row per user per topic)
   - PostgreSQL functions: `get_user_elo()`, `update_user_elo()`
   - Updated `problems` table: `main_topic`, `subtopic` columns
   - Migration: `003_topic_elo_table.sql`

2. **Common Types** - Topic enums and structures
   - `MainTopic` enum with 8 subjects (Arithmetic → Linear Algebra)
   - Each topic has subtopics (e.g., Algebra 1: factoring, quadratic equations, etc.)
   - Updated `UserProfile` with 8 ELO fields
   - Updated `ProblemResponse` with `main_topic` and `subtopic`

3. **Backend Models** - Dynamic queries
   - `User::get_elo_for_topic()` - uses PostgreSQL function
   - `User::update_elo_for_topic()` - uses PostgreSQL function
   - `User::get_all_elos()` - returns HashMap
   - `User::leaderboard()` - per-topic leaderboard
   - `Problem::get_random()` - filters by main_topic + subtopics array

4. **Frontend Components**
   - `TopicSelector` component - two-step selection (subject → subtopics)

## In Progress ⏳

**API Handlers** - Need to update:
- `auth.rs` - `to_profile()` is now async
- `problems.rs` - parse subtopics from query param
- `submit.rs` - update correct topic ELO
- `leaderboard.rs` - filter by topic

## TODO

- Frontend Practice page - add TopicSelector
- Frontend Ranked page - add TopicSelector
- Frontend Leaderboard - topic dropdown
- Update seed data with new schema

