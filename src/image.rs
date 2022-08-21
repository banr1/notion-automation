use crate::block_basic::BlockBasic;
use crate::file::{External, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Image {
    #[serde(flatten)]
    pub block_basic: BlockBasic,
    pub image: ImageContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum ImageContent {
    File(File),
    External(External),
}
