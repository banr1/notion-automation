use crate::block_basic::BlockBasic;
use crate::color::Color;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Toggle {
    #[serde(flatten)]
    block_basic: BlockBasic,
    toggle: ToggleContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ToggleContent {
    rich_text: Option<Vec<RichText>>,
    color: Color,
    // children: ...,
}
