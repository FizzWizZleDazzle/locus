//! Authentication endpoints

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, response::AppendHeaders};

use locus_common::{
    AuthResponse, ChangePasswordRequest, ChangeUsernameRequest, DeleteAccountRequest, LoginRequest,
    RegisterRequest, SetPasswordRequest, SuccessResponse, UnlinkOAuthRequest, UserProfile,
};
use serde::{Deserialize, Serialize};

use crate::{
    AppError,
    auth::{AuthUser, build_auth_cookie, build_clear_cookie, create_token},
    models::{EmailVerificationToken, OAuthAccount, PasswordResetToken, User},
};

use super::AppState;
use locus_common::validation;

use axum::http::header::SET_COOKIE;

/// Hash a password using Argon2 on a blocking thread
async fn hash_password_blocking(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))
    })
    .await
    .map_err(|e| AppError::Internal(format!("Hash task failed: {}", e)))?
}

/// Verify a password against a hash using Argon2 on a blocking thread
async fn verify_password_blocking(password: String, hash: String) -> Result<(), AppError> {
    tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash)
            .map_err(|_| AppError::Internal("Invalid password hash in database".into()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Auth("Invalid email or password".into()))
    })
    .await
    .map_err(|e| AppError::Internal(format!("Verify task failed: {}", e)))?
}

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
    // Validate TOS acceptance
    if !req.accepted_tos {
        return Err(AppError::BadRequest(
            "You must accept the Terms of Service and Privacy Policy to register".into(),
        ));
    }

    // Validate username
    validation::validate_username(&req.username)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Validate password complexity
    validation::validate_password(&req.password)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Validate email format
    validation::validate_email(&req.email).map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Check if username or email already exists
    if User::username_exists(&state.pool, &req.username).await? {
        return Err(AppError::BadRequest("Username already taken".into()));
    }
    if User::email_exists(&state.pool, &req.email).await? {
        return Err(AppError::BadRequest("Email already registered".into()));
    }

    // Hash password (offloaded to blocking thread)
    let password_hash = hash_password_blocking(req.password.clone()).await?;

    // Create user (email_verified defaults to FALSE)
    let user = User::create(&state.pool, &req.username, &req.email, &password_hash).await?;

    // Generate verification token
    let verification_token = EmailVerificationToken::create(&state.pool, user.id).await?;

    // Send verification email
    state
        .email_service
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
) -> Result<
    (
        AppendHeaders<[(axum::http::HeaderName, String); 1]>,
        Json<AuthResponse>,
    ),
    AppError,
