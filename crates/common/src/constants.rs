//! Game constants shared between frontend and backend

/// Initial ELO rating for new users in all topics
pub const INITIAL_ELO: i32 = 1500;

/// Default problem difficulty when no target ELO is specified
pub const DEFAULT_DIFFICULTY: i32 = 1500;

/// Minimum ELO rating (floor)
pub const MIN_ELO: i32 = 100;

/// Maximum ELO rating (practical ceiling)
pub const MAX_ELO: i32 = 3000;
