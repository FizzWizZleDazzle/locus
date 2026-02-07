use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidateEmail;

lazy_static! {
    static ref PASSWORD_UPPERCASE: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref PASSWORD_LOWERCASE: Regex = Regex::new(r"[a-z]").unwrap();
    static ref PASSWORD_NUMBER: Regex = Regex::new(r"[0-9]").unwrap();
    static ref PASSWORD_SPECIAL: Regex = Regex::new(r"[!@#$%^&*()\-_=+\[\]{}|;:,.<>?]").unwrap();
}

#[derive(Debug)]
pub enum ValidationError {
    PasswordTooShort,
    PasswordNoUppercase,
    PasswordNoLowercase,
    PasswordNoNumber,
    PasswordNoSpecialChar,
    InvalidEmail,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::PasswordTooShort => {
                write!(f, "Password must be at least 8 characters long")
            }
            ValidationError::PasswordNoUppercase => {
                write!(f, "Password must contain at least one uppercase letter (A-Z)")
            }
            ValidationError::PasswordNoLowercase => {
                write!(f, "Password must contain at least one lowercase letter (a-z)")
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
    if password.len() < 8 {
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

/// Validates email using RFC-compliant validation
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.validate_email() {
        Ok(())
    } else {
        Err(ValidationError::InvalidEmail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_too_short() {
        assert!(matches!(
            validate_password("Pass1!"),
            Err(ValidationError::PasswordTooShort)
        ));
    }

    #[test]
    fn test_password_no_uppercase() {
        assert!(matches!(
            validate_password("password1!"),
            Err(ValidationError::PasswordNoUppercase)
        ));
    }

    #[test]
    fn test_password_no_lowercase() {
        assert!(matches!(
            validate_password("PASSWORD1!"),
            Err(ValidationError::PasswordNoLowercase)
        ));
    }

    #[test]
    fn test_password_no_number() {
        assert!(matches!(
            validate_password("Password!"),
            Err(ValidationError::PasswordNoNumber)
        ));
    }

    #[test]
    fn test_password_no_special_char() {
        assert!(matches!(
            validate_password("Password1"),
            Err(ValidationError::PasswordNoSpecialChar)
        ));
    }

    #[test]
    fn test_password_valid() {
        assert!(validate_password("Password1!").is_ok());
        assert!(validate_password("MyP@ssw0rd").is_ok());
        assert!(validate_password("Str0ng!Pass").is_ok());
        assert!(validate_password("C0mpl3x#Password").is_ok());
    }

    #[test]
    fn test_password_all_special_chars() {
        // Test various special characters
        assert!(validate_password("Pass1word!").is_ok());
        assert!(validate_password("Pass2word@").is_ok());
        assert!(validate_password("Pass3word#").is_ok());
        assert!(validate_password("Pass4word$").is_ok());
        assert!(validate_password("Pass5word%").is_ok());
        assert!(validate_password("Pass6word^").is_ok());
        assert!(validate_password("Pass7word&").is_ok());
        assert!(validate_password("Pass8word*").is_ok());
        assert!(validate_password("Pass9word(").is_ok());
        assert!(validate_password("Pass0word)").is_ok());
        assert!(validate_password("Pass1word-").is_ok());
        assert!(validate_password("Pass2word_").is_ok());
        assert!(validate_password("Pass3word=").is_ok());
        assert!(validate_password("Pass4word+").is_ok());
        assert!(validate_password("Pass5word[").is_ok());
        assert!(validate_password("Pass6word]").is_ok());
        assert!(validate_password("Pass7word{").is_ok());
        assert!(validate_password("Pass8word}").is_ok());
        assert!(validate_password("Pass9word|").is_ok());
        assert!(validate_password("Pass0word;").is_ok());
        assert!(validate_password("Pass1word:").is_ok());
        assert!(validate_password("Pass2word,").is_ok());
        assert!(validate_password("Pass3word.").is_ok());
        assert!(validate_password("Pass4word<").is_ok());
        assert!(validate_password("Pass5word>").is_ok());
        assert!(validate_password("Pass6word?").is_ok());
    }

    #[test]
    fn test_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user@domain.co.uk").is_ok());
        assert!(validate_email("name+tag@company.org").is_ok());
    }

    #[test]
    fn test_email_invalid() {
        assert!(matches!(
            validate_email("notanemail"),
            Err(ValidationError::InvalidEmail)
        ));
        assert!(matches!(
            validate_email("@nodomain.com"),
            Err(ValidationError::InvalidEmail)
        ));
        assert!(matches!(
            validate_email("double@@domain.com"),
            Err(ValidationError::InvalidEmail)
        ));
        assert!(matches!(
            validate_email(""),
            Err(ValidationError::InvalidEmail)
        ));
        assert!(matches!(
            validate_email("no-at-sign.com"),
            Err(ValidationError::InvalidEmail)
        ));
    }
}
