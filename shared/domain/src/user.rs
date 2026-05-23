use std::fmt;
use std::sync::OnceLock;
use nutype::nutype;
use regex::Regex;
use crate::UserFacingError;

#[nutype(
    validate(not_empty, len_char_min = 3, len_char_max = 10),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Username(String);

impl UserFacingError for UsernameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            UsernameError::NotEmptyViolated => "Username cannot be empty",
            UsernameError::LenCharMinViolated => "Username is too short, must be more than 3 chars",
            UsernameError::LenCharMaxViolated => "Username is too long, must be less than 10 chars",
        }
    }
}

#[nutype(
    validate(not_empty, len_char_min = 8, len_char_max = 15),
    derive(AsRef, Clone, Serialize, Deserialize, PartialEq)
)]
pub struct Password(String);

impl UserFacingError for PasswordError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PasswordError::NotEmptyViolated => "Password cannot be empty",
            PasswordError::LenCharMinViolated => "Password is too short, must be more than 8 chars",
            PasswordError::LenCharMaxViolated => "Password is too long, must be less than 15 chars",
        }
    }
}

#[nutype(
    validate(len_char_max = 10),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct DisplayName(String);

impl DisplayName {
    pub const MAX_CHARS: usize = 10;
}

impl UserFacingError for DisplayNameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            DisplayNameError::LenCharMaxViolated => "DisplayName is too long, must be less than 10 chars",
        }
    }
}

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail(String),
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::InvalidEmail(email) => {
                write!(f, "Invalid email: {}", email)
            }
        }
    }
}

static EMAIL_REGEX: OnceLock<EmailRegex> = OnceLock::new();

#[derive(Debug)]
pub struct EmailRegex(Regex);

impl EmailRegex {
    pub fn global() -> &'static Self {
        EMAIL_REGEX.get().expect("email regex is not initialized")
    }

    pub fn init() -> Self {
        Self(regex::Regex::new(r#"^\S+@\S+\.\S{1,64}$"#).unwrap())
    }
    pub fn is_valid<T: AsRef<str>>(&self, text: T) -> bool {
        self.0.is_match(text.as_ref())
    }
}

fn is_valid_email(email: &str) -> Result<(), EmailError> {
    let email_regex = EMAIL_REGEX.get_or_init(EmailRegex::init);
    if email_regex.is_valid(email) {
        Ok(())
    } else {
        Err(EmailError::InvalidEmail(email.to_string()))
    }
}

#[nutype(
    validate(with = is_valid_email, error = EmailError),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Email(String);

impl UserFacingError for EmailError {
    fn formatted_error(&self) -> &'static str {
        match self {
            EmailError::InvalidEmail(_) => "Email is not valid, Format: your_name@explame.com"
        }
    }
}