use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChildPage {
    #[serde(flatten)]
    block_basic: BlockBasic,
    child_page: ChildPageContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChildPageContent {
    title: String,
}
