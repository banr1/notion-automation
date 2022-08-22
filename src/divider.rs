use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Divider {
    #[serde(flatten)]
    block_basic: BlockBasic,
    divider: (),
}
