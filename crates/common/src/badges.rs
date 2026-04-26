//! Badge definitions — computed dynamically from user stats, no DB tables needed.

use serde::{Deserialize, Serialize};

use crate::TopicStatsEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeCategory {
    Streak,
    Elo,
    Problems,
    TopicMastery,
    DailyPuzzle,
    Fun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeTier {
    Bronze,
    Silver,
    Gold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarnedBadge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: BadgeCategory,
    pub tier: BadgeTier,
}

/// Badge with earned status — used for displaying all badges (earned + locked).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeDisplay {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: BadgeCategory,
    pub tier: BadgeTier,
    pub earned: bool,
}

struct BadgeDef {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    category: BadgeCategory,
    tier: BadgeTier,
}

const BADGES: &[BadgeDef] = &[
    // Streak badges
    BadgeDef {
        id: "streak_3",
        name: "Getting Started",
        description: "3-day streak",
        category: BadgeCategory::Streak,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "streak_7",
        name: "Week Warrior",
        description: "7-day streak",
        category: BadgeCategory::Streak,
        tier: BadgeTier::Silver,
    },
    BadgeDef {
        id: "streak_30",
        name: "Monthly Master",
        description: "30-day streak",
        category: BadgeCategory::Streak,
        tier: BadgeTier::Gold,
    },
    // ELO badges
    BadgeDef {
        id: "elo_1600",
        name: "Rising Star",
        description: "Reached 1600 ELO in any topic",
        category: BadgeCategory::Elo,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "elo_1800",
        name: "Skilled",
        description: "Reached 1800 ELO in any topic",
        category: BadgeCategory::Elo,
        tier: BadgeTier::Silver,
    },
    BadgeDef {
        id: "elo_2000",
        name: "Expert",
        description: "Reached 2000 ELO in any topic",
        category: BadgeCategory::Elo,
        tier: BadgeTier::Gold,
    },
    // Problems solved badges
    BadgeDef {
        id: "solved_50",
        name: "Problem Solver",
        description: "Solved 50 problems correctly",
        category: BadgeCategory::Problems,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "solved_250",
        name: "Dedicated",
        description: "Solved 250 problems correctly",
        category: BadgeCategory::Problems,
        tier: BadgeTier::Silver,
    },
    BadgeDef {
        id: "solved_1000",
        name: "Thousand Club",
        description: "Solved 1000 problems correctly",
        category: BadgeCategory::Problems,
        tier: BadgeTier::Gold,
    },
    // Topic mastery badges
    BadgeDef {
        id: "topics_3",
        name: "Explorer",
        description: "Reached 1600+ ELO in 3 topics",
        category: BadgeCategory::TopicMastery,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "topics_5",
        name: "Polymath",
        description: "Reached 1600+ ELO in 5 topics",
        category: BadgeCategory::TopicMastery,
        tier: BadgeTier::Silver,
    },
    BadgeDef {
        id: "topics_8",
        name: "Renaissance",
        description: "Reached 1600+ ELO in 8 topics",
        category: BadgeCategory::TopicMastery,
        tier: BadgeTier::Gold,
    },
    // Daily puzzle badges
    BadgeDef {
        id: "daily_3",
        name: "Daily Dabbler",
        description: "3-day daily puzzle streak",
        category: BadgeCategory::DailyPuzzle,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "daily_7",
        name: "Daily Regular",
        description: "7-day daily puzzle streak",
        category: BadgeCategory::DailyPuzzle,
        tier: BadgeTier::Silver,
    },
    BadgeDef {
        id: "daily_30",
        name: "Daily Devotee",
        description: "30-day daily puzzle streak",
        category: BadgeCategory::DailyPuzzle,
        tier: BadgeTier::Gold,
    },
    // Extra ELO milestones
    BadgeDef {
        id: "elo_2500",
        name: "Competitor",
        description: "Reached 2500 ELO in any topic",
        category: BadgeCategory::Elo,
        tier: BadgeTier::Gold,
    },
    BadgeDef {
        id: "solved_5000",
        name: "Veteran",
        description: "Solved 5000 problems correctly",
        category: BadgeCategory::Problems,
        tier: BadgeTier::Gold,
    },
    BadgeDef {
        id: "streak_100",
        name: "Unstoppable",
        description: "100-day streak",
        category: BadgeCategory::Streak,
        tier: BadgeTier::Gold,
    },
    // Fun / novelty badges
    BadgeDef {
        id: "first_blood",
        name: "First Blood",
        description: "Solve your very first problem",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "try_hard",
        name: "Try Hard",
        description: "500 submissions, right or wrong",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "perfectionist",
        name: "Perfectionist",
        description: "Flawless in a topic (20+ problems)",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "sharpshooter",
        name: "Sharpshooter",
        description: "90%+ overall accuracy (100+ attempts)",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "one_trick",
        name: "One Trick Pony",
        description: "80%+ of solves in one topic",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "lucky",
        name: "Lucky Seven",
        description: "Reach 777 correct answers",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "stubborn",
        name: "Stubborn",
        description: "50+ wrong answers in a single topic",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "unbreakable",
        name: "Unbreakable",
        description: "20 correct in a row in one topic",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "the_wall",
        name: "The Wall",
        description: "50 correct in a row in one topic",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "triple_threat",
        name: "Triple Threat",
        description: "90%+ accuracy in 3 topics (20+ each)",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
    BadgeDef {
        id: "glass_cannon",
        name: "Glass Cannon",
        description: "High accuracy, few attempts, high ELO",
        category: BadgeCategory::Fun,
        tier: BadgeTier::Bronze,
    },
];

impl BadgeDef {
    fn to_earned(&self) -> EarnedBadge {
        EarnedBadge {
            id: self.id.to_string(),
            name: self.name.to_string(),
            description: self.description.to_string(),
            category: self.category,
            tier: self.tier,
        }
    }

    fn to_display(&self, earned: bool) -> BadgeDisplay {
        BadgeDisplay {
            id: self.id.to_string(),
            name: self.name.to_string(),
            description: self.description.to_string(),
            category: self.category,
            tier: self.tier,
            earned,
        }
    }
}

/// Pre-compute derived stats from topic data for badge qualification checks.
struct DerivedStats {
    max_elo: i32,
    topics_above_1600: usize,
    has_perfectionist: bool,
    has_one_trick: bool,
    has_stubborn: bool,
    has_unbreakable: bool,
    has_the_wall: bool,
    high_accuracy_topics: usize,
    has_glass_cannon: bool,
}

fn derive_stats(
    correct_attempts: i64,
    total_attempts: i64,
    topic_stats: &[TopicStatsEntry],
) -> DerivedStats {
    let max_elo = topic_stats.iter().map(|t| t.peak_elo).max().unwrap_or(0);
    let topics_above_1600 = topic_stats.iter().filter(|t| t.peak_elo >= 1600).count();

    let has_perfectionist = topic_stats
        .iter()
        .any(|t| t.correct == t.total && t.total >= 20);

    let max_topic_correct = topic_stats.iter().map(|t| t.correct).max().unwrap_or(0);
    let has_one_trick =
        correct_attempts >= 50 && max_topic_correct as f64 / correct_attempts as f64 >= 0.8;

    let has_stubborn = topic_stats.iter().any(|t| (t.total - t.correct) >= 50);

    let has_unbreakable = topic_stats.iter().any(|t| t.peak_topic_streak >= 20);

    let has_the_wall = topic_stats.iter().any(|t| t.peak_topic_streak >= 50);

    let high_accuracy_topics = topic_stats
        .iter()
        .filter(|t| t.total >= 20 && t.correct as f64 / t.total as f64 >= 0.9)
        .count();

    let has_glass_cannon = topic_stats.iter().any(|t| {
        t.total > 0
            && t.total < 30
            && t.correct as f64 / t.total as f64 > 0.95
            && t.peak_elo >= 1700
    });

    DerivedStats {
        max_elo,
        topics_above_1600,
        has_perfectionist,
        has_one_trick,
        has_stubborn,
        has_unbreakable,
        has_the_wall,
        high_accuracy_topics,
        has_glass_cannon,
    }
}

fn qualifies(
    badge_id: &str,
    current_streak: i32,
    daily_puzzle_streak: i32,
    correct_attempts: i64,
    total_attempts: i64,
    ds: &DerivedStats,
) -> bool {
    match badge_id {
        // Streak
        "streak_3" => current_streak >= 3,
        "streak_7" => current_streak >= 7,
        "streak_30" => current_streak >= 30,
        "streak_100" => current_streak >= 100,
        // ELO
        "elo_1600" => ds.max_elo >= 1600,
        "elo_1800" => ds.max_elo >= 1800,
        "elo_2000" => ds.max_elo >= 2000,
        "elo_2500" => ds.max_elo >= 2500,
        // Problems
        "solved_50" => correct_attempts >= 50,
        "solved_250" => correct_attempts >= 250,
        "solved_1000" => correct_attempts >= 1000,
        "solved_5000" => correct_attempts >= 5000,
        // Topic mastery
        "topics_3" => ds.topics_above_1600 >= 3,
        "topics_5" => ds.topics_above_1600 >= 5,
        "topics_8" => ds.topics_above_1600 >= 8,
        // Daily puzzle
        "daily_3" => daily_puzzle_streak >= 3,
        "daily_7" => daily_puzzle_streak >= 7,
        "daily_30" => daily_puzzle_streak >= 30,
        // Fun
        "first_blood" => correct_attempts >= 1,
        "try_hard" => total_attempts >= 500,
        "perfectionist" => ds.has_perfectionist,
        "sharpshooter" => {
            total_attempts >= 100 && correct_attempts as f64 / total_attempts as f64 >= 0.9
        }
        "one_trick" => ds.has_one_trick,
        "lucky" => correct_attempts >= 777,
        "stubborn" => ds.has_stubborn,
        "unbreakable" => ds.has_unbreakable,
        "the_wall" => ds.has_the_wall,
        "triple_threat" => ds.high_accuracy_topics >= 3,
        "glass_cannon" => ds.has_glass_cannon,
        _ => false,
    }
}

/// Compute all earned badges from user stats.
#[allow(dead_code)]
pub fn compute_badges(
    current_streak: i32,
    daily_puzzle_streak: i32,
    correct_attempts: i64,
    total_attempts: i64,
    topic_stats: &[TopicStatsEntry],
) -> Vec<EarnedBadge> {
    let ds = derive_stats(correct_attempts, total_attempts, topic_stats);

    let mut earned = Vec::new();
    for badge in BADGES {
        // Fun badges are defined but not yet displayed
        if matches!(badge.category, BadgeCategory::Fun) {
            continue;
        }
        if qualifies(
            badge.id,
            current_streak,
            daily_puzzle_streak,
            correct_attempts,
            total_attempts,
            &ds,
        ) {
            earned.push(badge.to_earned());
        }
    }
    earned
}

/// Compute all badges with earned/locked status.
/// Returns all badges in fixed order, each with `earned: true/false`.
pub fn compute_all_badges(
    current_streak: i32,
    daily_puzzle_streak: i32,
    correct_attempts: i64,
    total_attempts: i64,
    topic_stats: &[TopicStatsEntry],
) -> Vec<BadgeDisplay> {
    let ds = derive_stats(correct_attempts, total_attempts, topic_stats);

    BADGES
        .iter()
        // Fun badges are defined but not yet displayed
        .filter(|badge| !matches!(badge.category, BadgeCategory::Fun))
        .map(|badge| {
            let q = qualifies(
                badge.id,
                current_streak,
                daily_puzzle_streak,
                correct_attempts,
                total_attempts,
                &ds,
            );
            badge.to_display(q)
        })
        .collect()
}
