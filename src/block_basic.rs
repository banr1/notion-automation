use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BlockBasic {
    pub id: String,
    pub has_children: bool,
    pub parent: Parent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Parent {
    #[serde(rename = "database_id")]
    DatabaseParent,
    #[serde(rename = "page_id")]
    PageParent,
    #[serde(rename = "workspace")]
    WorkspaceParent,
    #[serde(rename = "block_id")]
    BlockParent,
}
