use governor::middleware::NoOpMiddleware;
use std::time::Duration;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor, GovernorLayer,
};

/// Creates a rate limiter for authentication endpoints (register)
/// Limit: 5 requests per 15 minutes per IP
pub fn auth_rate_limiter() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware, axum::body::Body> {
    let requests: u32 = std::env::var("RATE_LIMIT_AUTH_PER_15MIN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);

    let config = GovernorConfigBuilder::default()
        .per_second(requests as u64)
        .burst_size(requests)
        .period(Duration::from_secs(15 * 60))
        .finish()
        .expect("Failed to create auth rate limiter config");

    GovernorLayer::new(config)
}

/// Creates a rate limiter for login endpoints
/// Limit: 10 requests per 15 minutes per IP
pub fn login_rate_limiter() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware, axum::body::Body> {
    let requests: u32 = std::env::var("RATE_LIMIT_LOGIN_PER_15MIN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    let config = GovernorConfigBuilder::default()
        .per_second(requests as u64)
        .burst_size(requests)
        .period(Duration::from_secs(15 * 60))
        .finish()
        .expect("Failed to create login rate limiter config");

    GovernorLayer::new(config)
}

/// Creates a general rate limiter for all other endpoints
/// Limit: 1000 requests per minute per IP
pub fn general_rate_limiter() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware, axum::body::Body> {
    let requests: u32 = std::env::var("RATE_LIMIT_GENERAL_PER_MIN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);

    let config = GovernorConfigBuilder::default()
        .per_second(requests as u64)
        .burst_size(requests)
        .period(Duration::from_secs(60))
        .finish()
        .expect("Failed to create general rate limiter config");

    GovernorLayer::new(config)
}
