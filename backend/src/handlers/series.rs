use crate::errors::ApiError;
use crate::database::PoolType;
use crate::helpers::{respond_json, respond_ok};
use crate::models::series::{create, delete, find, get_all, count_all, update, NewSeries, UpdateSeries, Series};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path, Query};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SeriesResponse {
    pub id: i32,
    pub capitulos: String,
    pub categoria: String,
    pub fansub: String,
    pub idioma: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SeriesListingResponse {
    pub items: Vec<Series>,
    pub total: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, Default)]
pub struct SeriesListRequest {
    pub page: i64,
    pub search: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateSeriesRequest {
    pub capitulos: String,
    pub categoria: String,
    pub fansub: String,
    pub idioma: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateSeriesRequest {
    pub id: i32,
    pub capitulos: String,
    pub categoria: String,
    pub fansub: String,
    pub idioma: String,
    pub name: String,
}

pub async fn create_series(
    pool: Data<PoolType>,
    params: Json<CreateSeriesRequest>,
) -> Result<HttpResponse, ApiError> {
    validate(&params)?;

    // temporarily use the new user's id for created_at/updated_at
    // update when auth is added
    let new_item = NewSeries {
        capitulos: params.capitulos.to_string(),
        categoria: params.categoria.to_string(),
        fansub: params.fansub.to_string(),
        idioma: params.idioma.to_string(),
        name: params.name.to_string(),
    }
    .into();
    block(move || create(&pool, &new_item)).await?;
    respond_ok()
}

pub async fn delete_series(
    item_id: Path<i32>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *item_id)).await?;
    respond_ok()
}

pub async fn get_series(
    item_id: Path<i32>,
    pool: Data<PoolType>,
) -> Result<Json<SeriesResponse>, ApiError> {
    let item = block(move || find(&pool, *item_id)).await?;
    respond_json(item)
}

pub async fn series_list(pool: Data<PoolType>, params: Query<SeriesListRequest>) -> Result<Json<SeriesListingResponse>, ApiError> {

    let page_size: i64 = 10;
    let page = params.page;
    let search_token = &params.search;

    let users = get_all(&pool, search_token, page, page_size);
    let total = count_all(&pool, search_token);

    respond_json( SeriesListingResponse {
      items: users?,
      total: total?,
    } )
}

pub async fn update_series(
    item_id: Path<i32>,
    pool: Data<PoolType>,
    params: Json<UpdateSeriesRequest>,
) -> Result<HttpResponse, ApiError> {
    validate(&params)?;

    let update_item = UpdateSeries {
        id: *item_id,
        capitulos: params.capitulos.to_string(),
        categoria: params.categoria.to_string(),
        fansub: params.fansub.to_string(),
        idioma: params.idioma.to_string(),
        name: params.name.to_string(),
    };
    block(move || update(&pool, &update_item)).await?;
    respond_ok()
}

impl From<Series> for SeriesResponse {
    fn from(item: Series) -> Self {
        SeriesResponse {
            id: item.id,
            capitulos: item.capitulos.to_string(),
            categoria: item.categoria.to_string(),
            fansub: item.fansub.to_string(),
            idioma: item.idioma.to_string(),
            name: item.name.to_string(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::series::tests::create_serie as model_create_serie;
    use crate::tests::helpers::tests::get_data_pool;

    pub fn get_all_series(pool: &PoolType) -> Vec<Series> {
        get_all(&pool, &"".to_string(), 0, 10).unwrap()
    }

    pub fn get_first_series_id(pool: &PoolType) -> i32 {
        get_all_series(&pool)[0].id
    }

    #[actix_rt::test]
    async fn it_creates_a_series() {
        let data_pool = get_data_pool();
        let pool = &data_pool;

        let params = Json(CreateSeriesRequest {
            capitulos: "Capitulos".to_string(),
            categoria: "Categoria".to_string(),
            fansub: "Fansub".to_string(),
            idioma: "Idioma".to_string(),
            name: "Name".to_string(),
        });
        let response = create_series(data_pool.clone(), Json(params.clone()))
            .await;
        assert!(response.is_ok());
        let first_item = &get_all_series(&pool)[0];
        assert_eq!(first_item.name, params.name);
    }

    #[actix_rt::test]
    async fn it_deletes_a_serie() {
        let data_pool = get_data_pool();
        model_create_serie(&data_pool).unwrap();
        let first_item = &get_all_series(&data_pool)[0];
        let item_id = first_item.id;
        let item_id_path: Path<i32> = item_id.into();
        let item = find(&data_pool, item_id);
        assert!(item.is_ok());
        delete_series(item_id_path, data_pool.clone()).await.unwrap();
        let item = find(&data_pool, item_id);
        assert!(item.is_err());
    }

    #[actix_rt::test]
    async fn it_gets_a_series() {
        let data_pool = get_data_pool();
        let pool = &data_pool;

        model_create_serie(&pool).unwrap();
        let first_item = &get_all_series(&pool)[0];
        let item_id: Path<i32> = get_first_series_id(&pool).into();
        let response = get_series(item_id, data_pool.clone()).await.unwrap();
        assert_eq!(response.id, first_item.id);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_series() {
        let data_pool = get_data_pool();
        let id = -1;
        let item_id: Path<i32> = id.into();
        let response = get_series(item_id, data_pool).await;
        let expected_error = ApiError::NotFound(format!("Serie {} not found", id.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_series_a_music() {
        let data_pool = get_data_pool();
        let pool = &data_pool;
        model_create_serie(&pool).unwrap();
        let first_item = &get_all_series(&pool)[0];
        let item_id: i32 = first_item.id;
        let item_id_path: Path<i32> = item_id.into();
        let params = Json(UpdateSeriesRequest {
            id: item_id,
            capitulos: "CapitulosUpdate".to_string(),
            categoria: "CategoriaUpdate".to_string(),
            fansub: "FansubUpdate".to_string(),
            idioma: "IdiomaUpdate".to_string(),
            name: "NameUpdate".to_string(),
        });
        let response = update_series(item_id_path, data_pool.clone(), Json(params.clone()))
            .await;
        assert!(response.is_ok());
        let first_item = &get_all_series(&pool)[0];
        assert_eq!("CapitulosUpdate", first_item.capitulos);
        assert_eq!("NameUpdate", first_item.name);
    }

}
