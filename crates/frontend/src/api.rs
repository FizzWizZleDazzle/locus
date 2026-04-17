//! HTTP API client

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use web_sys::RequestCredentials;

use locus_common::{
    ApiError, AuthResponse, ChangePasswordRequest, ChangeUsernameRequest,
    DailyActivityResponse, DailyArchiveEntry, DailyPuzzleDetailResponse, DailyPuzzleResponse,
    DailySubmitRequest, DailySubmitResponse, DeleteAccountRequest, EloHistoryResponse,
    LeaderboardResponse, LoginRequest, ProblemResponse, PublicProfileResponse, RegisterRequest,
    SetPasswordRequest, SubmitRequest, SubmitResponse, SuccessResponse, UnlinkOAuthRequest,
    UserProfile, UserStatsResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub display_name: String,
    pub sort_order: i32,
    pub enabled: bool,
    pub subtopics: Vec<Subtopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtopic {
    pub id: String,
    pub display_name: String,
    pub sort_order: i32,
    pub enabled: bool,
}

// Import centralized environment configuration
// CRITICAL: Never hardcode production URLs - always use crate::env functions
use crate::env;

const USERNAME_KEY: &str = "locus_username";

/// Check if user is logged in (has stored username)
pub fn is_logged_in() -> bool {
    LocalStorage::get::<String>(USERNAME_KEY).is_ok()
}

/// Get stored username
pub fn get_stored_username() -> Option<String> {
    LocalStorage::get::<String>(USERNAME_KEY).ok()
}

/// Store username for UI display (token is in httpOnly cookie)
pub fn store_username(username: &str) {
    let _ = LocalStorage::set(USERNAME_KEY, username);
}

/// Clear auth data
pub fn clear_auth() {
    LocalStorage::delete(USERNAME_KEY);
}

/// API error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestError {
    pub message: String,
}

/// Registration success response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub email: String,
}

/// Verify email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

/// Verify email response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailResponse {
    pub success: bool,
    pub message: String,
}

/// Resend verification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendVerificationRequest {
    pub email: String,
}

/// Resend verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendVerificationResponse {
    pub success: bool,
    pub message: String,
}

/// Forgot password request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// Forgot password response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordResponse {
    pub success: bool,
    pub message: String,
}

/// Validate reset token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResetTokenRequest {
    pub token: String,
}

/// Validate reset token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResetTokenResponse {
    pub valid: bool,
    pub message: Option<String>,
}

/// Reset password request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

/// Reset password response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// On 401, clear local auth state and redirect to /login.
fn handle_unauthorized() {
    clear_auth();
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href("/login");
    }
}

/// Make an authenticated GET request
async fn get_request<T: DeserializeOwned>(path: &str) -> Result<T, RequestError> {
    let url = format!("{}{}", env::api_base(), path);

    let resp = Request::get(&url)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| RequestError {
            message: format!("Network error: {}", e),
        })?;

    if resp.ok() {
        resp.json::<T>().await.map_err(|e| RequestError {
            message: format!("Parse error: {}", e),
        })
    } else {
        if resp.status() == 401 && is_logged_in() {
            handle_unauthorized();
        }
        let error: ApiError = resp.json().await.unwrap_or(ApiError::new("Unknown error"));
        Err(RequestError {
            message: error.error,
        })
    }
}

