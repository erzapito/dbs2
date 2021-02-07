use crate::handlers::{
    music::{create_music, delete_music, get_music, music_list, update_music},
    series::{create_series, delete_series, get_series, series_list, update_series},
    wanted::{downloaded_wanted, mark_wanted, wanted_list}
};
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dbs2")
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
                        .route("", web::post().to(create_music))
                )
                // SERIES routes
                .service(
                    web::scope("/series")
                        .route("/{id}", web::get().to(get_series))
                        .route("/{id}", web::put().to(update_series))
                        .route("/{id}", web::delete().to(delete_series))
                        .route("", web::get().to(series_list))
                        .route("", web::post().to(create_series))
                )
                // WANTED routes
                .service(
                    web::scope("/wanted")
                        .route("/{id}/mark", web::post().to(mark_wanted))
                        .route("/{id}/downloaded", web::post().to(downloaded_wanted))
                        .route("", web::get().to(wanted_list))
                )
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
    );
}
