use dotenv::dotenv;
use std::env;

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

    let resp = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::AUTHORIZATION, auth)
        .send()?;
    let result = resp.text()?;
    println!("{:#?}", result);
    Ok(())
}
