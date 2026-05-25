use nutype::nutype;
use crate::UserFacingError;

#[nutype(
    validate(not_empty, len_char_max = 30),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Heading(String);

impl Heading {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadingError {
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadingError::NotEmptyViolated => "Heading cannot be empty",
            HeadingError::LenCharMaxViolated => "Heading is too long, must be less than 30 chars",
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
        }
    }
}

impl Message {
    pub const MAX_CHARS: usize = 100;
}

// ------------------------------upload image caption
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
        }
    }
}

impl Caption {
    pub const MAX_CHARS: usize = 60;
}


// --------------------------------------poll
#[nutype(
    validate(not_empty, len_char_max = 50),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct PollHeading(String);

impl PollHeading {
    pub const MAX_CHARS: usize = 50;
}

impl UserFacingError for PollHeadingError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PollHeadingError::NotEmptyViolated => "Poll heading cannot be empty",
            PollHeadingError::LenCharMaxViolated => "Poll Heading is too long, must be less than 50 chars",
        }
    }
}

#[nutype(
    validate(not_empty, len_char_max = 80),
    derive(AsRef, Debug, Clone, Serialize, Deserialize, PartialEq)
)]
pub struct PollChoiceDescription(String);

impl UserFacingError for PollChoiceDescriptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PollChoiceDescriptionError::NotEmptyViolated => "PollChoiceDescriptionError cannot be empty",
            PollChoiceDescriptionError::LenCharMaxViolated => "PollChoiceDescriptionError is too long, must be less than 80 chars",
        }
    }
}

impl PollChoiceDescription {
    pub const MAX_CHARS: usize = 80;
}