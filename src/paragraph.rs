use crate::block_basic::BlockBasic;
use crate::color::Color;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Paragraph {
    #[serde(flatten)]
    block_basic: BlockBasic,
    paragraph: ParagraphContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct ParagraphContent {
    color: Color,
    rich_text: Option<Vec<RichText>>,
    // children: Option<Vec<Object>>,
}
