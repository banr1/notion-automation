use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Icon {
    #[serde(rename = "emoji")]
    Emoji(Emoji),
    #[serde(rename = "file")]
    File(File),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Emoji {
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct File {
    pub file: FileContent,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct FileContent {
    pub url: String,
    pub expiry_time: String,
}
