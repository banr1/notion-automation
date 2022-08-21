use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BlockBasic {
    id: String,
    has_children: bool,
}
