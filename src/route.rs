use std::fs;

use crate::request::Request;
use crate::response::Response;

type RequestHandler = fn(Request) -> Response;
#[derive(Clone)]
pub struct Route {
    path: String,
    handler: RequestHandler,
}

impl Route {
    pub fn new(path: &str, handler: RequestHandler) -> Self {
        Route {
            path: path.to_string(),
            handler: handler,
        }
    }

    pub fn path(&self) -> &str {
        &self.path[..]
    }

    pub fn handle_request(&self, request: Request) -> Response {
        // Forward request object to request handler
        let response = (self.handler)(request);
        response
    }
}

pub fn parse_template(template_path: &str) -> String {
    let content = fs::read_to_string(template_path).expect("Unable to open file");
    content.clone()
}
