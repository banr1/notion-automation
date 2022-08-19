use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Icon {
    #[serde(rename = "emoji")]
    Emoji,
    #[serde(rename = "file")]
    File,
}

#[derive(Serialize, Deserialize)]
pub struct Emoji {
    emoji: String,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    file: FileContent,
}

#[derive(Serialize, Deserialize)]
pub struct FileContent {
    url: String,
    expiry_time: String,
}
