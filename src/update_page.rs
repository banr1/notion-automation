use crate::icon::Emoji;
use crate::notion::Notion;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SelectOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct MultiSelectOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub enum Property {
    Vertical {
        multi_select: Vec<MultiSelectOption>,
    },
    Horizontal {
        multi_select: Vec<MultiSelectOption>,
    },
    External {
        multi_select: Vec<MultiSelectOption>,
    },
    Version {
        select: SelectOption,
    },
    Temporary {
        multi_select: Vec<MultiSelectOption>,
    },
}

#[derive(Serialize)]
pub struct UpdatePageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Property>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Emoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePageResp {
    // object: String,
    pub id: String,
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
        let body_ = serde_json::to_string(&body).unwrap();
        println!("{}", body_);
        let resp = self
            .client
            .patch(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth)
            .body(body_)
            .send()?
            .json()?;
        Ok(resp)
    }
}
