use serde::Deserialize;

use crate::icon::Icon;

#[derive(Deserialize)]
pub struct Page {
    // https://developers.notion.com/reference/page
    object: String,
    pub id: String,
    pub icon: Icon,
    pub url: String,
}
