//! Authentication endpoints

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, Json};

use locus_common::{AuthResponse, LoginRequest, RegisterRequest, SetPasswordRequest, UserProfile};

use crate::{
    auth::{create_token, AuthUser},
    models::User,
    AppError,
};

use locus_common::validation;
use super::AppState;

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate username
    validation::validate_username(&req.username)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Validate password complexity
    validation::validate_password(&req.password)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Validate email format
    validation::validate_email(&req.email)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Check if username or email already exists
    if User::username_exists(&state.pool, &req.username).await? {
        return Err(AppError::BadRequest("Username already taken".into()));
    }
    if User::email_exists(&state.pool, &req.email).await? {
        return Err(AppError::BadRequest("Email already registered".into()));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?
        .to_string();

    // Create user
    let user = User::create(&state.pool, &req.username, &req.email, &password_hash).await?;

    // Generate token
    let token = create_token(user.id, &user.username, &state.jwt_secret, 24)
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

    Ok(Json(AuthResponse {
        token,
        user: user.to_profile(&state.pool).await?,
    }))
}

/// Login with email and password
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find user by email
    let user = User::find_by_email(&state.pool, &req.email)
        .await?
        .ok_or_else(|| AppError::Auth("Invalid email or password".into()))?;

    // Check if user has a password set
    let password_hash = user.password_hash.as_deref()
        .ok_or_else(|| AppError::Auth(
            "This account uses social login. Sign in with Google or GitHub, or set a password in Settings.".into()
        ))?;

    // Verify password
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| AppError::Internal("Invalid password hash in database".into()))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Auth("Invalid email or password".into()))?;

    // Generate token
    let token = create_token(user.id, &user.username, &state.jwt_secret, 24)
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

    Ok(Json(AuthResponse {
        token,
        user: user.to_profile(&state.pool).await?,
    }))
}

/// Set password for the current user (for OAuth users who want email+password login)
pub async fn set_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<SetPasswordRequest>,
) -> Result<Json<UserProfile>, AppError> {
    // Validate password complexity
    validation::validate_password(&req.password)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?
        .to_string();

    // Update user
    User::set_password_hash(&state.pool, auth_user.id, &password_hash).await?;

    let user = User::find_by_id(&state.pool, auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user.to_profile(&state.pool).await?))
}

/// Get current user profile
pub async fn get_me(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<UserProfile>, AppError> {
    let user = User::find_by_id(&state.pool, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user.to_profile(&state.pool).await?))
}
