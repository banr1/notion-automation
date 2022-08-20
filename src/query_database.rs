use crate::filter::FilterKind;
use crate::notion::Notion;
use crate::page::Page;
use crate::sort::Sorts;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct QueryDatabaseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Sorts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterKind>,
}

#[derive(Deserialize)]
pub struct QueryDatabaseResp {
    pub results: Vec<Page>,
}

impl Notion {
    pub fn query_database(
        &self,
        body: &QueryDatabaseBody,
    ) -> Result<QueryDatabaseResp, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.notion.com/v1/databases/{database_id}/query",
            database_id = &self.database_id
        );
        println!("{}", serde_json::to_string(&body).unwrap());
        let resp: QueryDatabaseResp = self
            .client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth)
            .body(serde_json::to_string(&body).unwrap())
            .send()?
            .json()?;
        Ok(resp)
    }
}
