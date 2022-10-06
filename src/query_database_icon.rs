use crate::icon::Icon;
use crate::notion::Notion;
use crate::page::Page;
use crate::query_database::QueryDatabaseBody;

impl Notion {
    pub fn query_database_icon(
        &self,
        body: &mut QueryDatabaseBody,
        max_num: Option<usize>,
    ) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let pages = match max_num {
            Some(v) => self.query_database_recursive(body, v)?,
            None => self.query_database_all(body)?,
        };
        let mut filtered_pages = Vec::<Page>::new();
        for page in &pages {
            if let Some(v) = &page.icon {
                match v {
                    Icon::Emoji(e) => {
                        let true_emoji = page.properties.symbol.select.name.to_string();

                        if e.emoji != true_emoji {
                            filtered_pages.push(page.clone());
                        }
                    }
                    Icon::File(_) | Icon::External(_) => {
                        filtered_pages.push(page.clone());
                    }
                }
            }
        }
        Ok(filtered_pages)
    }
}
