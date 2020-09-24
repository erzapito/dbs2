extern crate diesel;

use diesel::prelude::*;
use crate::entities::*;
use crate::schema::music::columns::disc;
use crate::schema::music::columns::artist;
use crate::schema::music::columns::id;


fn get_search_token(  src_search_token: &Option<String> ) -> String {
    let mut search_token: String = "%".to_string();

    if src_search_token.is_some() {
        search_token.push_str( src_search_token.as_ref().unwrap() );
    }

    search_token.push_str( "%" );
    search_token
}


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

  pub fn list (&self, page : usize, page_size: usize, src_search_token: &Option<String>) -> Vec<Music> {
      use crate::schema::music::dsl::*;

      let search_token = get_search_token( src_search_token );

      music
          .filter( artist.like(&search_token).or( disc.like( &search_token ) ))
          .limit(page_size as i64)
          .offset((page * page_size) as i64)
          .load::<Music>(&self.connection)
          .expect("Error loading music")
  }

  pub fn total(&self, src_search_token: &Option<String>) -> u32 {
      use crate::schema::music::dsl::*;

      let search_token = get_search_token( src_search_token );

      let r = music.count()
          .filter( artist.like( &search_token ).or( disc.like( &search_token ) ))
          .get_result(&self.connection)
          .unwrap_or(0);

      r as u32
  }
}
