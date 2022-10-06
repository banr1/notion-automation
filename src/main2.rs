mod block;
mod block_basic;
mod bookmark;
mod bulleted_list_item;
mod child_database;
mod child_page;
mod code;
mod color;
mod column;
mod delete_block;
mod divider;
mod embed;
mod equation;
mod file;
mod filter;
mod heading;
mod icon;
mod image;
mod link_preview;
mod notion;
mod numbered_list_item;
mod object;
mod page;
mod paragraph;
mod property;
mod query_database;
mod retrieve_blocks;
mod rich_text;
mod sort;
mod symbol;
mod synced_block;
mod table_of_contents;
mod todo;
mod toggle;
// mod update_page;
use crate::block::Block;
use crate::column::{External, Horizontal, Temporary, Vertical};
use crate::filter::{
    Filter, FilterKind, MultiSelectFilter,
};
use crate::query_database::QueryDatabaseBody;
use crate::sort::{Sort, SortDirection};

use dotenv::dotenv;
use std::env;
use strum::IntoEnumIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(notion_api_token, database_id);
    let verticals = Vertical::iter().collect::<Vec<_>>();

    for vertical in verticals {
        println!("{:?}", vertical);
        let mut query_database_body = QueryDatabaseBody {
            sorts: Some(vec![Sort::Timestamp {
                timestamp: "last_edited_time".to_string(),
                direction: SortDirection::Ascending,
            }]),
            filter: Some(FilterKind::And(vec![
                Filter::Vertical {
                    multi_select: MultiSelectFilter::<Vertical>::Contains(vertical),
                },
                Filter::Horizontal {
                    multi_select: MultiSelectFilter::<Horizontal>::DoesNotContain(
                        Horizontal::Project,
                    ),
                },
                Filter::Horizontal {
                    multi_select: MultiSelectFilter::<Horizontal>::DoesNotContain(
                        Horizontal::Archive,
                    ),
                },
                Filter::Horizontal {
                    multi_select: MultiSelectFilter::<Horizontal>::Contains(Horizontal::Content),
                },
                Filter::External {
                    multi_select: MultiSelectFilter::<External>::DoesNotContain(External::Amazon),
                },
                Filter::External {
                    multi_select: MultiSelectFilter::<External>::DoesNotContain(External::Kindle),
                },
                Filter::Temporary {
                    multi_select: MultiSelectFilter::<Temporary>::DoesNotContain(
                        Temporary::CannotRetrieve,
                    ),
                },
            ])),
            start_cursor: None,
        };
        let pages = notion.query_database_all(&mut query_database_body)?;
        println!("length of pages: {}", pages.len());

        for page in pages {
            println!("{:?}", page.url.to_string());
            let blocks = notion.retrieve_blocks_all(&page.id.to_string())?;
            if let Block::Image(image) = &blocks[0] {
                notion.delete_block(&image.block_basic.id.to_string())?;
            }
        }
    }

    Ok(())
}
