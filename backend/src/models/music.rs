use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::music::MusicResponse;
use crate::schema::music;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "music"]
pub struct Music {
    pub id: i32,
    pub artist: String,
    pub disc: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
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

/// Find a user by the user's authentication information (email + password)
/// Return an Unauthorized error if it doesn't match
pub fn find_by_auth(
    pool: &PoolType,
    user_email: &str,
    user_password: &str,
) -> Result<MusicResponse, ApiError> {
    use crate::schema::music::dsl::{artist, disc, music};

    let conn = pool.get()?;
    let result = music
        .filter(artist.eq(user_email.to_string()))
        .filter(disc.eq(user_password.to_string()))
        .first::<Music>(&conn)
        .map_err(|_| ApiError::Unauthorized("Invalid login".into()))?;
    Ok(result.into())
}

/// Create a new user
pub fn create(pool: &PoolType, new_item: &Music) -> Result<MusicResponse, ApiError> {
    use crate::schema::music::dsl::music;

    let conn = pool.get()?;
    diesel::insert_into(music).values(new_item).execute(&conn)?;
    Ok(new_item.clone().into())
}

pub fn update(pool: &PoolType, update_item: &UpdateMusic) -> Result<MusicResponse, ApiError> {
    use crate::schema::music::dsl::{id, music};

    let conn = pool.get()?;
    diesel::update(music)
        .filter(id.eq(update_item.id))
        .set(update_item)
        .execute(&conn)?;
    find(&pool, update_item.id)
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
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_users() -> Result<UsersResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    pub fn create_user() -> Result<UserResponse, ApiError> {
        let user_id = Uuid::new_v4();
        let new_user = NewUser {
            id: user_id.to_string(),
            first_name: "Model".to_string(),
            last_name: "Test".to_string(),
            email: "model-test@nothing.org".to_string(),
            password: "123456".to_string(),
            created_by: user_id.to_string(),
            updated_by: user_id.to_string(),
        };
        let user: User = new_user.into();
        create(&get_pool(), &user)
    }

    #[test]
    fn it_gets_a_user() {
        let users = get_all_users();
        assert!(users.is_ok());
    }

    #[test]
    fn test_find() {
        let users = get_all_users().unwrap();
        let user = &users.0[0];
        let found_user = find(&get_pool(), user.id).unwrap();
        assert_eq!(user, &found_user);
    }

    #[test]
    fn it_doesnt_find_a_user() {
        let user_id = Uuid::new_v4();
        let not_found_user = find(&get_pool(), user_id);
        assert!(not_found_user.is_err());
    }

    #[test]
    fn it_creates_a_user() {
        let created = create_user();
        assert!(created.is_ok());
        let unwrapped = created.unwrap();
        let found_user = find(&get_pool(), unwrapped.id.clone()).unwrap();
        assert_eq!(unwrapped, found_user);
    }

    #[test]
    fn it_updates_a_user() {
        let users = get_all_users().unwrap();
        let user = &users.0[1];
        let update_user = UpdateUser {
            id: user.id.to_string(),
            first_name: "ModelUpdate".to_string(),
            last_name: "TestUpdate".to_string(),
            email: "model-update-test@nothing.org".to_string(),
            updated_by: user.id.to_string(),
        };
        let updated = update(&get_pool(), &update_user);
        assert!(updated.is_ok());
        let found_user = find(&get_pool(), user.id).unwrap();
        assert_eq!(updated.unwrap(), found_user);
    }

    #[test]
    fn it_fails_to_update_a_nonexistent_user() {
        let user_id = Uuid::new_v4();
        let update_user = UpdateUser {
            id: user_id.to_string(),
            first_name: "ModelUpdateFailure".to_string(),
            last_name: "TestUpdateFailure".to_string(),
            email: "model-update-failure-test@nothing.org".to_string(),
            updated_by: user_id.to_string(),
        };
        let updated = update(&get_pool(), &update_user);
        assert!(updated.is_err());
    }

    #[test]
    fn it_deletes_a_user() {
        let created = create_user();
        let user_id = created.unwrap().id;
        let user = find(&get_pool(), user_id);
        assert!(user.is_ok());
        delete(&get_pool(), user_id).unwrap();
        let user = find(&get_pool(), user_id);
        assert!(user.is_err());
    }
}
