use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bookmark {
    #[serde(flatten)]
    block_basic: BlockBasic,
    bookmark: BookmarkContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BookmarkContent {
    url: String,
}
