//! Game constants shared between frontend and backend

/// Initial ELO rating for new users in all topics
pub const INITIAL_ELO: i32 = 1500;

/// Default problem difficulty when no target ELO is specified
pub const DEFAULT_DIFFICULTY: i32 = 1500;

/// Minimum ELO rating (floor)
pub const MIN_ELO: i32 = 100;

/// Maximum ELO rating (practical ceiling)
pub const MAX_ELO: i32 = 5000;

/// Default number of problems to fetch per batch
pub const PROBLEM_BATCH_SIZE: u32 = 30;

/// Maximum problems allowed in a single batch request
pub const PROBLEM_BATCH_MAX: u32 = 50;

/// Refill the problem queue when it drops to this many remaining
pub const PROBLEM_QUEUE_REFILL_THRESHOLD: usize = 5;

/// Number of practice problems in ranked warmup
pub const WARMUP_SIZE: usize = 5;
