use warp::Filter;
use serde::{Deserialize,Serialize};
use std::convert::Infallible;

use crate::entities::Music;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
  pub page: Option<usize>,
}
#[derive(Debug, Serialize)]
pub struct ListResult {
  pub items: Vec<Music>,
  pub total: u32,
}

pub fn endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!( "api" / "music" )
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and_then(handle_list_music)
}

async fn handle_list_music(opts: ListOptions) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of todos, applying the limit and offset.

    let page_size: u32 = 10;
    let page: u32 = opts.page.unwrap_or(0) as u32;

    let result = ListResult {
        items: crate::dao::music::list(page, page_size),
        total: crate::dao::music::total(), 
    };
    Ok(warp::reply::json(&result))
}
