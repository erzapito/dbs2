use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Music {
    pub id: i32,
    pub artist: Option<String>,
    pub disc: Option<String>,
    pub version: Option<i32>,
}
