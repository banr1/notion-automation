use crate::icon::Icon;
use crate::notion::Notion;
use crate::page::Page;
use crate::query_database::{QueryDatabaseBody, QueryDatabaseResp};

impl Notion {
    pub fn query_database_emoji(
        &self,
        body: &QueryDatabaseBody,
        cond: &String,
    ) -> Result<QueryDatabaseResp, Box<dyn std::error::Error>> {
        let resp: QueryDatabaseResp = self.query_database(&body)?;
        let mut filtered_resp = QueryDatabaseResp {
            results: Vec::<Page>::new(),
        };
        for page in &resp.results {
            if let Icon::Emoji(_emoji) = &page.icon {
                if &_emoji.emoji == cond {
                    filtered_resp.results.push(page.clone());
                }
            }
        }
        Ok(filtered_resp)
    }
}
