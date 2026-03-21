use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

const API_URL: &str = env!("COMMUNITY_API_URL");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: i32,
    pub user_id: Uuid,
    pub username: String,
    pub category: String,
    pub title: String,
    pub body: String,
    pub upvotes: i32,
    pub comment_count: i32,
    pub status: String,
    pub pinned: bool,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumComment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Uuid,
    pub username: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostListResponse {
    pub posts: Vec<ForumPost>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDetailResponse {
    pub post: ForumPost,
    pub comments: Vec<ForumComment>,
    pub user_voted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResponse {
    pub voted: bool,
    pub upvotes: i32,
}

fn creds() -> web_sys::RequestCredentials {
    web_sys::RequestCredentials::Include
}

pub async fn logout() -> Result<(), String> {
    Request::post(&format!("{API_URL}/auth/logout"))
        .credentials(creds())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_me() -> Result<UserInfo, String> {
    let resp = Request::get(&format!("{API_URL}/auth/me"))
        .credentials(creds())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err("Not logged in".to_string());
    }

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn list_posts(
    category: Option<&str>,
    status: Option<&str>,
    sort: Option<&str>,
    page: i64,
    search: Option<&str>,
) -> Result<PostListResponse, String> {
    let mut url = format!("{API_URL}/forum/posts?page={page}");
    if let Some(c) = category { url.push_str(&format!("&category={c}")); }
    if let Some(s) = status { url.push_str(&format!("&status={s}")); }
    if let Some(s) = sort { url.push_str(&format!("&sort={s}")); }
    if let Some(s) = search {
        let s = s.trim();
        if !s.is_empty() {
            url.push_str(&format!("&search={}", js_sys::encode_uri_component(s)));
        }
    }

    let resp = Request::get(&url)
        .credentials(creds())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_post(id: i32) -> Result<PostDetailResponse, String> {
    let resp = Request::get(&format!("{API_URL}/forum/posts/{id}"))
        .credentials(creds())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err("Post not found".to_string());
    }

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn create_post(category: &str, title: &str, body: &str) -> Result<ForumPost, String> {
    let resp = Request::post(&format!("{API_URL}/forum/posts"))
        .credentials(creds())
        .json(&serde_json::json!({ "category": category, "title": title, "body": body }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        let text = resp.text().await.unwrap_or_default();
        return Err(text);
    }

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn add_comment(post_id: i32, body: &str) -> Result<ForumComment, String> {
    let resp = Request::post(&format!("{API_URL}/forum/posts/{post_id}/comment"))
        .credentials(creds())
        .json(&serde_json::json!({ "body": body }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        let text = resp.text().await.unwrap_or_default();
        return Err(text);
    }

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn toggle_vote(post_id: i32) -> Result<VoteResponse, String> {
    let resp = Request::post(&format!("{API_URL}/forum/posts/{post_id}/vote"))
        .credentials(creds())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json().await.map_err(|e| e.to_string())
}
