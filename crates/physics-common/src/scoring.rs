//! Scoring model for physics problem attempts.
//!
//! The scoring rewards the *process* of solving, not just the final number.
//! A student who builds a perfect FBD and equation but makes an arithmetic
//! error scores higher than one who guesses the right answer without
//! understanding.

use serde::{Deserialize, Serialize};

/// Breakdown of how a physics attempt is scored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptScore {
    /// 0-40: Did the student produce the correct final answer(s)?
    pub correctness: u32,
    /// 0-30: Process quality — FBD on first try, equation correct, etc.
    pub process: u32,
    /// 0-15: How close was their prediction to the actual result?
    pub prediction_accuracy: u32,
    /// 0-15: Fewer hints = more independence.
    pub independence: u32,
    /// Bonus (tracked separately, not in total): +5 per "What if?" explored.
    pub exploration_bonus: u32,
}

impl AttemptScore {
    /// Total score out of 100 (exploration bonus is separate).
    pub fn total(&self) -> u32 {
        self.correctness + self.process + self.prediction_accuracy + self.independence
    }

    /// Compute the score from raw attempt metrics.
    pub fn compute(
        is_correct: bool,
        parts_correct: usize,
        parts_total: usize,
        fbd_attempts: i32,
        stages_completed: i32,
        total_stages: i32,
        prediction_error_pct: Option<f64>,
        hints_used: i32,
        what_ifs_explored: i32,
    ) -> Self {
        // -- Correctness (0-40) --
        let correctness = if parts_total == 0 {
            0
        } else if is_correct {
            40
        } else {
            // Partial credit per correct part.
            ((parts_correct as f64 / parts_total as f64) * 30.0) as u32
        };

        // -- Process (0-30) --
        let fbd_score = match fbd_attempts {
            0 => 0,  // didn't attempt
            1 => 15, // first try
            2 => 10,
            3 => 5,
            _ => 2,
        };
        let stage_completion_ratio = if total_stages == 0 {
            0.0
        } else {
            stages_completed as f64 / total_stages as f64
        };
        let stage_score = (stage_completion_ratio * 15.0) as u32;
        let process = (fbd_score + stage_score).min(30);

        // -- Prediction accuracy (0-15) --
        let prediction_accuracy = match prediction_error_pct {
            None => 0,
            Some(err) if err <= 2.0 => 15,
            Some(err) if err <= 5.0 => 12,
            Some(err) if err <= 10.0 => 8,
            Some(err) if err <= 25.0 => 4,
            Some(_) => 1,
        };

        // -- Independence (0-15) --
        let independence = match hints_used {
            0 => 15,
            1 => 10,
            2 => 6,
            3 => 3,
            _ => 0,
        };

        // -- Exploration bonus --
        let exploration_bonus = (what_ifs_explored as u32).saturating_mul(5);

        Self {
            correctness,
            process,
            prediction_accuracy,
            independence,
            exploration_bonus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_attempt() {
        let score = AttemptScore::compute(
            true,  // is_correct
            2,     // parts_correct
            2,     // parts_total
            1,     // fbd_attempts (first try)
            5,     // stages_completed
            5,     // total_stages
            Some(1.5), // prediction within 2%
            0,     // no hints
            3,     // explored 3 what-ifs
        );
        assert_eq!(score.correctness, 40);
        assert_eq!(score.process, 30);
        assert_eq!(score.prediction_accuracy, 15);
        assert_eq!(score.independence, 15);
        assert_eq!(score.total(), 100);
        assert_eq!(score.exploration_bonus, 15);
    }

    #[test]
    fn wrong_answer_good_process() {
        let score = AttemptScore::compute(
            false, 1, 2,   // got 1/2 parts right
            1,             // FBD on first try
            5, 5,          // all stages
            Some(8.0),     // 8% prediction error
            1,             // 1 hint
            0,
        );
        assert_eq!(score.correctness, 15); // partial: 1/2 * 30
        assert_eq!(score.process, 30);
        assert_eq!(score.prediction_accuracy, 8);
        assert_eq!(score.independence, 10);
        // Process matters more than guessing
        assert!(score.total() > 50);
    }

    #[test]
    fn lucky_guess_bad_process() {
        let score = AttemptScore::compute(
            true, 1, 1,
            0,         // never attempted FBD
            1, 5,      // only completed 1/5 stages
            None,      // no prediction
            4,         // 4 hints
            0,
        );
        assert_eq!(score.correctness, 40);
        assert_eq!(score.process, 3); // 0 from FBD + 3 from 1/5 stages
        assert_eq!(score.prediction_accuracy, 0);
        assert_eq!(score.independence, 0);
        assert!(score.total() < 50); // correct answer but bad process
    }
}
