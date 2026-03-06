//! Authentication module

mod cookie;
mod jwt;
mod middleware;

pub use cookie::{build_auth_cookie, build_clear_cookie, extract_token_from_cookies};
pub use jwt::{create_token, verify_token};
pub use middleware::AuthUser;
