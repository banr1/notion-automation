mod column;
mod file;
mod filter;
mod icon;
mod notion;
mod object;
mod page;
mod property;
mod query_database;
mod query_database_icon;
mod sort;
mod symbol;
mod update_page;
use crate::filter::{Filter, FilterKind, SelectFilter};
use crate::icon::{Emoji, Icon};
use crate::property::{Property, SelectOption};
use crate::query_database::QueryDatabaseBody;
use crate::sort::{Sort, SortDirection};
use crate::symbol::Symbol;
use crate::update_page::UpdatePageBody;

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(api_token, database_id);
    let num_pages = 200;

    let mut query_database_body = QueryDatabaseBody {
        sorts: Some(vec![Sort::Timestamp {
            timestamp: "last_edited_time".to_string(),
            direction: SortDirection::Descending,
        }]),
        filter: Some(FilterKind::And(vec![Filter::Symbol {
            select: SelectFilter::IsNotEmpty(true),
        }])),
        start_cursor: None,
    };

    let mut pages = notion.query_database_icon(&mut query_database_body, Some(num_pages))?;
    pages.reverse();
    println!("length of pages: {}", pages.len());
    for page in pages {
        let symbol = page.properties.symbol.select.name;
        let update_page_body = UpdatePageBody {
            properties: Some(Property::Symbol {
                select: SelectOption::<Symbol> { name: Some(symbol) },
            }),
            archived: None,
            icon: Some(Icon::Emoji(Emoji {
                emoji: symbol.to_string(),
            })),
            cover: None,
        };

        let resp = notion.update_page(&page.id.to_string(), &update_page_body)?;
        println!("{}: {}", symbol, &resp.url.to_string());
    }
    Ok(())
}
