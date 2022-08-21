mod block;
mod color;
mod column;
mod filter;
mod icon;
mod notion;
mod page;
mod query_database;
mod retrieve_blocks;
mod rich_text;
mod sort;
mod update_page;
use crate::column::{Horizontal, Temporary, Version, Vertical};
use crate::filter::{
    Filter, FilterKind, FormulaFilter, MultiSelectFilter, NumberFilter, SelectFilter,
};
use crate::query_database::QueryDatabaseBody;
use crate::sort::{Sort, SortDirection};

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(notion_api_token, database_id);

    let mut query_database_body = QueryDatabaseBody {
        sorts: Some(vec![Sort::Timestamp {
            timestamp: "last_edited_time".to_string(),
            direction: SortDirection::Descending,
        }]),
        filter: Some(FilterKind::And(vec![Filter::Temporary {
            multi_select: MultiSelectFilter::<Temporary>::Contains(Temporary::Debug),
        }])),
        start_cursor: None,
    };
    let pages = notion.query_database_all(&mut query_database_body)?;
    println!("length of pages: {}", pages.len());

    for page in pages {
        let blocks = notion.retrieve_blocks_all(&page.id.to_string())?;
        for block in blocks {
            println!("block: {:?}", &block);
        }
    }

    Ok(())
}
