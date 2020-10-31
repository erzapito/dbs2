//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    auth::{login, logout},
    health::get_health,
    music::{create_music, delete_music, get_music, music_list, update_music},
};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_files::Files;
use actix_web::web;

pub fn auth_endpoints(cfg: &mut web::ServiceConfig) {
  cfg
        .route("/login", web::post().to(login))
        .route("/logout", web::get().to(logout));
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api")
                // AUTH routes
                .service(
                    web::scope("/auth")
                        .configure( auth_endpoints )
                )
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
        // Serve secure static files from the static-private folder
        .service(
            web::scope("/secure").wrap(AuthMiddleware).service(
                Files::new("", "./static-secure")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
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
