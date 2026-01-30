//! Authentication module

mod jwt;
mod middleware;

pub use jwt::{create_token, Claims};
pub use middleware::AuthUser;
