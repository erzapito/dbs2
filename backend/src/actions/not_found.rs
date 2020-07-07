#[path = "../server.rs"] pub mod server;
#[path = "../network.rs"] pub mod network;

use network::Request;
use network::Response;
pub use server::ServerAction;

pub struct NotFound {
}

impl ServerAction for NotFound {

  fn name(&self) -> &'static str {
    "NOT FOUND"
  }

  fn request_method(&self) -> &'static str {
    "GET"
  }

  fn request_path(&self) -> &'static str {
    "*"
  }

  fn can_process(&self, request: &Request) -> bool {
    true
  }

  fn run(&self, _request: &Request) -> Response {
    let r = Response::new(404, "NotFound");
    r
  }

}
