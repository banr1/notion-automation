use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Page,
}
