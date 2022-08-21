use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct File {
    pub file: FileContent,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FileContent {
    pub url: String,
    pub expiry_time: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct External {
    pub external: ExternalContent,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ExternalContent {
    pub url: String,
}
