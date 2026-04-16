use nutype::nutype;
use serde::{Deserialize, Serialize};
use crate::UserFacingError;

#[nutype(
    validate(not_empty, len_char_min = 1, len_char_max = 30),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Heading(String);

impl UserFacingError for HeadingError {
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadingError::NotEmptyViolated => "Heading cannot be empty",
            HeadingError::LenCharMinViolated => "Heading is too short, must be more than 1 chars",
            HeadingError::LenCharMaxViolated => "Heading is too long, must be less than 30 chars",
            _ => {""}
        }
    }
}

#[nutype(
    validate(not_empty, len_char_min = 1, len_char_max = 100),
    derive(AsRef, Debug, Clone, Serialize, Deserialize, PartialEq)
)]
pub struct Message(String);

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::NotEmptyViolated => "Message cannot be empty",
            MessageError::LenCharMinViolated => "Message is too short, must be more than 1 chars",
            MessageError::LenCharMaxViolated => "Message is too long, must be less than 100 chars",
            _ => ""
        }
    }
}