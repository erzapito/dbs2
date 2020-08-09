mod list;

use actix_web::web;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/music")
          .route( web::get().to(list::endpoint) )
    );
}
