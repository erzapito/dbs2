#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate log4rs;

//use log::{error, info, warn};

mod actions;
mod dao;
mod db;
mod entities;
mod schema;

fn configure_logger() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}

#[tokio::main]
async fn main() {
    configure_logger();

    let music = actions::music::endpoints();

    let routes = music;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

}
