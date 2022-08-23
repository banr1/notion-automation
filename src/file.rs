use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct File {
    pub file: FileContent,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FileContent {
    pub url: String,
    pub expiry_time: Option<String>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct External {
    pub external: ExternalContent,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ExternalContent {
    pub url: String,
}
