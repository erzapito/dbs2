pub mod music;

use crate::db::DbConnection;

pub type MusicDao = music::MusicDao;

pub struct Dao {
}

impl Dao {

  fn new_connection() -> DbConnection {
    crate::db::establish_connection()
  }

  pub fn music_dao( &self ) -> MusicDao {
    MusicDao{
      connection: Self::new_connection()
    }
  }

}
