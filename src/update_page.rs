use crate::icon::Emoji;
use crate::notion::Notion;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UpdatePageBody {
    pub properties: Option<String>,
    pub archived: Option<bool>,
    pub icon: Option<Emoji>,
    pub cover: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePageResp {
    object: String,
    id: String,
    pub url: String,
}

impl Notion {
    pub fn update_page(
        &self,
        page_id: String,
        body: &UpdatePageBody,
    ) -> Result<UpdatePageResp, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.notion.com/v1/pages/{page_id}",
            page_id = page_id,
        );
        let resp = self
            .client
            .patch(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth)
            .body(serde_json::to_string(&body).unwrap())
            .send()?
            .json()?;
        Ok(resp)
    }
}
