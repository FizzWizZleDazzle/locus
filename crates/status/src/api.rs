use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_URL: &str = env!("COMMUNITY_API_URL");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentStatus {
    pub is_healthy: Option<bool>,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub checked_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCheck {
    pub id: i32,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub is_healthy: bool,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UptimeResponse {
    pub uptime_30d_percent: f64,
    pub daily_breakdown: Vec<DayUptime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayUptime {
    pub date: String,
    pub checks_total: i64,
    pub checks_healthy: i64,
    pub uptime_percent: f64,
}

pub async fn get_current() -> Result<CurrentStatus, String> {
    let resp = Request::get(&format!("{API_URL}/status/current"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_history() -> Result<Vec<StatusCheck>, String> {
    let resp = Request::get(&format!("{API_URL}/status/history"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_uptime() -> Result<UptimeResponse, String> {
    let resp = Request::get(&format!("{API_URL}/status/uptime"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}
