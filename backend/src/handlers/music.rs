use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::music::{create, delete, find, get_all, count_all, update, NewMusic, UpdateMusic, Music};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path, Query};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;



#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MusicResponse {
    pub id: i32,
    pub artist: String,
    pub disc: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MusicListingResponse {
    pub items: Vec<Music>,
    pub total: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateMusicRequest {
    pub artist: String,
    pub disc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, Default)]
pub struct MusicListRequest {
    pub page: i64,
    pub search: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateMusicRequest {
    pub id: usize,
    pub artist: String,
    pub disc: String,
}

/// Get a user
pub async fn get_music(
    item_id: Path<i32>,
    pool: Data<PoolType>,
) -> Result<Json<MusicResponse>, ApiError> {
    let item = block(move || find(&pool, *item_id)).await?;
    respond_json(item)
}

pub async fn music_list(pool: Data<PoolType>, params: Query<MusicListRequest>) -> Result<Json<MusicListingResponse>, ApiError> {

    let page_size: i64 = 10;
    let page = params.page;
    let search_token = &params.search;

    let users = get_all(&pool, search_token, page, page_size);
    let total = count_all(&pool, search_token);

    respond_json( MusicListingResponse {
      items: users?,
      total: total?,
    } )
}

pub async fn create_music(
    pool: Data<PoolType>,
    params: Json<CreateMusicRequest>,
) -> Result<Json<MusicResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the new user's id for created_at/updated_at
    // update when auth is added
    let new_user: Music = NewMusic {
        artist: params.artist.to_string(),
        disc: params.disc.to_string(),
    }
    .into();
    let user = block(move || create(&pool, &new_user)).await?;
    respond_json(user.into())
}

pub async fn update_music(
    item_id: Path<i32>,
    pool: Data<PoolType>,
    params: Json<UpdateMusicRequest>,
) -> Result<Json<MusicResponse>, ApiError> {
    validate(&params)?;

    let update_item = UpdateMusic {
        id: *item_id,
        artist: params.artist.to_string(),
        disc: params.disc.to_string(),
    };
    let item = block(move || update(&pool, &update_item)).await?;
    respond_json(item.into())
}

pub async fn delete_music(
    item_id: Path<i32>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *item_id)).await?;
    respond_ok()
}

impl From<Music> for MusicResponse {
    fn from(item: Music) -> Self {
        MusicResponse {
            id: item.id,
            artist: item.artist.to_string(),
            disc: item.disc.to_string(),
        }
    }
}

impl From<Music> for UserResponse {
    fn from(item: Music) -> Self {
        UserResponse {
            id: Uuid::new_v4(),
            first_name: item.artist.to_string(),
            last_name: item.disc.to_string(),
            email: item.artist.to_string(),
        }
    }
}

/*impl From<Vec<Music>> for MusicListingResponse {
    fn from(items: Vec<Music>) -> Self {
        MusicListingResponse {
          items: items.into_par_iter().map(|item| item.into()).collect(),
          total: 10,
        }
    }
}*/

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::user::tests::create_user as model_create_user;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    pub fn get_all_users() -> UsersResponse {
        let pool = get_pool();
        get_all(&pool).unwrap()
    }

    pub fn get_first_users_id() -> Uuid {
        get_all_users().0[0].id
    }

    #[actix_rt::test]
    async fn it_gets_a_user() {
        let first_user = &get_all_users().0[0];
        let user_id: Path<Uuid> = get_first_users_id().into();
        let response = get_user(user_id, get_data_pool()).await.unwrap();
        assert_eq!(response.into_inner(), *first_user);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_user() {
        let uuid = Uuid::new_v4();
        let user_id: Path<Uuid> = uuid.into();
        let response = get_user(user_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("User {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_gets_all_users() {
        let response = get_users(get_data_pool()).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().0[0], get_all_users().0[0]);
    }

    #[actix_rt::test]
    async fn it_creates_a_user() {
        let params = Json(CreateUserRequest {
            first_name: "Satoshi".into(),
            last_name: "Nakamoto".into(),
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        });
        let response = create_user(get_data_pool(), Json(params.clone()))
            .await
            .unwrap();
        assert_eq!(response.into_inner().first_name, params.first_name);
    }

    #[actix_rt::test]
    async fn it_updates_a_user() {
        let first_user = &get_all_users().0[0];
        let user_id: Path<Uuid> = get_first_users_id().into();
        let params = Json(UpdateUserRequest {
            first_name: first_user.first_name.clone(),
            last_name: first_user.last_name.clone(),
            email: first_user.email.clone(),
        });
        let response = update_user(user_id, get_data_pool(), Json(params.clone()))
            .await
            .unwrap();
        assert_eq!(response.into_inner().first_name, params.first_name);
    }

    #[actix_rt::test]
    async fn it_deletes_a_user() {
        let created = model_create_user();
        let user_id = created.unwrap().id;
        let user_id_path: Path<Uuid> = user_id.into();
        let user = find(&get_pool(), user_id);
        assert!(user.is_ok());
        delete_user(user_id_path, get_data_pool()).await.unwrap();
        let user = find(&get_pool(), user_id);
        assert!(user.is_err());
    }
}
