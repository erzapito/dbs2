use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde::Serialize;

use crate::entities::Music;
use crate::dao::Dao;

#[derive(Deserialize)]
pub struct ListOptions {
  pub page: Option<usize>,
  pub search: Option<String>,
}

#[derive(Serialize)]
pub struct ListResult {
  pub items: Vec<Music>,
  pub total: u32,
}

pub async fn endpoint(opts: web::Query<ListOptions>, dao: web::Data<Dao>) -> HttpResponse {
    // Just return a JSON array of todos, applying the limit and offset.

    let page_size: usize = 10;
    let page: usize = opts.page.unwrap_or(0);
    let search_token = &opts.search;

    let music_dao = dao.music_dao();

    let result = ListResult {
        items: music_dao.list(page, page_size, search_token),
        total: music_dao.total(search_token),
    };
    HttpResponse::Ok().json(result)
}
