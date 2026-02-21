//! ELO rating calculation
//!
//! Shared ELO logic used by both frontend (for prediction) and backend (for authoritative updates).

/// K-factor for rating changes
const K_FACTOR: f64 = 32.0;

/// Calculate expected score for player A against player B
fn expected_score(rating_a: i32, rating_b: i32) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf((rating_b - rating_a) as f64 / 400.0))
}

/// Calculate time multiplier for ELO changes based on solve speed
///
/// Faster solving = higher multiplier (up to 1.5x)
/// Slower solving = normal multiplier (1.0x, unlimited time allowed)
///
/// # Arguments
/// * `time_ms` - Time taken to solve in milliseconds
/// * `difficulty` - Problem difficulty (ELO rating)
/// * `time_limit_seconds` - Optional per-problem time limit in seconds
///
/// # Returns
/// Multiplier to apply to K-factor (1.0 to 1.5)
pub fn time_multiplier(time_ms: i32, difficulty: i32, time_limit_seconds: Option<i32>) -> f64 {
    // Use per-problem time limit if set, otherwise fall back to difficulty tiers
    let expected_time_ms = if let Some(limit) = time_limit_seconds {
        (limit as f64) * 1000.0
    } else {
        match difficulty {
            0..=1200 => 30_000.0,      // 30 seconds for easy problems
            1201..=1400 => 60_000.0,   // 1 minute for medium problems
            1401..=1600 => 120_000.0,  // 2 minutes for hard problems
            _ => 180_000.0,            // 3 minutes for expert problems
        }
    };

    // Calculate ratio: actual_time / expected_time
    let time_ratio = (time_ms as f64) / expected_time_ms;

    // Only apply bonus for fast solves, no penalty for slow solves
    // Very fast (≤0.5x expected) → 1.5x multiplier (maximum bonus)
    // Expected (1.0x) → 1.0x multiplier
    // Slow (>1.0x expected) → 1.0x multiplier (no penalty, unlimited time)
    if time_ratio <= 0.5 {
        // Maximum bonus for blazing fast solves
        1.5
    } else if time_ratio < 1.0 {
        // Linear bonus from 1.5x at 0.5 to 1.0x at 1.0
        // Formula: 2.0 - time_ratio
        2.0 - time_ratio
    } else {
        // No penalty for slow solving - always 1.0x
        1.0
    }
}

/// Calculate new ELO rating after a match
///
/// # Arguments
/// * `player_elo` - Current player ELO
/// * `problem_difficulty` - Problem difficulty (treated as opponent ELO)
/// * `won` - Whether the player solved the problem correctly
/// * `time_taken_ms` - Optional time taken in milliseconds (for time bonus)
///
/// # Returns
/// The new ELO rating
pub fn calculate_new_elo(
    player_elo: i32,
    problem_difficulty: i32,
    won: bool,
    time_taken_ms: Option<i32>,
    time_limit_seconds: Option<i32>,
) -> i32 {
    let expected = expected_score(player_elo, problem_difficulty);
    let actual = if won { 1.0 } else { 0.0 };

    // Apply time multiplier if time data is available
    let k_factor = if let Some(time_ms) = time_taken_ms {
        K_FACTOR * time_multiplier(time_ms, problem_difficulty, time_limit_seconds)
    } else {
        K_FACTOR  // No bonus if time not tracked
    };

    let change = k_factor * (actual - expected);

    (player_elo as f64 + change).round() as i32
}

