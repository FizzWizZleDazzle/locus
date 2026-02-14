//! OAuth authentication (Google + GitHub)

use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::{
    auth::create_token,
    models::{User, OAuthAccount},
    AppError,
};
use super::AppState;

// ============================================================================
// CSRF State JWT
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct OAuthStateClaims {
    /// Provider name
    provider: String,
    /// Expiry
    exp: i64,
    /// Issued at
    iat: i64,
}

fn create_state_token(provider: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = OAuthStateClaims {
        provider: provider.to_string(),
        exp: now + 600, // 10 minutes
        iat: now,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| AppError::Internal(format!("Failed to create OAuth state: {}", e)))
}

fn verify_state_token(token: &str, expected_provider: &str, secret: &str) -> Result<(), AppError> {
    let claims = decode::<OAuthStateClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::BadRequest("Invalid or expired OAuth state".into()))?
    .claims;

    if claims.provider != expected_provider {
        return Err(AppError::BadRequest("OAuth state provider mismatch".into()));
    }
    Ok(())
}

// ============================================================================
// Redirect endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

pub async fn oauth_redirect(
    State(state): State<AppState>,
    Path(provider): Path<String>,
) -> Result<Html<String>, AppError> {
    let url = match provider.as_str() {
        "google" => google_auth_url(state).await?,
        "github" => github_auth_url(state).await?,
        _ => return Err(AppError::BadRequest(format!("Unknown OAuth provider: {}", provider))),
    };
    // Use client-side redirect so dev proxies don't follow the redirect server-side
    Ok(Html(format!(
        r#"<!DOCTYPE html><html><head><meta http-equiv="refresh" content="0;url={url}"></head><body><script>window.location.href="{url}";</script></body></html>"#,
        url = url
    )))
}

pub async fn oauth_callback(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(params): Query<CallbackParams>,
) -> Result<Html<String>, AppError> {
    if let Some(error) = &params.error {
        return Ok(build_callback_html_error(error));
    }

    let code = params.code.as_deref()
        .ok_or_else(|| AppError::BadRequest("Missing authorization code".into()))?;
    let csrf_state = params.state.as_deref()
        .ok_or_else(|| AppError::BadRequest("Missing state parameter".into()))?;

    verify_state_token(csrf_state, &provider, &state.jwt_secret)?;

    match provider.as_str() {
        "google" => google_callback(state, code).await,
        "github" => github_callback(state, code).await,
        _ => Err(AppError::BadRequest(format!("Unknown OAuth provider: {}", provider))),
    }
}

// ============================================================================
// Google OAuth
// ============================================================================

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    name: Option<String>,
}

async fn google_auth_url(state: AppState) -> Result<String, AppError> {
    let client_id = state.google_client_id.as_deref()
        .ok_or_else(|| AppError::BadRequest("Google OAuth not configured".into()))?;

    let csrf = create_state_token("google", &state.jwt_secret)?;
    let redirect_uri = format!("{}/api/auth/oauth/google/callback", state.oauth_redirect_base);

    Ok(format!(
        "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        urlencoding::encode(client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode("openid email profile"),
        urlencoding::encode(&csrf),
    ))
}

async fn google_callback(state: AppState, code: &str) -> Result<Html<String>, AppError> {
    let client_id = state.google_client_id.as_deref()
        .ok_or_else(|| AppError::Internal("Google OAuth not configured".into()))?;
    let client_secret = state.google_client_secret.as_deref()
        .ok_or_else(|| AppError::Internal("Google OAuth not configured".into()))?;

    let redirect_uri = format!("{}/api/auth/oauth/google/callback", state.oauth_redirect_base);

    // Exchange code for tokens
    let token_resp: GoogleTokenResponse = state.http_client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", &redirect_uri),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Google token exchange failed: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Google token parse failed: {}", e)))?;

    // Fetch user info
    let user_info: GoogleUserInfo = state.http_client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&token_resp.access_token)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Google userinfo failed: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Google userinfo parse failed: {}", e)))?;

    let display_name = user_info.name.as_deref().unwrap_or("");
    oauth_login_or_register(
        &state,
        "google",
        &user_info.id,
        &user_info.email,
        display_name,
    )
    .await
}

// ============================================================================
// GitHub OAuth
// ============================================================================

#[derive(Debug, Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: i64,
    login: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

async fn github_auth_url(state: AppState) -> Result<String, AppError> {
    let client_id = state.github_client_id.as_deref()
        .ok_or_else(|| AppError::BadRequest("GitHub OAuth not configured".into()))?;

    let csrf = create_state_token("github", &state.jwt_secret)?;
    let redirect_uri = format!("{}/api/auth/oauth/github/callback", state.oauth_redirect_base);

    Ok(format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}",
        urlencoding::encode(client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode("read:user user:email"),
        urlencoding::encode(&csrf),
    ))
}

