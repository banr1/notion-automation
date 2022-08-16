use dotenv::dotenv;
use serde::Deserialize;
use std::env;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.notion.com/v1/databases/{database_id}/query",
        database_id = database_id
    );
    let auth = format!(
        "Bearer {notion_api_token}",
        notion_api_token = notion_api_token
    );

    let resp: QueryDatabaseResp = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::AUTHORIZATION, auth)
        .send()?
        .json()?;
    println!("{:#?}", resp.object);
    println!("{:#?}", resp.results[0].object);
    println!("{:#?}", resp.results[0].id);
    Ok(())
}
