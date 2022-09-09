use serde::Deserialize;

use crate::icon::Icon;
use crate::object::Object;
use crate::property::Properties;

#[derive(Deserialize, Clone, Debug)]
pub struct Page {
    // https://developers.notion.com/reference/page
    pub object: Object,
    pub id: String,
    pub properties: Properties,
    pub icon: Option<Icon>,
    pub url: String,
}
