use nutype::nutype;
use serde::{Deserialize, Serialize};
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
            _ => {""}
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
            _ => ""
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
            _ => {""}
        }
    }
}