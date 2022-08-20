mod column;
mod filter;
mod icon;
mod notion;
mod page;
mod query_database;
mod query_database_emoji;
mod sort;
mod update_page;
use crate::column::{MultiSelectColumn, Temporary, Vertical};
use crate::filter::{Filter, FilterKind, FormulaFilter, MultiSelectFilter, NumberFilter};
use crate::query_database::QueryDatabaseBody;
use crate::sort::{Sort, SortDirection};
use crate::update_page::{MultiSelectOption, Property, UpdatePageBody};

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(notion_api_token, database_id);
    let vertical = Vertical::Politics;

    let mut query_database_body = QueryDatabaseBody {
        sorts: Some(vec![Sort::Timestamp {
            timestamp: "last_edited_time".to_string(),
            direction: SortDirection::Descending,
        }]),
        filter: Some(FilterKind::And(vec![
            Filter::NumOfVertical {
                formula: FormulaFilter::NumberFilter(NumberFilter::Equals(1)),
            },
            Filter::Vertical {
                multi_select: MultiSelectFilter::Contains(MultiSelectColumn::Vertical(vertical)),
            },
            // Filter::Temporary {
            //     multi_select: MultiSelectFilter::Contains(MultiSelectColumn::Templorary(
            //         Temporary::Debug,
            //     )),
            // },
        ])),
        start_cursor: None,
    };

    // let pages = notion.query_database_all(&mut query_database_body)?;
    let query_database_cond = "🧑".to_string();
    let pages = notion.query_database_emoji(&mut query_database_body, &query_database_cond)?;
    println!("length of pages: {}", pages.len());
    let update_page_body = UpdatePageBody {
        // properties: Some(Property::Temporary {
        //     multi_select: vec![MultiSelectOption {
        //         name: Some("NoVerticalPerson".to_string()),
        //         id: None,
        //     }],
        // }),
        properties: None,
        archived: None,
        icon: Some(icon::Emoji {
            emoji: vertical.to_string(),
        }),
        // icon: None,
        cover: None,
    };
    for page in &pages {
        let resp = notion.update_page(page.id.to_string(), &update_page_body)?;
        println!("{}", &resp.url.to_string());
    }

    Ok(())
}
