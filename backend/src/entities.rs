use serde::{Deserialize, Serialize};

use crate::schema::music;

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Music {
    pub id: i32,
    pub artist: Option<String>,
    pub disc: Option<String>,
    pub version: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "music"]
pub struct MusicNew {
    pub artist: Option<String>,
    pub disc: Option<String>,
}
