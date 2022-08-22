use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Embed {
    #[serde(flatten)]
    block_basic: BlockBasic,
    embed: EmbedContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedContent {
    url: String,
}
