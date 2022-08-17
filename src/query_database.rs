use crate::notion::Notion;
use serde::Deserialize;

#[derive(Deserialize)]
struct Page {
    object: String,
    id: String,
}

#[derive(Deserialize)]
struct QueryDatabaseResp {
    object: String,
    results: Vec<Page>,
}

impl Notion {
    pub fn query_database(&self) -> Result<(), Box<dyn std::error::Error>> {
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
        println!("{:#?}", resp.results[0].id);
        Ok(())
    }
}
