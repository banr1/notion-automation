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
use crate::column::Vertical;
use crate::filter::{
    Filter, FilterKind, FormulaFilter, MultiSelectFilter, NumberFilter, SelectFilter,
};
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
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(notion_api_token, database_id);
    let symbol = Symbol::Python;

    let mut query_database_body = QueryDatabaseBody {
        sorts: Some(vec![Sort::Timestamp {
            timestamp: "last_edited_time".to_string(),
            direction: SortDirection::Descending,
        }]),
        filter: Some(FilterKind::And(vec![
            Filter::Vertical {
                multi_select: MultiSelectFilter::<Vertical>::Contains(Vertical::Computer),
            },
            // Filter::Vertical {
            //     multi_select: MultiSelectFilter::<Vertical>::Contains(Vertical::ML),
            // },
            Filter::Symbol {
                select: SelectFilter::IsEmpty(true),
            },
            Filter::NumOfVertical {
                formula: FormulaFilter::NumberFilter(NumberFilter::Equals(2)),
            },
            // Filter::Temporary {
            //     multi_select: MultiSelectFilter::<Temporary>::Contains(Temporary::Debug),
            // },
        ])),
        start_cursor: None,
    };

    let pages = notion.query_database_icon(&mut query_database_body, None)?;
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
    Ok(())
}
