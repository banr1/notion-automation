use crate::block_basic::BlockBasic;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Equation {
    #[serde(flatten)]
    block_basic: BlockBasic,
    equation: EquationContent,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EquationContent {
    expression: String,
}
