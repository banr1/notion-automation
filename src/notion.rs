pub struct Notion {
    pub client: reqwest::blocking::Client,
    pub auth: String,
    pub database_id: String,
}

impl Notion {
    pub fn new(api_token: String, database_id: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            auth: format!("Bearer {notion_api_token}", notion_api_token = api_token),
            database_id: database_id,
        }
    }
}
