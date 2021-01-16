use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::music::MusicResponse;
use crate::schema::music;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable)]
#[table_name = "music"]
pub struct Music {
    pub id: i32,
    pub artist: String,
    pub disc: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "music"]
pub struct NewMusic {
    pub artist: String,
    pub disc: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "music"]
pub struct UpdateMusic {
    pub id: i32,
    pub artist: String,
    pub disc: String,
}

fn get_search_token(  src_search_token: &String ) -> String {
    let mut search_token: String = "%".to_string();
    search_token.push_str( src_search_token );
    search_token.push_str( "%" );
    search_token
}

/// Get all users
pub fn get_all(pool: &PoolType, src_search_token: &String, page: i64, page_size: i64) -> Result<Vec<Music>, ApiError> {
    use crate::schema::music::dsl::{music,artist,disc};

    let search_token = get_search_token( src_search_token );

    let conn = pool.get()?;
    let result = music
      .filter( artist.like(&search_token).or( disc.like( &search_token ) ))
      .order(( artist.asc(), disc.asc() ))
      .limit(page_size)
      .offset(page * page_size)
      .load(&conn)?;

    Ok(result.into())
}

/// count all users
pub fn count_all(pool: &PoolType, src_search_token: &String) -> Result<i64, ApiError> {
    use crate::schema::music::dsl::{music,artist,disc};

    let search_token = get_search_token( src_search_token );

    let conn = pool.get()?;
    let result = music.count()
      .filter( artist.like(&search_token).or( disc.like( &search_token ) ))
      .get_result(&conn)?;

    Ok(result)
}

/// Find a user by the user's id or error out
pub fn find(pool: &PoolType, item_id: i32) -> Result<MusicResponse, ApiError> {
    use crate::schema::music::dsl::{id, music};

    let not_found = format!("Music {} not found", item_id);
    let conn = pool.get()?;
    let result = music
        .filter(id.eq(item_id))
        .first::<Music>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(result.into())
}

/// Create a new user
pub fn create(pool: &PoolType, new_item: &NewMusic) -> Result<(), ApiError> {
    use crate::schema::music::dsl::music;

    let conn = pool.get()?;
    diesel::insert_into(music).values(new_item).execute(&conn)?;
    Ok(())
}

pub fn update(pool: &PoolType, update_item: &UpdateMusic) -> Result<(), ApiError> {
    use crate::schema::music::dsl::{id, music};

    let conn = pool.get()?;
    diesel::update(music)
        .filter(id.eq(update_item.id))
        .set(update_item)
        .execute(&conn)?;
    Ok(())
}

pub fn delete(pool: &PoolType, item_id: i32) -> Result<(), ApiError> {
    use crate::schema::music::dsl::{id, music};

    let conn = pool.get()?;
    diesel::delete(music)
        .filter(id.eq(item_id))
        .execute(&conn)?;
    Ok(())
}

impl From<NewMusic> for Music {
    fn from(item: NewMusic) -> Self {
        Music {
            id: 0,
            artist: item.artist,
            disc: item.disc,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::init_test_pool;

    pub fn get_all_music(pool: &PoolType) -> Result<Vec<Music>, ApiError> {
        get_all(&pool,&"".to_string(),0,10)
    }

    pub fn create_music(pool: &PoolType) -> Result<(), ApiError> {
        let new_item = NewMusic {
            artist: "Artist".to_string(),
            disc: "Disc".to_string(),
        };
        create(pool, &new_item)
    }

    #[test]
    fn it_gets_a_music() {
        let pool = init_test_pool();
        let items = get_all_music(&pool);
        assert!(items.is_ok());
    }

    #[test]
    fn test_find() {
        let pool = init_test_pool();
        let created = create_music(&pool);
        assert!(created.is_ok());
        let items = get_all_music(&pool).unwrap();
        let item = &items[0];
        let found_item = find(&pool, items[0].id).unwrap();
        assert_eq!(item.id, found_item.id);
    }

    #[test]
    fn it_doesnt_find_a_music() {
        let pool = init_test_pool();
        let item_id = -1;
        let not_found_item = find(&pool, item_id);
        assert!(not_found_item.is_err());
    }

    #[test]
    fn it_creates_a_music() {
        let pool = init_test_pool();

        let created = create_music(&pool);
        assert!(created.is_ok());
        let items = get_all_music(&pool).unwrap();
        assert_eq!(1, items.len());
        let item = &items[0];
        let found_item = find(&pool, item.id).unwrap();
        assert_eq!("Artist", found_item.artist);
        assert_eq!("Disc", found_item.disc);
    }

    #[test]
    fn it_updates_a_music() {
        let pool = init_test_pool();
        let created = create_music(&pool);
        assert!(created.is_ok());
        let items = get_all_music(&pool).unwrap();
        let item = &items[0];
        let update_item = UpdateMusic {
            id: item.id,
            artist: "ArtistUpdate".to_string(),
            disc: "DiscUpdate".to_string(),
        };
        let updated = update(&pool, &update_item);
        assert!(updated.is_ok());
        let found_item = find(&pool, item.id).unwrap();
        assert_eq!("ArtistUpdate", found_item.artist);
        assert_eq!("DiscUpdate", found_item.disc);
    }

    #[test]
    fn it_deletes_a_music() {
        let pool = init_test_pool();
        let created = create_music(&pool);
        assert!(created.is_ok());
        let items = get_all_music(&pool).unwrap();
        assert_eq!(1, items.len());
        let item = &items[0];
        let item_id = item.id;
        assert_eq!("Artist", item.artist);
        let item = find(&pool, item_id);
        assert!(item.is_ok());
        delete(&pool, item_id).unwrap();
        let item = find(&pool, item_id);
        assert!(item.is_err());
    }
}
