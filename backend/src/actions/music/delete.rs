use actix_web::{HttpResponse, web};

use crate::entities::Music;
use crate::dao::Dao;

pub async fn endpoint(item: web::Json<Music>, dao: web::Data<Dao>) -> HttpResponse {
    let music_dao = dao.music_dao();
    music_dao.delete( item.into_inner() );
    HttpResponse::Ok().finish()
}
