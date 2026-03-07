//! Application configuration

use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn from_env() -> Self {
        match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" | "prod" => Environment::Production,
            _ => Environment::Development,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub environment: Environment,
    pub allowed_origins: Vec<String>,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub oauth_redirect_base: String,
    pub resend_api_key: String,
    pub resend_from_email: String,
    pub resend_from_name: String,
    pub frontend_base_url: String,
    pub max_db_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let environment = Environment::from_env();

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "development-secret-change-in-production".to_string());

        // Validate JWT secret in production
        if environment == Environment::Production {
            if jwt_secret == "development-secret-change-in-production" {
                return Err(ConfigError::InsecureJwtSecret);
            }
            if jwt_secret.len() < 32 {
                return Err(ConfigError::JwtSecretTooShort);
            }
        } else if jwt_secret == "development-secret-change-in-production" {
            tracing::warn!(
                "Using default JWT secret in development mode. This is insecure for production!"
            );
        }

        // Parse allowed origins
        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:8080".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let max_db_connections = env::var("MAX_DB_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(if environment == Environment::Production { 50 } else { 10 });

        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidPort)?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingEnv("DATABASE_URL"))?,
            jwt_secret,
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            environment,
            allowed_origins,
            google_client_id: env::var("GOOGLE_CLIENT_ID").ok().filter(|s| !s.is_empty()),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .ok()
                .filter(|s| !s.is_empty()),
            github_client_id: env::var("GITHUB_CLIENT_ID").ok().filter(|s| !s.is_empty()),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET")
                .ok()
                .filter(|s| !s.is_empty()),
            oauth_redirect_base: env::var("OAUTH_REDIRECT_BASE")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            resend_api_key: env::var("RESEND_API_KEY")
                .map_err(|_| ConfigError::MissingEnv("RESEND_API_KEY"))?,
            resend_from_email: env::var("RESEND_FROM_EMAIL")
                .unwrap_or_else(|_| "no-reply@locusmath.org".to_string()),
            resend_from_name: env::var("RESEND_FROM_NAME").unwrap_or_else(|_| "Locus".to_string()),
            frontend_base_url: env::var("FRONTEND_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            max_db_connections,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),

    #[error("Invalid port number")]
    InvalidPort,

    #[error(
        "JWT secret cannot be the default development secret in production. Generate a secure secret with: openssl rand -base64 32"
    )]
    InsecureJwtSecret,

    #[error("JWT secret must be at least 32 characters long in production")]
    JwtSecretTooShort,

}
