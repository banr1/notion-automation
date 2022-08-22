use crate::block_basic::BlockBasic;
use crate::color::Color;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TableOfContents {
    #[serde(flatten)]
    block_basic: BlockBasic,
    table_of_contents: TableOfContentsContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TableOfContentsContent {
    color: Color,
}
