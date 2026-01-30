//! ELO rating calculation

/// K-factor for rating changes
const K_FACTOR: f64 = 32.0;

/// Calculate expected score for player A against player B
fn expected_score(rating_a: i32, rating_b: i32) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf((rating_b - rating_a) as f64 / 400.0))
}

/// Calculate new ELO rating after a match
///
/// # Arguments
/// * `player_elo` - Current player ELO
/// * `problem_difficulty` - Problem difficulty (treated as opponent ELO)
/// * `won` - Whether the player solved the problem correctly
///
/// # Returns
/// The new ELO rating
pub fn calculate_new_elo(player_elo: i32, problem_difficulty: i32, won: bool) -> i32 {
    let expected = expected_score(player_elo, problem_difficulty);
    let actual = if won { 1.0 } else { 0.0 };
    let change = K_FACTOR * (actual - expected);

    (player_elo as f64 + change).round() as i32
}

/// Calculate ELO change (for display purposes)
pub fn calculate_elo_change(player_elo: i32, problem_difficulty: i32, won: bool) -> i32 {
    calculate_new_elo(player_elo, problem_difficulty, won) - player_elo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elo_win_against_harder() {
        // Winning against harder problem gives more points
        let change = calculate_elo_change(1500, 1700, true);
        assert!(change > 16, "Expected significant gain, got {}", change);
    }

    #[test]
    fn test_elo_win_against_easier() {
        // Winning against easier problem gives fewer points
        let change = calculate_elo_change(1500, 1300, true);
        assert!(change < 16, "Expected small gain, got {}", change);
        assert!(change > 0, "Expected positive change, got {}", change);
    }

    #[test]
    fn test_elo_loss_against_harder() {
        // Losing against harder problem costs fewer points
        let change = calculate_elo_change(1500, 1700, false);
        assert!(change > -16, "Expected small loss, got {}", change);
        assert!(change < 0, "Expected negative change, got {}", change);
    }

    #[test]
    fn test_elo_loss_against_easier() {
        // Losing against easier problem costs more points
        let change = calculate_elo_change(1500, 1300, false);
        assert!(change < -16, "Expected significant loss, got {}", change);
    }
}
