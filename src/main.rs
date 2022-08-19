mod icon;
mod notion;
mod page;
mod query_database;
mod query_database_emoji;
mod update_page;
use crate::query_database::{Filter, FormulaFilter, NumberFilter, QueryDatabaseBody};
use crate::update_page::UpdatePageBody;

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(notion_api_token, database_id);

    let query_database_body = QueryDatabaseBody {
        sort: None,
        filter: Some(Filter {
            property: "Num of Vertical".to_string(),
            formula: FormulaFilter {
                number: NumberFilter { equals: 1 },
            },
        }),
    };
    let query_database_cond = "🐯".to_string();
    let database = notion.query_database_emoji(&query_database_body, &query_database_cond)?;
    let page_id = &database.results[0].id;
    println!("{}", &database.results[0].url.to_string());
    // let update_page_body = UpdatePageBody {
    //     properties: None,
    //     archived: None,
    //     icon: Some(Emoji::new("🐶".to_string())),
    //     cover: None,
    // };
    // let resp = notion.update_page(page_id.to_string(), &update_page_body)?;
    // println!("{}", &resp.url.to_string());

    Ok(())
}
