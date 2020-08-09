extern crate diesel;

use crate::entities::*;
use diesel::prelude::*;

pub fn list (page : usize, page_size: usize) -> Vec<Music> {
    use crate::schema::music::dsl::*;

    let connection = crate::db::establish_connection();
    
    music.limit(page_size as i64)
        .offset((page * page_size) as i64)
        .load::<Music>(&connection)
        .expect("Error loading music")
}

pub fn total() -> u32 {
    use crate::schema::music::dsl::*;

    let connection = crate::db::establish_connection();

    let r = music.count()
        .get_result(&connection)
        .unwrap_or(0);

    r as u32
}
