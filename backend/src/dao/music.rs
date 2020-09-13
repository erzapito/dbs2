extern crate diesel;

use diesel::prelude::*;
use crate::entities::*;
use crate::schema::music::columns::disc;
use crate::schema::music::columns::artist;
use crate::schema::music::columns::id;

pub struct MusicDao {
  pub connection: crate::db::DbConnection,
}

impl MusicDao {

  pub fn delete(&self, item : Music) {
    diesel::delete( crate::schema::music::table.filter(id.eq( item.id )) )
      .execute( &self.connection )
      .expect("Error deleting music");
  }

  pub fn update(&self, item : Music) {
    diesel::update( crate::schema::music::table.filter(id.eq( item.id )) )
      .set(( artist.eq( item.artist ), disc.eq( item.disc ) ))
      .execute( &self.connection )
      .expect("Error updating music");
  }

  pub fn insert(&self, item : MusicNew) {
    diesel::insert_into(crate::schema::music::table)
        .values(&item)
        .execute(&self.connection)
        .expect("Error saving new music");
  }

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
