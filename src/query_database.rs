use crate::notion::Notion;
use crate::page::Page;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NumberFilter {
    pub equals: i32,
}

#[derive(Serialize)]
pub struct FormulaFilter {
    pub number: NumberFilter,
}

#[derive(Serialize)]
pub struct PropertyFilter {
    pub property: String,
    pub formula: FormulaFilter,
}

#[derive(Serialize)]
pub struct QueryDatabaseBody {
    pub filter: PropertyFilter,
}

#[derive(Deserialize)]
pub struct QueryDatabaseResp {
    object: String,
    pub results: Vec<Page>,
}

impl Notion {
    pub fn query_database(
        &self,
        body: QueryDatabaseBody,
    ) -> Result<QueryDatabaseResp, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.notion.com/v1/databases/{database_id}/query",
            database_id = &self.database_id
        );
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
