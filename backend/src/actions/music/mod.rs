mod list;
mod create;
mod update;
mod delete;

use actix_web::web;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/music")
          .route( web::get().to(list::endpoint) )
          .route( web::post().to(create::endpoint) )
          .route( web::put().to(update::endpoint) )
          .route( web::delete().to(delete::endpoint) )
    );
}
