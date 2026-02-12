//! Authentication endpoints

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, Json};

use locus_common::{AuthResponse, LoginRequest, RegisterRequest, SetPasswordRequest, UserProfile};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{create_token, AuthUser},
    models::{User, EmailVerificationToken},
    AppError,
};

use locus_common::validation;
use super::AppState;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub email: String,
}

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
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

    // Create user (email_verified defaults to FALSE)
    let user = User::create(&state.pool, &req.username, &req.email, &password_hash).await?;

    // Generate verification token
    let verification_token = EmailVerificationToken::create(&state.pool, user.id).await?;

    // Send verification email
    state.email_service
        .send_verification_email(&user.email, &user.username, &verification_token.token)
        .await?;

    // Record email send for rate limiting
    EmailVerificationToken::record_send(&state.pool, user.id).await?;

    Ok(Json(RegisterResponse {
        success: true,
        message: "Registration successful! Check your email to verify your account.".to_string(),
        email: user.email,
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

    // Check if email is verified
    if !user.email_verified {
        return Err(AppError::Auth(
            "Please verify your email address before logging in. Check your inbox for the verification link.".into()
        ));
    }

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

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    pub success: bool,
    pub message: String,
}

/// Verify email with token
pub async fn verify_email(
    State(state): State<AppState>,
    Json(req): Json<VerifyEmailRequest>,
) -> Result<Json<VerifyEmailResponse>, AppError> {
    // Find token
    let token = EmailVerificationToken::find_by_token(&state.pool, &req.token)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid verification token".into()))?;

    // Check if token is valid (not expired, not used)
    if !token.is_valid() {
        if token.used_at.is_some() {
            return Err(AppError::BadRequest("This verification link has already been used".into()));
        } else {
            return Err(AppError::BadRequest("This verification link has expired. Request a new one.".into()));
        }
    }

    // Mark user as verified
    User::mark_email_verified(&state.pool, token.user_id).await?;

    // Mark token as used
    token.mark_used(&state.pool).await?;

    Ok(Json(VerifyEmailResponse {
        success: true,
        message: "Email verified! You can now log in.".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct ResendVerificationResponse {
    pub success: bool,
    pub message: String,
}

/// Resend verification email
pub async fn resend_verification(
    State(state): State<AppState>,
    Json(req): Json<ResendVerificationRequest>,
) -> Result<Json<ResendVerificationResponse>, AppError> {
    // Find user by email
    let user = User::find_by_email(&state.pool, &req.email)
        .await?
        .ok_or_else(|| AppError::BadRequest("If this email is registered, a verification link will be sent".into()))?;

    // Check if already verified
    if user.email_verified {
        return Err(AppError::BadRequest("Your email is already verified".into()));
    }

    // Check rate limit
    if !EmailVerificationToken::can_send_email(&state.pool, user.id).await? {
        return Err(AppError::BadRequest("Please wait 1 minute before requesting another verification email".into()));
    }

    // Generate new token
    let verification_token = EmailVerificationToken::create(&state.pool, user.id).await?;

    // Send verification email
    state.email_service
        .send_verification_email(&user.email, &user.username, &verification_token.token)
        .await?;

    // Record send time
    EmailVerificationToken::record_send(&state.pool, user.id).await?;

    Ok(Json(ResendVerificationResponse {
        success: true,
        message: "Verification email sent! Check your inbox.".to_string(),
    }))
}
