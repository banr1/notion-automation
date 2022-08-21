mod column;
mod filter;
mod icon;
mod notion;
mod page;
mod query_database;
mod query_database_emoji;
mod sort;
mod update_page;
use crate::column::{Horizontal, Temporary, Version, Vertical};
use crate::filter::{
    Filter, FilterKind, FormulaFilter, MultiSelectFilter, NumberFilter, SelectFilter,
};
use crate::query_database::QueryDatabaseBody;
use crate::sort::{Sort, SortDirection};
use crate::update_page::{MultiSelectOption, Property, SelectOption, UpdatePageBody};

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_api_token = env::var("NOTION_API_TOKEN")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let notion = notion::Notion::new(notion_api_token, database_id);
    // let verticals = vec![
    //     Vertical::Finance,
    //     Vertical::CS,
    //     Vertical::Crypto,
    //     Vertical::Philosophy,
    //     Vertical::Business,
    //     Vertical::ML,
    //     Vertical::Enterme,
    //     Vertical::Politics,
    //     Vertical::Geography,
    //     Vertical::Food,
    //     Vertical::Math,
    //     Vertical::Music,
    //     Vertical::Economy,
    //     Vertical::Activity,
    //     Vertical::Society,
    //     Vertical::Biology,
    //     Vertical::Physics,
    //     Vertical::Game,
    //     Vertical::Medical,
    //     Vertical::Transport,
    //     Vertical::Law,
    //     Vertical::Energy,
    //     Vertical::Design,
    //     Vertical::Language,
    // ];

    // for vertical in verticals {
    let mut query_database_body = QueryDatabaseBody {
        sorts: Some(vec![Sort::Timestamp {
            timestamp: "last_edited_time".to_string(),
            direction: SortDirection::Descending,
        }]),
        filter: Some(FilterKind::And(vec![
            Filter::Horizontal {
                multi_select: MultiSelectFilter::<Horizontal>::DoesNotContain(Horizontal::Content),
            },
            Filter::Horizontal {
                multi_select: MultiSelectFilter::<Horizontal>::DoesNotContain(Horizontal::Project),
            },
            Filter::Version {
                select: SelectFilter::IsEmpty(true),
            },
        ])),
        start_cursor: None,
    };

    let pages = notion.query_database_all(&mut query_database_body)?;
    println!("length of pages: {}", pages.len());
    let update_page_body = UpdatePageBody {
        properties: Some(Property::Version {
            select: SelectOption {
                name: Some("Aug2022".to_string()),
            },
        }),
        // properties: None,
        archived: None,
        icon: None,
        cover: None,
    };
    for page in pages {
        let resp = notion.update_page(&page.id.to_string(), &update_page_body)?;
        println!("{}", &resp.url.to_string());
    }
    // }
    Ok(())
}
