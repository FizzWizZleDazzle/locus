use std::time::{Duration, Instant};
use sqlx::PgPool;

use super::models;

pub fn spawn_health_checker(pool: PgPool, url: String, interval_secs: u64) {
    tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        let check_interval = Duration::from_secs(interval_secs);
        let cleanup_interval = Duration::from_secs(3600); // 1 hour
        let mut last_cleanup = Instant::now();

        loop {
            // Perform health check
            let start = Instant::now();
            let result = client.get(&url).send().await;
            let elapsed_ms = start.elapsed().as_millis() as i32;

            let (status_code, is_healthy) = match result {
                Ok(resp) => {
                    let code = resp.status().as_u16() as i32;
                    (Some(code), resp.status().is_success())
                }
                Err(e) => {
                    tracing::warn!("Health check failed: {}", e);
                    (None, false)
                }
            };

            if let Err(e) = models::insert_check(&pool, status_code, Some(elapsed_ms), is_healthy).await {
                tracing::error!("Failed to insert health check: {}", e);
            }

            // Periodic cleanup
            if last_cleanup.elapsed() >= cleanup_interval {
                match models::cleanup_old(&pool).await {
                    Ok(n) if n > 0 => tracing::info!("Cleaned up {} old status checks", n),
                    Err(e) => tracing::error!("Failed to cleanup old checks: {}", e),
                    _ => {}
                }
                last_cleanup = Instant::now();
            }

            tokio::time::sleep(check_interval).await;
        }
    });
}