> {
    // Find user by email
    let user = User::find_by_email(&state.pool, &req.email)
        .await?
        .ok_or_else(|| AppError::Auth("Invalid email or password".into()))?;

    // Check if user has a password set
    let password_hash = user.password_hash.as_deref()
        .ok_or_else(|| AppError::Auth(
            "This account uses social login. Sign in with Google or GitHub, or set a password in Settings.".into()
        ))?;

    // Verify password (offloaded to blocking thread)
    verify_password_blocking(req.password.clone(), password_hash.to_string()).await?;

    // Check if email is verified
    if !user.email_verified {
        return Err(AppError::Auth(
            "Please verify your email address before logging in. Check your inbox for the verification link.".into()
        ));
    }

    // Generate token
    let token = create_token(user.id, &user.username, &state.jwt_secret, 24)
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

    let cookie = build_auth_cookie(&token, 24, state.is_production);

    Ok((
        AppendHeaders([(SET_COOKIE, cookie)]),
        Json(AuthResponse {
            user: user.to_profile(&state.pool).await?,
        }),
    ))
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

    // Hash password (offloaded to blocking thread)
    let password_hash = hash_password_blocking(req.password.clone()).await?;

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

/// Verify email with token (atomic: marks token used + user verified in one query)
pub async fn verify_email(
    State(state): State<AppState>,
    Json(req): Json<VerifyEmailRequest>,
) -> Result<Json<VerifyEmailResponse>, AppError> {
    // Atomic CTE: mark token used AND verify user in a single query
    let result: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"
        WITH verified AS (
            UPDATE email_verification_tokens SET used_at = NOW()
            WHERE token = $1 AND used_at IS NULL AND expires_at > NOW()
            RETURNING user_id
        )
        UPDATE users SET email_verified = TRUE, email_verified_at = NOW()
        FROM verified WHERE users.id = verified.user_id
        RETURNING users.id
        "#,
    )
    .bind(&req.token)
    .fetch_optional(&state.pool)
    .await?;

    if result.is_none() {
        // Token was invalid, expired, or already used — check which for better error message
        let token = EmailVerificationToken::find_by_token(&state.pool, &req.token).await?;
        return Err(match token {
            Some(t) if t.used_at.is_some() => {
                AppError::BadRequest("This verification link has already been used".into())
            }
            Some(_) => AppError::BadRequest(
                "This verification link has expired. Request a new one.".into(),
            ),
            None => AppError::BadRequest("Invalid verification token".into()),
        });
    }

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
    // SECURITY: Always return generic success to prevent email enumeration
    let generic_response = ResendVerificationResponse {
        success: true,
        message: "If this email is registered and unverified, a verification link will be sent."
            .to_string(),
    };

    // Find user by email (silently succeed if not found)
    let user = match User::find_by_email(&state.pool, &req.email).await? {
        Some(user) => user,
        None => return Ok(Json(generic_response)),
    };

    // Already verified or rate limited — return generic response
    if user.email_verified {
        return Ok(Json(generic_response));
    }
    if !EmailVerificationToken::can_send_email(&state.pool, user.id).await? {
        return Ok(Json(generic_response));
    }

    // Generate new token
    let verification_token = EmailVerificationToken::create(&state.pool, user.id).await?;

    // Send verification email
    state
        .email_service
        .send_verification_email(&user.email, &user.username, &verification_token.token)
        .await?;

    // Record send time
    EmailVerificationToken::record_send(&state.pool, user.id).await?;

    Ok(Json(ResendVerificationResponse {
        success: true,
        message: "Verification email sent! Check your inbox.".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct ForgotPasswordResponse {
    pub success: bool,
    pub message: String,
}

/// Request password reset (sends email)
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(req): Json<ForgotPasswordRequest>,
) -> Result<Json<ForgotPasswordResponse>, AppError> {
    // SECURITY: Always return success to prevent email enumeration
    let generic_response = Json(ForgotPasswordResponse {
        success: true,
        message: "If this email is registered, you will receive a password reset link.".to_string(),
    });

    // Validate email format
    if validation::validate_email(&req.email).is_err() {
        return Ok(generic_response);
    }

    // Find user by email (silently fail if not found)
    let user = match User::find_by_email(&state.pool, &req.email).await? {
        Some(user) => user,
        None => return Ok(generic_response),
    };

    // Check rate limit (1 email per minute) — return generic response to prevent enumeration
    if !PasswordResetToken::can_send_email(&state.pool, user.id).await? {
        return Ok(generic_response);
    }

    // Generate reset token (30-minute expiry)
    let reset_token = PasswordResetToken::create(&state.pool, user.id).await?;

    // Send reset email
    state
        .email_service
        .send_password_reset_email(&user.email, &user.username, &reset_token.token)
        .await?;

    // Record send time
    PasswordResetToken::record_send(&state.pool, user.id).await?;

    Ok(generic_response)
}

#[derive(Deserialize)]
pub struct ValidateResetTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct ValidateResetTokenResponse {
    pub valid: bool,
    pub message: Option<String>,
}

/// Validate password reset token
pub async fn validate_reset_token(
    State(state): State<AppState>,
    Json(req): Json<ValidateResetTokenRequest>,
) -> Result<Json<ValidateResetTokenResponse>, AppError> {
    // Find token
    let token = PasswordResetToken::find_by_token(&state.pool, &req.token).await?;

    let response = match token {
        Some(token) if token.is_valid() => ValidateResetTokenResponse {
            valid: true,
            message: None,
        },
        Some(token) if token.used_at.is_some() => ValidateResetTokenResponse {
            valid: false,
            message: Some("This reset link has already been used".to_string()),
        },
        Some(_) => ValidateResetTokenResponse {
            valid: false,
            message: Some("This reset link has expired".to_string()),
        },
        None => ValidateResetTokenResponse {
            valid: false,
            message: Some("Invalid reset link".to_string()),
        },
    };

    Ok(Json(response))
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
}

/// Reset password with token (atomic: marks token used + updates password in transaction)
pub async fn reset_password(
    State(state): State<AppState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<ResetPasswordResponse>, AppError> {
    // Validate password complexity
    validation::validate_password(&req.new_password)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Hash new password (offloaded to blocking thread)
    let password_hash = hash_password_blocking(req.new_password.clone()).await?;

    // Atomic CTE: mark token used AND update password in one query
    let result: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"
        WITH used_token AS (
            UPDATE password_reset_tokens SET used_at = NOW()
            WHERE token = $1 AND used_at IS NULL AND expires_at > NOW()
            RETURNING user_id
        )
        UPDATE users SET password_hash = $2
        FROM used_token WHERE users.id = used_token.user_id
        RETURNING users.id
        "#,
    )
    .bind(&req.token)
    .bind(&password_hash)
    .fetch_optional(&state.pool)
    .await?;

    if result.is_none() {
        let token = PasswordResetToken::find_by_token(&state.pool, &req.token).await?;
        return Err(match token {
            Some(t) if t.used_at.is_some() => {
                AppError::BadRequest("This reset link has already been used".into())
            }
            Some(_) => {
                AppError::BadRequest("This reset link has expired. Request a new one.".into())
            }
            None => AppError::BadRequest("Invalid reset token".into()),
        });
    }

    Ok(Json(ResetPasswordResponse {
        success: true,
        message: "Password reset successful! You can now log in with your new password."
            .to_string(),
    }))
}

/// Change password for authenticated user (requires old password verification)
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    // Get user
    let user = User::find_by_id(&state.pool, auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Check if user has a password set
    let password_hash = user.password_hash.as_deref().ok_or_else(|| {
        AppError::BadRequest(
            "This account does not have a password set. Use 'Set Password' instead.".into(),
        )
    })?;

    // Verify old password (offloaded to blocking thread)
    verify_password_blocking(req.old_password.clone(), password_hash.to_string())
        .await
        .map_err(|_| AppError::Auth("Incorrect old password".into()))?;

    // Validate new password complexity
    validation::validate_password(&req.new_password)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Hash new password (offloaded to blocking thread)
    let new_password_hash = hash_password_blocking(req.new_password.clone()).await?;

    // Update password
    User::set_password_hash(&state.pool, auth_user.id, &new_password_hash).await?;

    Ok(Json(SuccessResponse {
        success: true,
        message: Some("Password changed successfully".to_string()),
    }))
}

/// Change username for authenticated user
pub async fn change_username(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<ChangeUsernameRequest>,
) -> Result<Json<UserProfile>, AppError> {
    // Validate username format
    validation::validate_username(&req.new_username)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Check uniqueness (exclude current user)
    let existing = User::find_by_id(&state.pool, auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if existing.username == req.new_username {
        return Err(AppError::BadRequest(
            "New username is the same as current username".into(),
        ));
    }

    // Update username - database UNIQUE constraint handles race conditions
    match User::update_username(&state.pool, auth_user.id, &req.new_username).await {
        Ok(_) => {
            let user = User::find_by_id(&state.pool, auth_user.id)
                .await?
                .ok_or_else(|| AppError::NotFound("User not found".into()))?;
            Ok(Json(user.to_profile(&state.pool).await?))
        }
        Err(sqlx::Error::Database(db_err)) => {
            // Check for unique constraint violation
            if let Some(constraint) = db_err.constraint() {
                if constraint == "users_username_key" {
                    return Err(AppError::BadRequest("Username already taken".into()));
                }
            }
            Err(AppError::Database(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(AppError::Database(e)),
    }
}

/// Logout — clears the auth cookie
pub async fn logout(
    State(state): State<AppState>,
) -> (
    AppendHeaders<[(axum::http::HeaderName, String); 1]>,
    Json<SuccessResponse>,
) {
    let cookie = build_clear_cookie(state.is_production);
    (
        AppendHeaders([(SET_COOKIE, cookie)]),
        Json(SuccessResponse {
            success: true,
            message: Some("Logged out successfully".to_string()),
        }),
    )
}

/// Delete user account
pub async fn delete_account(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<DeleteAccountRequest>,
) -> Result<
    (
        AppendHeaders<[(axum::http::HeaderName, String); 1]>,
        Json<SuccessResponse>,
    ),
    AppError,
> {
    let user = User::find_by_id(&state.pool, auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // If user has password, verify it
    if let Some(password_hash) = user.password_hash.as_deref() {
        let provided_password = req
            .password
            .as_deref()
            .ok_or_else(|| AppError::BadRequest("Password required to delete account".into()))?;

        verify_password_blocking(provided_password.to_string(), password_hash.to_string())
            .await
            .map_err(|_| AppError::Auth("Incorrect password".into()))?;
    } else {
        // OAuth-only account: require username confirmation
        let confirmation = req
            .confirmation
            .as_deref()
            .ok_or_else(|| AppError::BadRequest("Please confirm by typing your username".into()))?;
        if confirmation != user.username {
            return Err(AppError::BadRequest(
                "Confirmation does not match your username".into(),
            ));
        }
    }

    // Delete user (cascades to oauth_accounts, attempts, etc.)
    User::delete_account(&state.pool, auth_user.id).await?;

    let cookie = build_clear_cookie(state.is_production);
    Ok((
        AppendHeaders([(SET_COOKIE, cookie)]),
        Json(SuccessResponse {
            success: true,
            message: Some("Account deleted successfully".to_string()),
        }),
    ))
}

/// Validate OAuth provider name
fn validate_oauth_provider(provider: &str) -> Result<(), AppError> {
    match provider {
        "google" | "github" => Ok(()),
        _ => Err(AppError::BadRequest(format!(
            "Invalid OAuth provider: {}. Supported providers: google, github",
            provider
        ))),
    }
}

/// Unlink OAuth provider
pub async fn unlink_oauth(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<UnlinkOAuthRequest>,
) -> Result<Json<UserProfile>, AppError> {
    // Validate provider name
    validate_oauth_provider(&req.provider)?;

    let user = User::find_by_id(&state.pool, auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Check if user has password OR another OAuth account (prevent lockout)
    let has_password = user.password_hash.is_some();
    let oauth_count = OAuthAccount::count_by_user(&state.pool, auth_user.id).await?;

    if !has_password && oauth_count <= 1 {
        return Err(AppError::BadRequest(
            "Cannot unlink your only authentication method. Set a password or link another account first.".into()
        ));
    }

    // Unlink the OAuth account
    OAuthAccount::delete_by_user_and_provider(&state.pool, auth_user.id, &req.provider).await?;

    // Return updated profile
    let user = User::find_by_id(&state.pool, auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user.to_profile(&state.pool).await?))
}
