//! HTTP API client

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use locus_common::{
    AuthResponse, LeaderboardResponse, LoginRequest, ProblemResponse, RegisterRequest,
    SetPasswordRequest, SubmitRequest, SubmitResponse, UserProfile, ApiError,
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

// API base URL - configurable at compile time
// Production: https://api.locusmath.org/api
// Local dev: /api (same domain)
const API_BASE: &str = match option_env!("LOCUS_API_URL") {
    Some(url) => url,
    None => "https://api.locusmath.org/api",
};

const TOKEN_KEY: &str = "locus_token";
const USERNAME_KEY: &str = "locus_username";

/// Check if user is logged in (has stored token)
pub fn is_logged_in() -> bool {
    LocalStorage::get::<String>(TOKEN_KEY).is_ok()
}

/// Get stored username
pub fn get_stored_username() -> Option<String> {
    LocalStorage::get::<String>(USERNAME_KEY).ok()
}

/// Get stored token
fn get_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_KEY).ok()
}

/// Store auth data (internal)
fn store_auth(token: &str, username: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token);
    let _ = LocalStorage::set(USERNAME_KEY, username);
}

/// Store auth data from OAuth popup result
pub fn store_oauth_auth(token: &str, username: &str) {
    store_auth(token, username);
}

/// Clear auth data
pub fn clear_auth() {
    LocalStorage::delete(TOKEN_KEY);
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

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Make an authenticated GET request
async fn get_request<T: DeserializeOwned>(path: &str) -> Result<T, RequestError> {
    let url = format!("{}{}", API_BASE, path);

    let mut req = Request::get(&url);

    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let resp = req.send().await.map_err(|e| RequestError {
        message: format!("Network error: {}", e),
    })?;

    if resp.ok() {
        resp.json::<T>().await.map_err(|e| RequestError {
            message: format!("Parse error: {}", e),
        })
    } else {
        let error: ApiError = resp.json().await.unwrap_or(ApiError::new("Unknown error"));
        Err(RequestError { message: error.error })
    }
}

/// Make an authenticated POST request with JSON body
async fn post_request<T: DeserializeOwned, B: Serialize>(path: &str, body: &B) -> Result<T, RequestError> {
    let url = format!("{}{}", API_BASE, path);

    let mut req = Request::post(&url)
        .header("Content-Type", "application/json");

    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let req = req
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
        let error: ApiError = resp.json().await.unwrap_or(ApiError::new("Unknown error"));
        Err(RequestError { message: error.error })
    }
}

// ============================================================================
// Auth API
// ============================================================================

pub async fn register(username: &str, email: &str, password: &str) -> Result<RegisterResponse, RequestError> {
    let req = RegisterRequest {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
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

pub async fn login(email: &str, password: &str) -> Result<AuthResponse, RequestError> {
    let req = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let resp: AuthResponse = post_request("/auth/login", &req).await?;
    store_auth(&resp.token, &resp.user.username);
    Ok(resp)
}

pub fn logout() {
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
