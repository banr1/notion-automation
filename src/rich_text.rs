use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum RichText {
    Text,
    Mention,
    Equation,
}
