#[path = "./network.rs"] pub mod network;

use network::Request;
use network::Response;

use std::vec::Vec;

pub struct ServerConfig {
  pub base_path: String,
}

impl ServerConfig {
  pub fn new(base_path: &str) -> ServerConfig {
    ServerConfig {
      base_path: base_path.to_string(),
    }
  }

  pub fn set_base_path(&mut self, s : &str) -> &ServerConfig {
    self.base_path = s.to_string();
    self
  }

}

pub trait ServerAction {
  fn name(&self) -> &'static str;
  fn request_method(&self) -> &'static str;
  fn request_path(&self) -> &'static str;

  fn run(&self, request: &Request) -> Response;

  fn can_process(&self, request: &Request) -> bool {
    if self.request_method() != request.method {
      return false;
    }
    if self.request_path() != request.path {
      return false;
    }
    return true;
  }
}

pub struct Server {
  pub config: ServerConfig,
  pub actions: Vec<Box<dyn ServerAction>>,
}

impl Server {

  pub fn new(config: ServerConfig) -> Server {
    Server{
      config,
      actions: Vec::new(),
    }
  }

  pub fn add_action(&mut self, action: Box<dyn ServerAction>) {
    self.actions.push(action);
  }

  pub fn process_request(&self, request : &Request) -> Response {
    for action in self.actions.iter() {
      if action.can_process(request) {
        return action.run(request);
      }
    }
    Response::new_default()
  }

}
