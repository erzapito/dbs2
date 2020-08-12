extern crate diesel;

use diesel::prelude::*;
use crate::entities::*;

pub struct MusicDao {
  pub connection: crate::db::DbConnection,
}

impl MusicDao {
  pub fn list (&self, page : usize, page_size: usize) -> Vec<Music> {
      use crate::schema::music::dsl::*;

      music.limit(page_size as i64)
          .offset((page * page_size) as i64)
          .load::<Music>(&self.connection)
          .expect("Error loading music")
  }

  pub fn total(&self) -> u32 {
      use crate::schema::music::dsl::*;

      let r = music.count()
          .get_result(&self.connection)
          .unwrap_or(0);

      r as u32
  }
}
