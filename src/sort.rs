use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Sort {
    // Property {
    //     property: String,
    //     direction: SortDirection,
    // },
    Timestamp {
        timestamp: String,
        direction: SortDirection,
    },
}

pub type Sorts = Vec<Sort>;
