use actix_web::{HttpResponse, web};

use crate::entities::MusicNew;
use crate::dao::Dao;

pub async fn endpoint(item: web::Json<MusicNew>, dao: web::Data<Dao>) -> HttpResponse {
    let music_dao = dao.music_dao();
    music_dao.insert( item.into_inner() );
    HttpResponse::Ok().finish()
}
