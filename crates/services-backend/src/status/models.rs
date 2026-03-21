use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StatusCheck {
    pub id: i32,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub is_healthy: bool,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UptimeResponse {
    pub uptime_30d_percent: f64,
    pub daily_breakdown: Vec<DayUptime>,
}

#[derive(Debug, Serialize)]
pub struct DayUptime {
    pub date: String,
    pub checks_total: i64,
    pub checks_healthy: i64,
    pub uptime_percent: f64,
}

pub async fn get_latest(pool: &PgPool) -> Result<Option<StatusCheck>, sqlx::Error> {
    sqlx::query_as::<_, StatusCheck>(
        "SELECT * FROM status_checks ORDER BY checked_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_history_24h(pool: &PgPool) -> Result<Vec<StatusCheck>, sqlx::Error> {
    sqlx::query_as::<_, StatusCheck>(
        "SELECT * FROM status_checks
         WHERE checked_at > NOW() - INTERVAL '24 hours'
         ORDER BY checked_at ASC"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_uptime_30d(pool: &PgPool) -> Result<UptimeResponse, sqlx::Error> {
    // Overall 30-day uptime
    let row: (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*)::bigint,
            COUNT(*) FILTER (WHERE is_healthy)::bigint
         FROM status_checks
         WHERE checked_at > NOW() - INTERVAL '30 days'"
    )
    .fetch_one(pool)
    .await?;

    let uptime_30d_percent = if row.0 > 0 {
        (row.1 as f64 / row.0 as f64) * 100.0
    } else {
        100.0
    };

    // Daily breakdown
    let daily: Vec<(String, i64, i64)> = sqlx::query_as(
        "SELECT
            TO_CHAR(checked_at::date, 'YYYY-MM-DD'),
            COUNT(*)::bigint,
            COUNT(*) FILTER (WHERE is_healthy)::bigint
         FROM status_checks
         WHERE checked_at > NOW() - INTERVAL '30 days'
         GROUP BY checked_at::date
         ORDER BY checked_at::date ASC"
    )
    .fetch_all(pool)
    .await?;

    let daily_breakdown = daily
        .into_iter()
        .map(|(date, total, healthy)| DayUptime {
            date,
            checks_total: total,
            checks_healthy: healthy,
            uptime_percent: if total > 0 { (healthy as f64 / total as f64) * 100.0 } else { 100.0 },
        })
        .collect();

    Ok(UptimeResponse {
        uptime_30d_percent,
        daily_breakdown,
    })
}

pub async fn insert_check(
    pool: &PgPool,
    status_code: Option<i32>,
    response_time_ms: Option<i32>,
    is_healthy: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO status_checks (status_code, response_time_ms, is_healthy) VALUES ($1, $2, $3)"
    )
    .bind(status_code)
    .bind(response_time_ms)
    .bind(is_healthy)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn cleanup_old(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM status_checks WHERE checked_at < NOW() - INTERVAL '30 days'")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
