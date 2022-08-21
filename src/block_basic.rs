use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BlockBasic {
    pub id: String,
    pub has_children: bool,
}
