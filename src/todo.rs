use crate::block_basic::BlockBasic;
use crate::color::Color;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ToDo {
    #[serde(flatten)]
    block_basic: BlockBasic,
    to_do: ToDoContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ToDoContent {
    rich_text: Option<Vec<RichText>>,
    checked: bool,
    color: Color,
    // children: ...,
}
