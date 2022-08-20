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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryDatabaseResp {
    pub results: Vec<Page>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
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

    pub fn query_database_all(
        &self,
        body: &mut QueryDatabaseBody,
    ) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let mut has_more = true;
        let mut start_cursor: Option<String> = None;
        let mut pages = Vec::<Page>::new();
        while has_more {
            body.start_cursor = start_cursor;
            let mut resp = self.query_database(&body)?;
            pages.append(&mut resp.results);
            has_more = resp.has_more;
            start_cursor = resp.next_cursor;
        }
        Ok(pages)
    }
}
