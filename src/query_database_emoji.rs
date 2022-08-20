use crate::icon::Icon;
use crate::notion::Notion;
use crate::page::Page;
use crate::query_database::QueryDatabaseBody;

impl Notion {
    pub fn query_database_emoji(
        &self,
        body: &mut QueryDatabaseBody,
        cond: &String,
    ) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let pages: Vec<Page> = self.query_database_all(body)?;
        let mut filtered_pages = Vec::<Page>::new();
        for page in &pages {
            if let Some(Icon::Emoji(_emoji)) = &page.icon {
                if &_emoji.emoji == cond {
                    filtered_pages.push(page.clone());
                }
            }
        }
        Ok(filtered_pages)
    }
}
