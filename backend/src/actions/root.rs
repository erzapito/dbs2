#[path = "../server.rs"] pub mod server;
#[path = "../network.rs"] pub mod network;

use network::Request;
use network::Response;
pub use server::ServerAction;

pub struct Root {
}

impl ServerAction for Root {

  fn name(&self) -> &'static str {
    "ROOT"
  }

  fn request_method(&self) -> &'static str {
    "GET"
  }

  fn request_path(&self) -> &'static str {
    "/"
  }

  fn run(&self, _request: &Request) -> Response {
    let mut r = Response::new_default();
    r.header("Content-Type","text/html");
    r.content = "<html><body>ROOT</body></html>".to_string();
    r
  }

}
