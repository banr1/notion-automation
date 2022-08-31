mod column;
mod file;
mod filter;
mod icon;
mod notion;
mod page;
mod query_database;
mod query_database_icon;
mod sort;
mod symbol;
mod update_page;
use crate::filter::{Filter, FilterKind, SelectFilter};
use crate::icon::{Emoji, Icon};
use crate::query_database::QueryDatabaseBody;
use crate::sort::{Sort, SortDirection};
use crate::symbol::Symbol;
use crate::update_page::{Property, SelectOption, UpdatePageBody};

use dotenv::dotenv;
use std::env;
use strum::IntoEnumIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(api_token, database_id);
    // let all_symbols = Symbol::iter().collect::<Vec<_>>();
    let all_symbols = vec![Symbol::MUTB];

    for symbol in all_symbols {
        let mut query_database_body = QueryDatabaseBody {
            sorts: Some(vec![Sort::Timestamp {
                timestamp: "last_edited_time".to_string(),
                direction: SortDirection::Descending,
            }]),
            filter: Some(FilterKind::And(vec![Filter::Symbol {
                select: SelectFilter::<Symbol>::Equals(symbol),
            }])),
            start_cursor: None,
        };

        let pages = notion.query_database_all(&mut query_database_body)?;
        println!("length of pages: {}", pages.len());
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
        for page in pages {
            let resp = notion.update_page(&page.id.to_string(), &update_page_body)?;
            println!("{}", &resp.url.to_string());
        }
    }
    Ok(())
}
