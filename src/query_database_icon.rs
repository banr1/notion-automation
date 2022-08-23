use crate::icon::Icon;
use crate::notion::Notion;
use crate::page::Page;
use crate::query_database::QueryDatabaseBody;

impl Notion {
    pub fn query_database_icon(
        &self,
        body: &mut QueryDatabaseBody,
        icon_cond: &IconCond,
    ) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let pages: Vec<Page> = self.query_database_all(body)?;
        let mut filtered_pages = Vec::<Page>::new();
        for page in &pages {
            match icon_cond {
                IconCond::ML => {
                    if let Some(v) = &page.icon {
                        match v {
                            Icon::File(f) => {
                                if f.file.url.contains(&"rust.png".to_string()) {
                                    filtered_pages.push(page.clone());
                                }
                            }
                            Icon::Emoji(_) => (),
                            Icon::External(_) => (),
                        }
                    }
                }
                IconCond::NotionAutomation => {
                    if let Some(v) = &page.icon {
                        match v {
                            Icon::Emoji(e) => {
                                println!("{:?}", e);
                                if e.emoji == "📦".to_string() {
                                    filtered_pages.push(page.clone());
                                }
                            }
                            Icon::File(_) => (),
                            Icon::External(_) => (),
                        }
                    }
                }
            }
        }
        Ok(filtered_pages)
    }
}

pub enum IconCond {
    ML,
    NotionAutomation,
}
