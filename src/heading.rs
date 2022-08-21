use crate::block_basic::BlockBasic;
use crate::color::Color;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Heading1 {
    #[serde(flatten)]
    block_basic: BlockBasic,
    heading_1: HeadingContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Heading2 {
    #[serde(flatten)]
    block_basic: BlockBasic,
    heading_2: HeadingContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Heading3 {
    #[serde(flatten)]
    block_basic: BlockBasic,
    heading_3: HeadingContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct HeadingContent {
    color: Color,
    rich_text: Option<Vec<RichText>>,
}
