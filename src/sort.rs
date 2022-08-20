use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    // Ascending,
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
