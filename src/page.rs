use serde::Deserialize;

#[derive(Deserialize)]
pub struct Page {
    object: String,
    pub id: String,
    pub url: String,
}
