use serde::Deserialize;

#[derive(Deserialize)]
struct PropertyItem {
    object: String,
    id: String,
    type_: String,
}

#[derive(Deserialize)]
struct RetrievePagePropertyItemResp {
    object: String,
    results: Vec<PropertyItem>,
}