/// Calculate ELO change (for display purposes)
pub fn calculate_elo_change(
    player_elo: i32,
    problem_difficulty: i32,
    won: bool,
    time_taken_ms: Option<i32>,
    time_limit_seconds: Option<i32>,
) -> i32 {
    calculate_new_elo(player_elo, problem_difficulty, won, time_taken_ms, time_limit_seconds) - player_elo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elo_win_against_harder() {
        // Winning against harder problem gives more points
        let change = calculate_elo_change(1500, 1700, true, None, None);
        assert!(change > 16, "Expected significant gain, got {}", change);
    }

    #[test]
    fn test_elo_win_against_easier() {
        // Winning against easier problem gives fewer points
        let change = calculate_elo_change(1500, 1300, true, None, None);
        assert!(change < 16, "Expected small gain, got {}", change);
        assert!(change > 0, "Expected positive change, got {}", change);
    }

    #[test]
    fn test_elo_loss_against_harder() {
        // Losing against harder problem costs fewer points
        let change = calculate_elo_change(1500, 1700, false, None, None);
        assert!(change > -16, "Expected small loss, got {}", change);
        assert!(change < 0, "Expected negative change, got {}", change);
    }

    #[test]
    fn test_elo_loss_against_easier() {
        // Losing against easier problem costs more points
        let change = calculate_elo_change(1500, 1300, false, None, None);
        assert!(change < -16, "Expected significant loss, got {}", change);
    }

    #[test]
    fn test_time_multiplier_fast_solve() {
        // Solve in half expected time → 1.5x multiplier
        let mult = time_multiplier(15_000, 1000, None);
        assert!((mult - 1.5).abs() < 0.01, "Expected 1.5x multiplier for fast solve, got {}", mult);
    }

    #[test]
    fn test_time_multiplier_expected_time() {
        // Solve in expected time → 1.0x multiplier
        let mult = time_multiplier(30_000, 1000, None);
        assert!((mult - 1.0).abs() < 0.01, "Expected 1.0x multiplier for expected time, got {}", mult);
    }

    #[test]
    fn test_time_multiplier_slow_solve() {
        // Solve in 2x expected time → 1.0x multiplier (no penalty)
        let mult = time_multiplier(60_000, 1000, None);
        assert!((mult - 1.0).abs() < 0.01, "Expected 1.0x multiplier for slow solve (no penalty), got {}", mult);
    }

    #[test]
    fn test_time_multiplier_very_slow_solve() {
        // Solve in 10x expected time → 1.0x multiplier (no penalty)
        let mult = time_multiplier(300_000, 1000, None);
        assert!((mult - 1.0).abs() < 0.01, "Expected 1.0x multiplier for very slow solve (no penalty), got {}", mult);
    }

    #[test]
    fn test_calculate_new_elo_with_time_bonus() {
        // Fast solve should give more points
        let elo_with_bonus = calculate_new_elo(1500, 1500, true, Some(15_000), None);
        let elo_no_bonus = calculate_new_elo(1500, 1500, true, None, None);
        assert!(elo_with_bonus > elo_no_bonus, "Fast solve should give more ELO");
    }

    #[test]
    fn test_calculate_new_elo_slow_solve_no_penalty() {
        // Slow solve should not be penalized
        let elo_slow = calculate_new_elo(1500, 1500, true, Some(120_000), None);
        let elo_no_time = calculate_new_elo(1500, 1500, true, None, None);
        assert!((elo_slow - elo_no_time).abs() < 1, "Slow solve should not be penalized");
    }

    #[test]
    fn test_time_multiplier_custom_limit() {
        // With a 60s custom limit, solving in 30s (half) should give 1.5x
        let mult = time_multiplier(30_000, 1000, Some(60));
        assert!((mult - 1.5).abs() < 0.01, "Expected 1.5x for half custom limit, got {}", mult);

        // Solving in 60s (full) should give 1.0x
        let mult = time_multiplier(60_000, 1000, Some(60));
        assert!((mult - 1.0).abs() < 0.01, "Expected 1.0x at custom limit, got {}", mult);

        // Custom limit overrides difficulty-based default
        // difficulty 1000 normally has 30s expected, but custom 120s changes it
        let mult = time_multiplier(60_000, 1000, Some(120));
        assert!(mult > 1.0, "60s solve with 120s limit should get bonus, got {}", mult);
    }
}
