use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::music::{create, delete, find, get_all, count_all, update, NewMusic, UpdateMusic, Music};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path, Query};
use serde::Serialize;
use validator::Validate;

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
    pub id: i32,
    pub artist: String,
    pub disc: String,
}

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
) -> Result<HttpResponse, ApiError> {
    validate(&params)?;

    // temporarily use the new user's id for created_at/updated_at
    // update when auth is added
    let new_user = NewMusic {
        artist: params.artist.to_string(),
        disc: params.disc.to_string(),
    }
    .into();
    block(move || create(&pool, &new_user)).await?;
    respond_ok()
}

pub async fn update_music(
    item_id: Path<i32>,
    pool: Data<PoolType>,
    params: Json<UpdateMusicRequest>,
) -> Result<HttpResponse, ApiError> {
    validate(&params)?;

    let update_item = UpdateMusic {
        id: *item_id,
        artist: params.artist.to_string(),
        disc: params.disc.to_string(),
    };
    block(move || update(&pool, &update_item)).await?;
    respond_ok()
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::music::tests::create_music as model_create_music;
    use crate::tests::helpers::tests::get_data_pool;

    pub fn get_all_music(pool: &PoolType) -> Vec<Music> {
        get_all(&pool, &"".to_string(), 0, 10).unwrap()
    }

    pub fn get_first_music_id(pool: &PoolType) -> i32 {
        get_all_music(&pool)[0].id
    }

    #[actix_rt::test]
    async fn it_gets_a_music() {
        let data_pool = get_data_pool();
        let pool = &data_pool;

        model_create_music(&pool).unwrap();
        let first_item = &get_all_music(&pool)[0];
        let item_id: Path<i32> = get_first_music_id(&pool).into();
        let response = get_music(item_id, data_pool.clone()).await.unwrap();
        assert_eq!(response.id, first_item.id);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_music() {
        let data_pool = get_data_pool();
        let id = -1;
        let item_id: Path<i32> = id.into();
        let response = get_music(item_id, data_pool).await;
        let expected_error = ApiError::NotFound(format!("Music {} not found", id.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_gets_all_music() {
        let data_pool = get_data_pool();
        let pool = &data_pool;

        model_create_music(&pool).unwrap();
        let params = Query(MusicListRequest {
            page: 0,
            search: "".to_string(),
        });
        let response = music_list(data_pool.clone(), params).await;
        assert!(response.is_ok());
        let response_u = response.unwrap().into_inner();
        assert_eq!(response_u.items[0], get_all_music(&pool)[0]);
        assert_eq!(response_u.total, 1);
    }

    #[actix_rt::test]
    async fn it_creates_a_music() {
        let data_pool = get_data_pool();
        let pool = &data_pool;

        let params = Json(CreateMusicRequest {
            artist: "ARTIST".into(),
            disc: "DISC".into(),
        });
        let response = create_music(data_pool.clone(), Json(params.clone()))
            .await;
        assert!(response.is_ok());
        let first_item = &get_all_music(&pool)[0];
        assert_eq!(first_item.artist, params.artist);
    }

    #[actix_rt::test]
    async fn it_updates_a_music() {
        let data_pool = get_data_pool();
        let pool = &data_pool;
        model_create_music(&pool).unwrap();
        let first_item = &get_all_music(&pool)[0];
        let item_id: i32 = first_item.id;
        let item_id_path: Path<i32> = item_id.into();
        let params = Json(UpdateMusicRequest {
            id: item_id,
            artist: "ArtistUpdate".into(),
            disc: "DiscUpdate".into(),
        });
        let response = update_music(item_id_path, data_pool.clone(), Json(params.clone()))
            .await;
        assert!(response.is_ok());
        let first_item = &get_all_music(&pool)[0];
        assert_eq!("ArtistUpdate", first_item.artist);
        assert_eq!("DiscUpdate", first_item.disc);
    }

    #[actix_rt::test]
    async fn it_deletes_a_music() {
        let data_pool = get_data_pool();
        model_create_music(&data_pool).unwrap();
        let first_item = &get_all_music(&data_pool)[0];
        let item_id = first_item.id;
        let item_id_path: Path<i32> = item_id.into();
        let item = find(&data_pool, item_id);
        assert!(item.is_ok());
        delete_music(item_id_path, data_pool.clone()).await.unwrap();
        let item = find(&data_pool, item_id);
        assert!(item.is_err());
    }
}
