use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SyncedBlock {
    id: String,
    has_children: bool,
    synced_block: SyncedBlockContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct SyncedBlockContent {
    synced_from: Option<String>,
    rich_text: Option<Vec<RichText>>,
}
