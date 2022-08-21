use crate::block::Block;
use crate::notion::Notion;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RetrieveBlockBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct RetrieveBlockResp {
    pub results: Vec<Block>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl Notion {
    pub fn retrieve_blocks(
        &self,
        page_id: &String,
        body: &RetrieveBlockBody,
    ) -> Result<RetrieveBlockResp, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.notion.com/v1/blocks/{page_id}/children",
            page_id = page_id,
        );
        let body_ = serde_json::to_string(body).unwrap();
        let resp = self
            .client
            .get(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth)
            .body(body_)
            .send()?
            .json()?;

        Ok(resp)
    }

    pub fn retrieve_blocks_all(
        &self,
        page_id: &String,
    ) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
        let mut has_more = true;
        let mut start_cursor: Option<String> = None;
        let mut blocks = Vec::<Block>::new();
        while has_more {
            let body = RetrieveBlockBody {
                start_cursor: start_cursor,
            };
            let mut resp = self.retrieve_blocks(page_id, &body)?;
            blocks.append(&mut resp.results);
            has_more = resp.has_more;
            start_cursor = resp.next_cursor;
        }
        Ok(blocks)
    }
}
