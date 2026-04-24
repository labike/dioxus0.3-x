use nutype::nutype;
use serde::{Deserialize, Serialize};
use crate::UserFacingError;

#[nutype(
    validate(len_char_max = 30),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Heading(String);

impl Heading {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadingError {
    fn formatted_error(&self) -> &'static str {
        match self {
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

impl Message {
    pub const MAX_CHARS: usize = 100;
}

#[nutype(
    validate(not_empty, len_char_max = 60),
    derive(AsRef, Debug, Clone, Serialize, Deserialize, PartialEq)
)]
pub struct Caption(String);

impl UserFacingError for CaptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            CaptionError::NotEmptyViolated => "Caption cannot be empty",
            CaptionError::LenCharMaxViolated => "Caption is too long, must be less than 60 chars",
            _ => ""
        }
    }
}

impl Caption {
    pub const MAX_CHARS: usize = 60;
}