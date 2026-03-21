//! Cookie helpers for JWT authentication

pub const COOKIE_NAME: &str = "locus_token";

/// Build a Set-Cookie header value for authentication
pub fn build_auth_cookie(token: &str, max_age_hours: i64, secure: bool, cookie_domain: Option<&str>) -> String {
    let max_age = max_age_hours * 3600;
    let secure_flag = if secure { "; Secure" } else { "" };
    let domain_flag = cookie_domain.map_or(String::new(), |d| format!("; Domain={}", d));
    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/api; Max-Age={}{}{}",
        COOKIE_NAME, token, max_age, secure_flag, domain_flag
    )
}

/// Build a Set-Cookie header value that clears the auth cookie
pub fn build_clear_cookie(secure: bool, cookie_domain: Option<&str>) -> String {
    let secure_flag = if secure { "; Secure" } else { "" };
    let domain_flag = cookie_domain.map_or(String::new(), |d| format!("; Domain={}", d));
    format!(
        "{}=; HttpOnly; SameSite=Lax; Path=/api; Max-Age=0{}{}",
        COOKIE_NAME, secure_flag, domain_flag
    )
}

/// Extract the locus_token value from a Cookie header string
pub fn extract_token_from_cookies(cookie_header: &str) -> Option<&str> {
    cookie_header.split(';').map(|s| s.trim()).find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        if name.trim() == COOKIE_NAME {
            Some(value.trim())
        } else {
            None
        }
    })
}
