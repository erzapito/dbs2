use crate::handlers::{
    music::{create_music, delete_music, get_music, music_list, update_music},
    series::{create_series, delete_series, get_series, series_list, update_series},
};
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // /api/v1 routes
        .service(
            web::scope("/api")
                // MUSIC routes
                .service(
                    web::scope("/music")
                        .route("/{id}", web::get().to(get_music))
                        .route("/{id}", web::put().to(update_music))
                        .route("/{id}", web::delete().to(delete_music))
                        .route("", web::get().to(music_list))
                        .route("", web::post().to(create_music)),
                )
                // SERIES routes
                .service(
                    web::scope("/series")
                        .route("/{id}", web::get().to(get_series))
                        .route("/{id}", web::put().to(update_series))
                        .route("/{id}", web::delete().to(delete_series))
                        .route("", web::get().to(series_list))
                        .route("", web::post().to(create_series)),
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
