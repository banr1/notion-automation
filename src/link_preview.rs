use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct LinkPreview {
    #[serde(flatten)]
    block_basic: BlockBasic,
    link_preview: LinkPreviewContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct LinkPreviewContent {
    url: String,
}
