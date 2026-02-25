//! Authentication module

mod api_key;
mod cookie;
mod jwt;
mod middleware;

pub use api_key::ApiKeyAuth;
pub use cookie::{build_auth_cookie, build_clear_cookie, extract_token_from_cookies};
pub use jwt::{create_token, verify_token};
pub use middleware::AuthUser;
