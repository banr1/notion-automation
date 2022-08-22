use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChildDatabase {
    #[serde(flatten)]
    block_basic: BlockBasic,
    child_database: ChildDatabaseContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChildDatabaseContent {
    title: String,
}
