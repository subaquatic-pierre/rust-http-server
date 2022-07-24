use std::collections::HashMap;
use std::io::Error;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::logger::Logger;
use crate::request::Request;
use crate::response::Response;
use crate::route::{parse_template, Route};
use crate::worker::ThreadPool;

pub struct Server {
    port: String,
    routes: HashMap<String, Route>,
    logger: Logger,
    workers: ThreadPool,
}

impl Server {
    pub fn init(port: &str, num_workers: i8) -> Result<Self, Error> {
        Ok(Server {
            port: port.to_string(),
            routes: HashMap::new(),
            logger: Logger {},
            workers: ThreadPool::new(num_workers).unwrap(),
        })
    }

    pub fn listener(&self) -> TcpListener {
        TcpListener::bind(&self.port).expect(&format!("Unable to bind to port {:}", self.port))
    }

    pub fn run(&self) -> Result<(), Error> {
        self.logger
            .log(&format!("Server started, listening at {} ...", self.port));

        // Get reference to connection from self
        let listener = self.listener();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let request = self.get_request(&stream);
            let route = self.get_route(&request);

            self.workers.execute(move || {
                Server::handle_route(&mut stream, route, request);
            })
        }

        Ok(())
    }

    pub fn register_router(&mut self, router: Router) {
        for route in router.routes {
            self.routes.insert(route.path().to_string(), route);
        }
    }

    fn handle_route(stream: &mut TcpStream, route: Option<Route>, request: Request) {
        let response = match route {
            Some(route) => route.handle_request(request),
            None => {
                let template = parse_template("templates/400.html");
                Response::new(400, &template)
            }
        };

        stream.write(response.format().as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn get_request(&self, stream: &TcpStream) -> Request {
        let request = Request::new(stream);
        // self.logger
        //     .log(&format!("\n--- Request ---\n\n{}---", &request.content()));
        request
    }

    fn get_route(&self, request: &Request) -> Option<Route> {
        let path = request.path();
        if let Some(route) = self.routes.get(&path) {
            return Some(route.clone());
        } else {
            return None;
        };
    }
}

pub trait Routing {
    fn add_route(&mut self, route: Route);
}

impl Routing for Server {
    fn add_route(&mut self, route: Route) {
        self.routes.insert(route.path().to_string(), route);
    }
}

pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
}

impl Routing for Router {
    fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }
}
