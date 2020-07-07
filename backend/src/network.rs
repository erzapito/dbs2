extern crate fastcgi;
extern crate log;

use std::collections::HashMap;
use std::io::Write;

pub struct Response {
  pub headers: HashMap<String,String>,
  pub content: String,
  pub code: i32,
  pub message: String,
}

impl Response {

  pub fn new_default() -> Response {
    let mut r = Response{
      content: "".to_string(),
      code: 200,
      message: "OK".to_string(),
      headers: HashMap::new(),
    };
    r.headers.insert(String::from("Content-Type"),String::from("text/plain"));
    r
  }

  pub fn new(c: i32, msg: &str) -> Response {
    let mut r = Response{
      content: "".to_string(),
      code: c,
      message: msg.to_string(),
      headers: HashMap::new(),
    };
    r.headers.insert(String::from("Content-Type"),String::from("text/plain"));
    r
  }

  pub fn header (&mut self, h: &str, v: &str) {
    self.headers.insert(String::from(h),String::from(v));
  }

  pub fn dump(&mut self, out: &mut dyn Write) {
    self.header("Status", &format!("{} {}",self.code, self.message));
    
    for (key, value) in &self.headers {
        let r = write!(out, "{}: {}\r\n", key, value);
        if let Err(e) = r {
          log::error!("Error writing rhe response: {}", e);
        }
    }
    let r = out.write(b"\r\n");
    if let Err(e) = r {
      log::error!("Error writing rhe response: {}", e);
    }
    let r = out.write(self.content.as_bytes());
    if let Err(e) = r {
      log::error!("Error writing rhe response: {}", e);
    }
  }

}

pub struct Request {
  pub method: String,
  pub path: String,
  pub remote_addr: Option<String>,
  pub headers: HashMap<String,String>,
}

impl Request {
  pub fn get_info(&self) -> String {
    let mut r = "".to_owned();
    {
      r.push_str("METHOD: ");
      r.push_str(&self.method);
    }
    {
      r.push_str("<BR/>");
      r.push_str("REMOTE ADDR: ");
      r.push_str(self.remote_addr.as_deref().unwrap_or(""));
    }
    {
    r.push_str("<BR/>");
      r.push_str("HEADERS:");
      for (key, value) in &self.headers {
        r.push_str("<BR/> -");
        r.push_str(&key);
        r.push_str(": ");
        r.push_str(&value);
      };
    }
    r
  }
}

pub fn from_fast_cgi<'a>(raw_request: & fastcgi::Request, base_path : &str) -> Option<Request> {
  if raw_request.param("REQUEST_METHOD").is_none() {
    return None
  }

  let mut path = raw_request.param("DOCUMENT_URI").unwrap_or("/".to_string());

  if path.starts_with( base_path ) {
    path = path.chars().skip( base_path.len() ).collect();
  }

  let fcgi_params =  raw_request.params();
  let mut request = Request{
    remote_addr: raw_request.param("REMOTE_ADDR"),
    method: raw_request.param("REQUEST_METHOD").unwrap(),
    headers: HashMap::new(),
    path: path,
//** HTTP_ACCEPT -> */* 
//** HTTP_HOST -> 127.0.0.1 
//** CONTENT_LENGTH ->  
//** SERVER_NAME -> localhost 
//** SCRIPT_NAME -> /dbs2 
//** REMOTE_PORT -> 60126 
//** SERVER_PORT -> 80 
//** REQUEST_URI -> /dbs2?a=b 
//** SERVER_PROTOCOL -> HTTP/1.1 
//** QUERY_STRING -> a=b 
//** SCRIPT_FILENAME -> /var/www/localhost/htdocs/dbs2 
//** SERVER_ADDR -> 127.0.0.1 
//** GATEWAY_INTERFACE -> CGI/1.1 
//** CONTENT_TYPE ->  
//** HTTP_USER_AGENT -> curl/7.69.1 
//** DOCUMENT_ROOT -> /var/www/localhost/htdocs 
//** SERVER_SOFTWARE -> nginx 
//** DOCUMENT_URI -> /dbs2 

  };

  // HEADERS
  for (key, value) in fcgi_params {
    if key.starts_with("HTTP_") {
      request.headers.insert(key[5..].to_string(), value);
    }
  }

  Some(request)
}
