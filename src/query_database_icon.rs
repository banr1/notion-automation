use crate::icon::Icon;
use crate::notion::Notion;
use crate::page::Page;
use crate::query_database::QueryDatabaseBody;

impl Notion {
    pub fn query_database_icon(
        &self,
        body: &mut QueryDatabaseBody,
    ) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let pages: Vec<Page> = self.query_database_all(body)?;
        let mut filtered_pages = Vec::<Page>::new();
        for page in &pages {
            if let Some(v) = &page.icon {
                match v {
                    Icon::Emoji(e) => {
                        // ()
                        filtered_pages.push(page.clone());
                        // if e.emoji == "⚙️".to_string() {
                        //     filtered_pages.push(page.clone());
                        // }
                    }
                    Icon::File(f) => {
                        ()
                        // if f.file.url.contains(&"notion.png".to_string()) {
                        //     filtered_pages.push(page.clone());
                        // }
                    }
                    Icon::External(_) => (),
                }
            }
        }
        Ok(filtered_pages)
    }
}
