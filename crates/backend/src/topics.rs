//! Dynamic Topics System
//!
//! Provides cached access to topics and subtopics from the database.
//! Cache is refreshed on startup and daily via background task.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub display_name: String,
    pub sort_order: i32,
    pub enabled: bool,
    pub subtopics: Vec<Subtopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtopic {
    pub id: String,
    pub display_name: String,
    pub sort_order: i32,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct TopicCache {
    inner: Arc<RwLock<Vec<Topic>>>,
    pool: PgPool,
}

impl TopicCache {
    /// Create a new topic cache and load initial data
    pub async fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        let cache = Self {
            inner: Arc::new(RwLock::new(Vec::new())),
            pool,
        };
        cache.refresh().await?;
        Ok(cache)
    }

    /// Refresh topics from database
    pub async fn refresh(&self) -> Result<(), sqlx::Error> {
        let topics = load_topics(&self.pool).await?;
        let mut cache = self.inner.write().await;
        *cache = topics;
        Ok(())
    }

    /// Get all topics (returns cached data)
    pub async fn get_all(&self) -> Vec<Topic> {
        self.inner.read().await.clone()
    }

    /// Get enabled topics only
    pub async fn get_enabled(&self) -> Vec<Topic> {
        self.inner
            .read()
            .await
            .iter()
            .filter(|t| t.enabled)
            .map(|t| {
                let mut topic = t.clone();
                topic.subtopics.retain(|s| s.enabled);
                topic
            })
            .collect()
    }

    /// Start background refresh task (daily)
    pub fn spawn_refresh_task(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400)); // 24 hours
            loop {
                interval.tick().await;
                if let Err(e) = self.refresh().await {
                    eprintln!("Failed to refresh topic cache: {}", e);
                }
            }
        });
    }
}

/// Load all topics and subtopics from database
async fn load_topics(pool: &PgPool) -> Result<Vec<Topic>, sqlx::Error> {
    // Load all topics
    let topic_rows: Vec<(String, String, i32, bool)> = sqlx::query_as(
        "SELECT id, display_name, sort_order, enabled FROM topics ORDER BY sort_order",
    )
    .fetch_all(pool)
    .await?;

    // Load all subtopics
    let subtopic_rows: Vec<(String, String, String, i32, bool)> = sqlx::query_as(
        "SELECT topic_id, id, display_name, sort_order, enabled FROM subtopics ORDER BY sort_order",
    )
    .fetch_all(pool)
    .await?;

    // Build topic tree
    let mut topics = Vec::new();
    for (id, display_name, sort_order, enabled) in topic_rows {
        let subtopics: Vec<Subtopic> = subtopic_rows
            .iter()
            .filter(|(topic_id, _, _, _, _)| topic_id == &id)
            .map(
                |(_, sub_id, sub_display_name, sub_sort_order, sub_enabled)| Subtopic {
                    id: sub_id.clone(),
                    display_name: sub_display_name.clone(),
                    sort_order: *sub_sort_order,
                    enabled: *sub_enabled,
                },
            )
            .collect();

        topics.push(Topic {
            id,
            display_name,
            sort_order,
            enabled,
            subtopics,
        });
    }

    Ok(topics)
}
