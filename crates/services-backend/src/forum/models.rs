use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ForumComment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Uuid,
    pub username: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

// Queries

pub async fn list_posts(
    pool: &PgPool,
    category: Option<&str>,
    status: Option<&str>,
    sort: &str,
    page: i64,
    search: Option<&str>,
) -> Result<(Vec<ForumPost>, bool), sqlx::Error> {
    let limit = 20i64;
    let offset = (page - 1) * limit;
    let order = if sort == "top" { "upvotes DESC" } else { "created_at DESC" };

    // Build query dynamically based on filters
    let mut query = String::from("SELECT * FROM forum_posts WHERE 1=1");
    let mut params: Vec<String> = Vec::new();

    if let Some(cat) = category {
        params.push(cat.to_string());
        query.push_str(&format!(" AND category = ${}", params.len()));
    }
    if let Some(st) = status {
        params.push(st.to_string());
        query.push_str(&format!(" AND status = ${}", params.len()));
    }
    if let Some(s) = search {
        let s = s.trim();
        if !s.is_empty() {
            params.push(s.to_string());
            let idx = params.len();
            query.push_str(&format!(
                " AND (title ILIKE '%' || ${idx} || '%' OR body ILIKE '%' || ${idx} || '%')"
            ));
        }
    }

    // Pinned first, then by sort order; fetch limit+1 to check has_more
    let fetch_limit = limit + 1;
    query.push_str(&format!(
        " ORDER BY pinned DESC, {} LIMIT ${} OFFSET ${}",
        order,
        params.len() + 1,
        params.len() + 2
    ));

    let mut q = sqlx::query_as::<_, ForumPost>(&query);
    for p in &params {
        q = q.bind(p);
    }
    q = q.bind(fetch_limit).bind(offset);

    let mut posts = q.fetch_all(pool).await?;
    let has_more = posts.len() > limit as usize;
    if has_more {
        posts.truncate(limit as usize);
    }

    Ok((posts, has_more))
}

pub async fn get_post(pool: &PgPool, post_id: i32) -> Result<Option<ForumPost>, sqlx::Error> {
    sqlx::query_as::<_, ForumPost>("SELECT * FROM forum_posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_post(
    pool: &PgPool,
    user_id: Uuid,
    username: &str,
    category: &str,
    title: &str,
    body: &str,
) -> Result<ForumPost, sqlx::Error> {
    sqlx::query_as::<_, ForumPost>(
        "INSERT INTO forum_posts (user_id, username, category, title, body)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(user_id)
    .bind(username)
    .bind(category)
    .bind(title)
    .bind(body)
    .fetch_one(pool)
    .await
}

pub async fn get_comments(pool: &PgPool, post_id: i32) -> Result<Vec<ForumComment>, sqlx::Error> {
    sqlx::query_as::<_, ForumComment>(
        "SELECT * FROM forum_comments WHERE post_id = $1 ORDER BY created_at ASC"
    )
    .bind(post_id)
    .fetch_all(pool)
    .await
}

pub async fn create_comment(
    pool: &PgPool,
    post_id: i32,
    user_id: Uuid,
    username: &str,
    body: &str,
) -> Result<ForumComment, sqlx::Error> {
    let comment = sqlx::query_as::<_, ForumComment>(
        "INSERT INTO forum_comments (post_id, user_id, username, body)
         VALUES ($1, $2, $3, $4)
         RETURNING *"
    )
    .bind(post_id)
    .bind(user_id)
    .bind(username)
    .bind(body)
    .fetch_one(pool)
    .await?;

    // Update comment count
    sqlx::query("UPDATE forum_posts SET comment_count = comment_count + 1, updated_at = NOW() WHERE id = $1")
        .bind(post_id)
        .execute(pool)
        .await?;

    Ok(comment)
}

pub async fn toggle_vote(pool: &PgPool, user_id: Uuid, post_id: i32) -> Result<bool, sqlx::Error> {
    // Check if vote exists
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM forum_votes WHERE user_id = $1 AND post_id = $2)"
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await?;

    if exists {
        // Remove vote
        sqlx::query("DELETE FROM forum_votes WHERE user_id = $1 AND post_id = $2")
            .bind(user_id)
            .bind(post_id)
            .execute(pool)
            .await?;
        sqlx::query("UPDATE forum_posts SET upvotes = upvotes - 1 WHERE id = $1")
            .bind(post_id)
            .execute(pool)
            .await?;
        Ok(false) // vote removed
    } else {
        // Add vote
        sqlx::query("INSERT INTO forum_votes (user_id, post_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(post_id)
            .execute(pool)
            .await?;
        sqlx::query("UPDATE forum_posts SET upvotes = upvotes + 1 WHERE id = $1")
            .bind(post_id)
            .execute(pool)
            .await?;
        Ok(true) // vote added
    }
}

pub async fn has_voted(pool: &PgPool, user_id: Uuid, post_id: i32) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM forum_votes WHERE user_id = $1 AND post_id = $2)"
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await
}

pub async fn update_post_status(pool: &PgPool, post_id: i32, status: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE forum_posts SET status = $1, updated_at = NOW() WHERE id = $2")
        .bind(status)
        .bind(post_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn toggle_pin(pool: &PgPool, post_id: i32) -> Result<bool, sqlx::Error> {
    let pinned = sqlx::query_scalar::<_, bool>(
        "UPDATE forum_posts SET pinned = NOT pinned, updated_at = NOW() WHERE id = $1 RETURNING pinned"
    )
    .bind(post_id)
    .fetch_one(pool)
    .await?;
    Ok(pinned)
}

pub async fn toggle_lock(pool: &PgPool, post_id: i32) -> Result<bool, sqlx::Error> {
    let locked = sqlx::query_scalar::<_, bool>(
        "UPDATE forum_posts SET locked = NOT locked, updated_at = NOW() WHERE id = $1 RETURNING locked"
    )
    .bind(post_id)
    .fetch_one(pool)
    .await?;
    Ok(locked)
}

pub async fn delete_comment(pool: &PgPool, comment_id: i32) -> Result<(), sqlx::Error> {
    // Get post_id before deleting
    let post_id = sqlx::query_scalar::<_, i32>(
        "DELETE FROM forum_comments WHERE id = $1 RETURNING post_id"
    )
    .bind(comment_id)
    .fetch_one(pool)
    .await?;

    sqlx::query("UPDATE forum_posts SET comment_count = GREATEST(comment_count - 1, 0) WHERE id = $1")
        .bind(post_id)
        .execute(pool)
        .await?;

    Ok(())
}
