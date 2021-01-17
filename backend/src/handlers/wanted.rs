use crate::errors::ApiError;
use crate::database::PoolType;
use crate::helpers::{respond_json, respond_ok};
use crate::models::wanted::{get_all, count_all, mark_as_downloaded, increase_wanted_week, Wanted};
use actix_web::web::{block, Data, HttpResponse, Json, Path};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WantedListingResponse {
    pub items: Vec<Wanted>,
    pub total: i64,
}

pub async fn wanted_list(pool: Data<PoolType>) -> Result<Json<WantedListingResponse>, ApiError> {

    let items = get_all(&pool);
    let total = count_all(&pool);

    respond_json( WantedListingResponse {
      items: items?,
      total: total?,
    } )
}

pub async fn downloaded_wanted(pool: Data<PoolType>, item_id: Path<i32>) -> Result<HttpResponse, ApiError> {
    block(move || mark_as_downloaded(&pool, *item_id)).await?;
    respond_ok()
}

pub async fn mark_wanted(pool: Data<PoolType>, item_id: Path<i32>) -> Result<HttpResponse, ApiError> {
    block(move || increase_wanted_week(&pool, *item_id)).await?;
    respond_ok()
}
