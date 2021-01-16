//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    music::{create_music, delete_music, get_music, music_list, update_music},
};
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // /api/v1 routes
        .service(
            web::scope("/api")
                // USER routes
                .service(
                    web::scope("/music")
                        .route("/{id}", web::get().to(get_music))
                        .route("/{id}", web::put().to(update_music))
                        .route("/{id}", web::delete().to(delete_music))
                        .route("", web::get().to(music_list))
                        .route("", web::post().to(create_music)),
                )
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );
}