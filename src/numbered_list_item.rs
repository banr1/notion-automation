use crate::block_basic::BlockBasic;
use crate::color::Color;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NumberedListItem {
    #[serde(flatten)]
    block_basic: BlockBasic,
    numbered_list_item: NumberedListItemContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NumberedListItemContent {
    rich_text: Option<Vec<RichText>>,
    color: Color,
    // children: ...,
}
