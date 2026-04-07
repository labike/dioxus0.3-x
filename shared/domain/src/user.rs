use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(
    validate(not_empty, len_char_min = 3, len_char_max = 10),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Username(String);

#[nutype(
    validate(not_empty, len_char_min = 8, len_char_max = 15),
    derive(AsRef, Clone, Serialize, Deserialize, PartialEq)
)]
pub struct Password(String);