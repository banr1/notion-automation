use crate::icon::Icon;
use crate::notion::Notion;
use crate::query_database::{QueryDatabaseBody, QueryDatabaseResp};

impl Notion {
    pub fn query_database_emoji(
        &self,
        body: &QueryDatabaseBody,
        cond: &String,
    ) -> Result<QueryDatabaseResp, Box<dyn std::error::Error>> {
        let resp: QueryDatabaseResp = self.query_database(&body)?;
        match resp.results[0].icon {
            Icon::File => {
                println!("file");
            }
            Icon::Emoji => {
                println!("emoji");
            }
        }
        Ok(resp)
    }
}
