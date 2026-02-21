//! Environment configuration
//!
//! CRITICAL: Never hardcode production URLs. Always use these functions.
//!
//! This module provides compile-time environment configuration that respects
//! the LOCUS_API_URL and LOCUS_FRONTEND_URL environment variables set during build.
//!
//! In development: These are set by dev.sh to localhost
//! In production: These should be set to production domains or use hardcoded fallbacks

/// Get API base URL (compile-time environment variable)
///
/// Set via LOCUS_API_URL environment variable at build time.
///
/// Examples:
/// - Dev: http://localhost:3000/api
/// - Prod: https://api.locusmath.org/api
pub const fn api_base() -> &'static str {
    match option_env!("LOCUS_API_URL") {
        Some(url) => url,
        None => "https://api.locusmath.org/api", // Fallback for production builds
    }
}

/// Get frontend base URL (for OAuth redirects, etc.)
///
/// Set via LOCUS_FRONTEND_URL environment variable at build time.
///
/// Examples:
/// - Dev: http://localhost:8080
/// - Prod: https://locusmath.org
pub const fn frontend_base() -> &'static str {
    match option_env!("LOCUS_FRONTEND_URL") {
        Some(url) => url,
        None => "https://locusmath.org", // Fallback for production builds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_base_returns_valid_url() {
        let base = api_base();
        assert!(!base.is_empty());
        assert!(base.starts_with("http://") || base.starts_with("https://"));
    }

    #[test]
    fn test_frontend_base_returns_valid_url() {
        let base = frontend_base();
        assert!(!base.is_empty());
        assert!(base.starts_with("http://") || base.starts_with("https://"));
    }
}