/// Make an authenticated POST request with JSON body
async fn post_request<T: DeserializeOwned, B: Serialize>(
    path: &str,
    body: &B,
) -> Result<T, RequestError> {
    let url = format!("{}{}", env::api_base(), path);

    let req = Request::post(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(serde_json::to_string(body).map_err(|e| RequestError {
            message: format!("Serialization error: {}", e),
        })?)
        .map_err(|e| RequestError {
            message: format!("Request error: {}", e),
        })?;

    let resp = req.send().await.map_err(|e| RequestError {
        message: format!("Network error: {}", e),
    })?;

    if resp.ok() {
        resp.json::<T>().await.map_err(|e| RequestError {
            message: format!("Parse error: {}", e),
        })
    } else {
        if resp.status() == 401 && is_logged_in() {
            handle_unauthorized();
        }
        let error: ApiError = resp.json().await.unwrap_or(ApiError::new("Unknown error"));
        Err(RequestError {
            message: error.error,
        })
    }
}

// ============================================================================
// Auth API
// ============================================================================

pub async fn register(
    username: &str,
    email: &str,
    password: &str,
    accepted_tos: bool,
) -> Result<RegisterResponse, RequestError> {
    let req = RegisterRequest {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        accepted_tos,
    };

    let resp: RegisterResponse = post_request("/auth/register", &req).await?;
    Ok(resp)
}

pub async fn verify_email(token: &str) -> Result<VerifyEmailResponse, RequestError> {
    let req = VerifyEmailRequest {
        token: token.to_string(),
    };

    post_request("/auth/verify-email", &req).await
}

pub async fn resend_verification(email: &str) -> Result<ResendVerificationResponse, RequestError> {
    let req = ResendVerificationRequest {
        email: email.to_string(),
    };

    post_request("/auth/resend-verification", &req).await
}

pub async fn forgot_password(email: &str) -> Result<ForgotPasswordResponse, RequestError> {
    let req = ForgotPasswordRequest {
        email: email.to_string(),
    };

    post_request("/auth/forgot-password", &req).await
}

pub async fn validate_reset_token(token: &str) -> Result<ValidateResetTokenResponse, RequestError> {
    let req = ValidateResetTokenRequest {
        token: token.to_string(),
    };

    post_request("/auth/validate-reset-token", &req).await
}

pub async fn reset_password(
    token: &str,
    new_password: &str,
) -> Result<ResetPasswordResponse, RequestError> {
    let req = ResetPasswordRequest {
        token: token.to_string(),
        new_password: new_password.to_string(),
    };

    post_request("/auth/reset-password", &req).await
}

pub async fn login(email: &str, password: &str) -> Result<AuthResponse, RequestError> {
    let req = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let resp: AuthResponse = post_request("/auth/login", &req).await?;
    store_username(&resp.user.username);
    Ok(resp)
}

pub async fn logout() {
    // Clear the httpOnly cookie via backend
    let _ = post_request::<SuccessResponse, _>("/auth/logout", &serde_json::json!({})).await;
    clear_auth();
}

pub async fn get_me() -> Result<UserProfile, RequestError> {
    get_request("/user/me").await
}

pub async fn set_password(password: &str) -> Result<UserProfile, RequestError> {
    let req = SetPasswordRequest {
        password: password.to_string(),
    };
    post_request("/auth/set-password", &req).await
}

pub async fn change_password(
    old_password: &str,
    new_password: &str,
) -> Result<SuccessResponse, RequestError> {
    let req = ChangePasswordRequest {
        old_password: old_password.to_string(),
        new_password: new_password.to_string(),
    };
    post_request("/auth/change-password", &req).await
}

pub async fn change_username(new_username: &str) -> Result<UserProfile, RequestError> {
    let req = ChangeUsernameRequest {
        new_username: new_username.to_string(),
    };
    let profile: UserProfile = post_request("/auth/change-username", &req).await?;
    // Update stored username
    let _ = LocalStorage::set(USERNAME_KEY, &profile.username);
    Ok(profile)
}

pub async fn delete_account(password: Option<&str>, confirmation: Option<&str>) -> Result<SuccessResponse, RequestError> {
    let req = DeleteAccountRequest {
        password: password.map(|s| s.to_string()),
        confirmation: confirmation.map(|s| s.to_string()),
    };
    let resp: SuccessResponse = post_request("/auth/delete-account", &req).await?;
    // Clear auth data after successful deletion
    clear_auth();
    Ok(resp)
}

pub async fn unlink_oauth(provider: &str) -> Result<UserProfile, RequestError> {
    let req = UnlinkOAuthRequest {
        provider: provider.to_string(),
    };
    post_request("/auth/unlink-oauth", &req).await
}

// ============================================================================
// Problem API
// ============================================================================

pub async fn get_problem(
    practice: bool,
    main_topic: Option<&str>,
    subtopics: Option<&[String]>,
) -> Result<ProblemResponse, RequestError> {
    let mut path = format!("/problem?practice={}", practice);
    if let Some(mt) = main_topic {
        path.push_str(&format!("&main_topic={}", mt));
    }
    if let Some(st) = subtopics {
        if !st.is_empty() {
            let st_str = st.join(",");
            path.push_str(&format!("&subtopics={}", st_str));
        }
    }
    get_request(&path).await
}

pub async fn get_problems(
    practice: bool,
    main_topic: Option<&str>,
    subtopics: Option<&[String]>,
    count: u32,
) -> Result<Vec<ProblemResponse>, RequestError> {
    let mut path = format!("/problems?practice={}&count={}", practice, count);
    if let Some(mt) = main_topic {
        path.push_str(&format!("&main_topic={}", mt));
    }
    if let Some(st) = subtopics {
        if !st.is_empty() {
            let st_str = st.join(",");
            path.push_str(&format!("&subtopics={}", st_str));
        }
    }
    get_request(&path).await
}

pub async fn submit_answer(
    problem_id: uuid::Uuid,
    user_input: &str,
    time_taken_ms: Option<i32>,
) -> Result<SubmitResponse, RequestError> {
    let req = SubmitRequest {
        problem_id,
        user_input: user_input.to_string(),
        time_taken_ms,
    };
    post_request("/submit", &req).await
}

// ============================================================================
// Topics API
// ============================================================================

pub async fn get_topics() -> Result<Vec<Topic>, RequestError> {
    get_request("/topics").await
}

// ============================================================================
// Leaderboard API
// ============================================================================

pub async fn get_leaderboard(topic: &str) -> Result<LeaderboardResponse, RequestError> {
    get_request(&format!("/leaderboard?topic={}", topic)).await
}

// ============================================================================
// Stats API
// ============================================================================

pub async fn get_user_stats() -> Result<UserStatsResponse, RequestError> {
    get_request("/user/stats").await
}

pub async fn get_elo_history(topic: &str) -> Result<EloHistoryResponse, RequestError> {
    get_request(&format!("/user/elo-history?topic={}", topic)).await
}

// ============================================================================
// Daily Puzzle API
// ============================================================================

pub async fn get_daily_today() -> Result<DailyPuzzleResponse, RequestError> {
    get_request("/daily/today").await
}

pub async fn get_daily_puzzle(date: &str) -> Result<DailyPuzzleDetailResponse, RequestError> {
    get_request(&format!("/daily/puzzle/{}", date)).await
}

pub async fn submit_daily(req: &DailySubmitRequest) -> Result<DailySubmitResponse, RequestError> {
    post_request("/daily/submit", req).await
}

pub async fn get_daily_archive(
    limit: i64,
    offset: i64,
) -> Result<Vec<DailyArchiveEntry>, RequestError> {
    get_request(&format!("/daily/archive?limit={}&offset={}", limit, offset)).await
}

pub async fn get_daily_activity() -> Result<DailyActivityResponse, RequestError> {
    get_request("/daily/activity").await
}

// ============================================================================
// Physics API
// ============================================================================

pub async fn get_physics_topics() -> Result<Vec<locus_physics_common::PhysicsTopicInfo>, RequestError> {
    get_request("/physics/topics").await
}

pub async fn get_physics_problems(
    topic: Option<&str>,
    subtopic: Option<&str>,
    count: u32,
) -> Result<Vec<locus_physics_common::PhysicsProblemSummary>, RequestError> {
    let mut path = format!("/physics/problems?count={}", count);
    if let Some(t) = topic {
        path.push_str(&format!("&physics_topic={}", t));
    }
    if let Some(st) = subtopic {
        path.push_str(&format!("&physics_subtopic={}", st));
    }
    get_request(&path).await
}

pub async fn get_physics_problem(
    id: uuid::Uuid,
) -> Result<locus_physics_common::PhysicsProblemResponse, RequestError> {
    get_request(&format!("/physics/problem/{}", id)).await
}

pub async fn submit_physics_answer(
    req: &locus_physics_common::PhysicsSubmitRequest,
) -> Result<locus_physics_common::PhysicsSubmitResponse, RequestError> {
    post_request("/physics/submit", req).await
}

pub async fn get_physics_progress() -> Result<locus_physics_common::PhysicsProgressResponse, RequestError> {
    get_request("/physics/progress").await
}

// ============================================================================
// Profile API
// ============================================================================

pub async fn get_public_profile(username: &str) -> Result<PublicProfileResponse, RequestError> {
    get_request(&format!("/profile/{}", username)).await
}
