use crate::block_basic::BlockBasic;
use crate::color::Color;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BulletedListItem {
    #[serde(flatten)]
    block_basic: BlockBasic,
    bulleted_list_item: BulletedListItemContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BulletedListItemContent {
    rich_text: Option<Vec<RichText>>,
    color: Color,
    // children: ...,
}
