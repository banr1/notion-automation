use serde::Deserialize;

use crate::icon::Icon;

#[derive(Deserialize, Clone, Debug)]
pub struct Page {
    // https://developers.notion.com/reference/page
    // object: String,
    pub id: String,
    pub icon: Option<Icon>,
    pub url: String,
}
