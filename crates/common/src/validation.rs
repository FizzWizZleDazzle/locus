//! Validation logic shared between frontend and backend
//!
//! Frontend uses this for client-side validation (better UX).
//! Backend uses this as the authoritative validation layer.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PASSWORD_UPPERCASE: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref PASSWORD_LOWERCASE: Regex = Regex::new(r"[a-z]").unwrap();
    static ref PASSWORD_NUMBER: Regex = Regex::new(r"[0-9]").unwrap();
    static ref PASSWORD_SPECIAL: Regex = Regex::new(r"[!@#$%^&*()\-_=+\[\]{}|;:,.<>?]").unwrap();
    static ref USERNAME_PATTERN: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

/// Minimum username length
pub const MIN_USERNAME_LEN: usize = 3;

/// Maximum username length
pub const MAX_USERNAME_LEN: usize = 50;

/// Minimum password length
pub const MIN_PASSWORD_LEN: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    PasswordTooShort,
    PasswordNoUppercase,
    PasswordNoLowercase,
    PasswordNoNumber,
    PasswordNoSpecialChar,
    InvalidEmail,
    UsernameTooShort,
    UsernameTooLong,
    UsernameInvalidChars,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::PasswordTooShort => {
                write!(
                    f,
                    "Password must be at least {} characters long",
                    MIN_PASSWORD_LEN
                )
            }
            ValidationError::PasswordNoUppercase => {
                write!(
                    f,
                    "Password must contain at least one uppercase letter (A-Z)"
                )
            }
            ValidationError::PasswordNoLowercase => {
                write!(
                    f,
                    "Password must contain at least one lowercase letter (a-z)"
                )
            }
            ValidationError::PasswordNoNumber => {
                write!(f, "Password must contain at least one number (0-9)")
            }
            ValidationError::PasswordNoSpecialChar => {
                write!(
                    f,
                    "Password must contain at least one special character (!@#$%^&*()_+-=[]{{}}|;:,.<>?)"
                )
            }
            ValidationError::InvalidEmail => {
                write!(f, "Invalid email address format")
            }
            ValidationError::UsernameTooShort => {
                write!(
                    f,
                    "Username must be at least {} characters long",
                    MIN_USERNAME_LEN
                )
            }
            ValidationError::UsernameTooLong => {
                write!(
                    f,
                    "Username must be at most {} characters long",
                    MAX_USERNAME_LEN
                )
            }
            ValidationError::UsernameInvalidChars => {
                write!(
                    f,
                    "Username can only contain letters, numbers, underscores, and hyphens"
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates password meets security requirements:
/// - At least 8 characters
/// - At least one uppercase letter (A-Z)
/// - At least one lowercase letter (a-z)
/// - At least one number (0-9)
/// - At least one special character (!@#$%^&*()_+-=[]{}|;:,.<>?)
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < MIN_PASSWORD_LEN {
        return Err(ValidationError::PasswordTooShort);
    }

    if !PASSWORD_UPPERCASE.is_match(password) {
        return Err(ValidationError::PasswordNoUppercase);
    }

    if !PASSWORD_LOWERCASE.is_match(password) {
        return Err(ValidationError::PasswordNoLowercase);
    }

    if !PASSWORD_NUMBER.is_match(password) {
        return Err(ValidationError::PasswordNoNumber);
    }

    if !PASSWORD_SPECIAL.is_match(password) {
        return Err(ValidationError::PasswordNoSpecialChar);
    }

    Ok(())
}

/// Validates email using a simple but effective regex
/// (Full RFC compliance is not necessary for most applications)
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    // Basic email validation: has @ symbol, domain, and TLD
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(ValidationError::InvalidEmail)
    }
}

/// Validates username meets requirements:
/// - Between 3 and 50 characters
/// - Only letters, numbers, underscores, and hyphens
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < MIN_USERNAME_LEN {
        return Err(ValidationError::UsernameTooShort);
    }

    if username.len() > MAX_USERNAME_LEN {
        return Err(ValidationError::UsernameTooLong);
    }

    if !USERNAME_PATTERN.is_match(username) {
        return Err(ValidationError::UsernameInvalidChars);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_too_short() {
        assert_eq!(
            validate_password("Pass1!"),
            Err(ValidationError::PasswordTooShort)
        );
    }

    #[test]
    fn test_password_no_uppercase() {
        assert_eq!(
            validate_password("password1!"),
            Err(ValidationError::PasswordNoUppercase)
        );
    }

    #[test]
    fn test_password_no_lowercase() {
        assert_eq!(
            validate_password("PASSWORD1!"),
            Err(ValidationError::PasswordNoLowercase)
        );
    }

    #[test]
    fn test_password_no_number() {
        assert_eq!(
            validate_password("Password!"),
            Err(ValidationError::PasswordNoNumber)
        );
    }

    #[test]
    fn test_password_no_special_char() {
        assert_eq!(
            validate_password("Password1"),
            Err(ValidationError::PasswordNoSpecialChar)
        );
    }

    #[test]
    fn test_password_valid() {
        assert!(validate_password("Password1!").is_ok());
        assert!(validate_password("MyP@ssw0rd").is_ok());
        assert!(validate_password("Str0ng!Pass").is_ok());
        assert!(validate_password("C0mpl3x#Password").is_ok());
    }

    #[test]
    fn test_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user@domain.co.uk").is_ok());
        assert!(validate_email("name+tag@company.org").is_ok());
    }

    #[test]
    fn test_email_invalid() {
        assert_eq!(
            validate_email("notanemail"),
            Err(ValidationError::InvalidEmail)
        );
        assert_eq!(
            validate_email("@nodomain.com"),
            Err(ValidationError::InvalidEmail)
        );
        assert_eq!(
            validate_email("double@@domain.com"),
            Err(ValidationError::InvalidEmail)
        );
        assert_eq!(validate_email(""), Err(ValidationError::InvalidEmail));
        assert_eq!(
            validate_email("no-at-sign.com"),
            Err(ValidationError::InvalidEmail)
        );
    }

    #[test]
    fn test_username_valid() {
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("my_user").is_ok());
        assert!(validate_username("user-name").is_ok());
        assert!(validate_username("ABC").is_ok());
    }

    #[test]
    fn test_username_too_short() {
        assert_eq!(
            validate_username("ab"),
            Err(ValidationError::UsernameTooShort)
        );
    }

    #[test]
    fn test_username_too_long() {
        let long_name = "a".repeat(51);
        assert_eq!(
            validate_username(&long_name),
            Err(ValidationError::UsernameTooLong)
        );
    }

    #[test]
    fn test_username_invalid_chars() {
        assert_eq!(
            validate_username("user@123"),
            Err(ValidationError::UsernameInvalidChars)
        );
        assert_eq!(
            validate_username("user name"),
            Err(ValidationError::UsernameInvalidChars)
        );
        assert_eq!(
            validate_username("user!"),
            Err(ValidationError::UsernameInvalidChars)
        );
    }
}
