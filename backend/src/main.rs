pub mod network;
pub mod server;
pub mod actions;

extern crate log;
extern crate log4rs;

use log::{error, info, warn};

pub use server::ServerAction;

fn configure_logger() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}

fn main() {

    configure_logger();

    fastcgi::run(|mut req| {
        let config = server::ServerConfig::new("/dbs2");

        let request = network::from_fast_cgi(&req, &config.base_path);
        
        if request.is_none() {
          return;
        }

        let mut server = server::Server::new( config );

        {
          let r = actions::root::Root{};
          server.add_action( Box::new( r ) );
        }
        {
          let r = actions::not_found::NotFound{};
          server.add_action( Box::new( r ) );
        }

        let mut response = server.process_request(&request.unwrap());
        response.dump(&mut req.stdout());
    });
}
