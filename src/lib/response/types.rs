use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedditResponse<T> {
    kind: Option<String>,
    data: T,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Listing<List> {
    after: Option<String>,
    before: Option<String>,
    dist: i32,
    modhash: Option<String>,
    geo_filter: String,
    children: Vec<RedditResponse<List>>,
}
