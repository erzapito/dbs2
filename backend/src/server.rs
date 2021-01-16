//! Spin up a HTTPServer

use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_pool)
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}