async fn github_callback(state: AppState, code: &str) -> Result<Html<String>, AppError> {
    let client_id = state.github_client_id.as_deref()
        .ok_or_else(|| AppError::Internal("GitHub OAuth not configured".into()))?;
    let client_secret = state.github_client_secret.as_deref()
        .ok_or_else(|| AppError::Internal("GitHub OAuth not configured".into()))?;

    // Exchange code for token
    let token_resp: GitHubTokenResponse = state.http_client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("code", code),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("GitHub token exchange failed: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("GitHub token parse failed: {}", e)))?;

    // Fetch user info
    let gh_user: GitHubUser = state.http_client
        .get("https://api.github.com/user")
        .header("User-Agent", "Locus")
        .bearer_auth(&token_resp.access_token)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("GitHub user fetch failed: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("GitHub user parse failed: {}", e)))?;

    // Fetch emails
    let emails: Vec<GitHubEmail> = state.http_client
        .get("https://api.github.com/user/emails")
        .header("User-Agent", "Locus")
        .bearer_auth(&token_resp.access_token)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("GitHub emails fetch failed: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("GitHub emails parse failed: {}", e)))?;

    // Pick primary verified email
    let email = emails.iter()
        .find(|e| e.primary && e.verified)
        .or_else(|| emails.iter().find(|e| e.verified))
        .map(|e| e.email.clone())
        .ok_or_else(|| AppError::BadRequest("No verified email found on GitHub account".into()))?;

    let display_name = gh_user.name.as_deref()
        .unwrap_or(&gh_user.login);

    oauth_login_or_register(
        &state,
        "github",
        &gh_user.id.to_string(),
        &email,
        display_name,
    )
    .await
}

// ============================================================================
// Shared OAuth logic
// ============================================================================

async fn oauth_login_or_register(
    state: &AppState,
    provider: &str,
    provider_user_id: &str,
    email: &str,
    display_name: &str,
) -> Result<Html<String>, AppError> {
    // 1. Check if OAuth account already exists → login
    if let Some(oauth_account) = OAuthAccount::find_by_provider(&state.pool, provider, provider_user_id).await? {
        let user = User::find_by_id(&state.pool, oauth_account.user_id)
            .await?
            .ok_or_else(|| AppError::Internal("OAuth linked user not found".into()))?;

        let token = create_token(user.id, &user.username, &state.jwt_secret, 24)
            .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

        let profile = user.to_profile(&state.pool).await?;
        return Ok(build_callback_html_success(&token, &profile));
    }

    // 2. SECURITY FIX: Block auto-linking if email already exists
    if User::find_by_email(&state.pool, email).await?.is_some() {
        return Ok(build_callback_html_error(
            &format!(
                "An account with email {} already exists. Please log in with your password first, \
                then link your {} account in Settings.",
                email, provider
            )
        ));
    }

    // 3. Create new user (email doesn't exist yet)
    let username = generate_unique_username(&state.pool, display_name, email).await?;
    let user = User::create_oauth(&state.pool, &username, email).await?;
    OAuthAccount::create(&state.pool, user.id, provider, provider_user_id, Some(email)).await?;

    let token = create_token(user.id, &user.username, &state.jwt_secret, 24)
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

    let profile = user.to_profile(&state.pool).await?;
    Ok(build_callback_html_success(&token, &profile))
}

/// Generate a unique username from display name or email prefix
async fn generate_unique_username(pool: &sqlx::PgPool, display_name: &str, email: &str) -> Result<String, AppError> {
    // Sanitize: take name or email prefix, keep alphanumeric + underscore
    let base = if !display_name.is_empty() {
        display_name.to_string()
    } else {
        email.split('@').next().unwrap_or("user").to_string()
    };

    let sanitized: String = base.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();

    // Truncate to leave room for suffix
    let sanitized = if sanitized.len() > 40 {
        sanitized[..40].to_string()
    } else if sanitized.is_empty() {
        "user".to_string()
    } else {
        sanitized
    };

    // Try base name first
    if sanitized.len() >= 3 && !User::username_exists(pool, &sanitized).await? {
        return Ok(sanitized);
    }

    // Append random suffix
    for _ in 0..10 {
        let suffix: u32 = rand::random_range(100..10000);
        let candidate = format!("{}_{}", sanitized, suffix);
        if !User::username_exists(pool, &candidate).await? {
            return Ok(candidate);
        }
    }

    Err(AppError::Internal("Failed to generate unique username".into()))
}

// ============================================================================
// Callback HTML (postMessage to opener)
// ============================================================================

fn build_callback_html_success(token: &str, profile: &locus_common::UserProfile) -> Html<String> {
    let auth_json = serde_json::json!({
        "token": token,
        "user": profile,
    });

    Html(format!(
        r#"<!DOCTYPE html>
<html><head><title>OAuth</title></head><body>
<script>
  if (window.opener) {{
    window.opener.postMessage({{
      type: "oauth_success",
      data: {}
    }}, "*");
  }}
  window.close();
</script>
<p>Sign-in successful. This window should close automatically.</p>
</body></html>"#,
        auth_json
    ))
}

fn build_callback_html_error(error: &str) -> Html<String> {
    let escaped = error.replace('\\', "\\\\").replace('"', "\\\"");
    Html(format!(
        r#"<!DOCTYPE html>
<html><head><title>OAuth Error</title></head><body>
<script>
  if (window.opener) {{
    window.opener.postMessage({{
      type: "oauth_error",
      error: "{}"
    }}, "*");
  }}
  window.close();
</script>
<p>Sign-in failed: {}. This window should close automatically.</p>
</body></html>"#,
        escaped, error
    ))
}
