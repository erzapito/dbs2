use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde::Serialize;

use crate::entities::Music;

#[derive(Deserialize)]
pub struct ListOptions {
  pub page: Option<usize>,
}

#[derive(Serialize)]
pub struct ListResult {
  pub items: Vec<Music>,
  pub total: u32,
}

pub async fn endpoint(opts: web::Query<ListOptions>) -> HttpResponse {
    // Just return a JSON array of todos, applying the limit and offset.

    let page_size: usize = 10;
    let page: usize = opts.page.unwrap_or(0);

    let result = ListResult {
        items: crate::dao::music::list(page, page_size),
        total: crate::dao::music::total(), 
    };
    HttpResponse::Ok().json(result)
}
