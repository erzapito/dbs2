#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate log4rs;

//use log::{error, info, warn};

use actix_web::{App,HttpServer,web};

mod actions;
mod dao;
mod db;
mod entities;
mod schema;

fn configure_logger() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    configure_logger();

    HttpServer::new(|| {
        App::new()
          .data( dao::Dao{} )
          .service( web::scope("/")
            .configure( actions::music::endpoints )
          )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
