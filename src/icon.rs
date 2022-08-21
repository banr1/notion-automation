use serde::{Deserialize, Serialize};

use crate::file::External;
use crate::file::File;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Icon {
    Emoji(Emoji),
    File(File),
    External(External),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Emoji {
    pub emoji: String,
}
