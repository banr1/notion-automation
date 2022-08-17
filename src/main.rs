mod emoji;
mod notion;
mod page;
mod query_database;
mod update_page;

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;

    let notion = notion::Notion::new(notion_api_token, database_id);
    let database = notion.query_database()?;
    let page_id = &database.results[0].id;
    println!("{}", &database.results[0].url.to_string());
    let resp = notion.update_page(page_id.to_string())?;
    println!("{}", &resp.url.to_string());

    Ok(())
}
