use crate::notion::Notion;
use crate::page::Page;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryDatabaseResp {
    object: String,
    pub results: Vec<Page>,
}

impl Notion {
    pub fn query_database(&self) -> Result<QueryDatabaseResp, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.notion.com/v1/databases/{database_id}/query",
            database_id = &self.database_id
        );
        let resp: QueryDatabaseResp = self
            .client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth)
            .send()?
            .json()?;
        Ok(resp)
    }
}
