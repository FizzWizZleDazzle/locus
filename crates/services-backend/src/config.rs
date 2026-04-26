use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub environment: Environment,
    pub allowed_origins: Vec<String>,
    pub health_check_url: String,
    pub health_check_interval_secs: u64,
    pub cookie_domain: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" | "prod" => Environment::Production,
            _ => Environment::Development,
        };

        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| ConfigError::MissingEnv("JWT_SECRET"))?;

        if environment == Environment::Production && jwt_secret.len() < 32 {
            return Err(ConfigError::JwtSecretTooShort);
        }

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:8081,http://localhost:8082".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "8090".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidPort)?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingEnv("DATABASE_URL"))?,
            jwt_secret,
            environment,
            allowed_origins,
            health_check_url: env::var("HEALTH_CHECK_URL")
                .unwrap_or_else(|_| "http://localhost:3000/api/health".to_string()),
            health_check_interval_secs: env::var("HEALTH_CHECK_INTERVAL_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            cookie_domain: env::var("COOKIE_DOMAIN").ok().filter(|s| !s.is_empty()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),

    #[error("Invalid port number")]
    InvalidPort,

    #[error("JWT secret must be at least 32 characters in production")]
    JwtSecretTooShort,
}
