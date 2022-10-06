use crate::notion::Notion;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteBlockResp {
    // id: String,
}

impl Notion {
    pub fn delete_block(
        &self,
        block_id: &String,
    ) -> Result<DeleteBlockResp, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.notion.com/v1/blocks/{block_id}",
            block_id = block_id
        );
        let resp = self
            .client
            .delete(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth)
            .send()?
            .json()?;
        Ok(resp)
    }
}
