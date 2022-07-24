use std::thread;
use std::time::Duration;

use crate::request::Request;
use crate::response::Response;
use crate::route::{parse_template, Route};
use crate::server::{Router, Routing};

pub fn main_router() -> Router {
    let mut router = Router::new();

    // Index route
    let home_route = Route::new("/", |_request: Request| {
        println!("Home request handler");
        let template = parse_template("templates/index.html");
        let response = Response::new(200, &template);
        response
    });

    let cool_route = Route::new("/cool", |_request: Request| {
        println!("Cool request handler");
        let template = parse_template("templates/cool.html");
        let response = Response::new(200, &template);
        response
    });

    let sleep_route = Route::new("/sleep", |_request: Request| {
        println!("Sleep request handler");
        thread::sleep(Duration::from_secs(20));
        let response = Response::new(200, "");
        response
    });

    router.add_route(home_route);
    router.add_route(cool_route);
    router.add_route(sleep_route);

    router
}
