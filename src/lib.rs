pub mod logger;
pub mod request;
pub mod response;
pub mod route;
pub mod routes;
pub mod server;
pub mod worker;

use routes::main_router;
use server::Server;

pub fn run() {
    // Initialize server with port number and number of workers
    let mut server =
        Server::init("127.0.0.1:7878", 5).expect("There was an error starting the server");

    let main_router = main_router();
    server.register_router(main_router);

    // Run the server
    server.run().expect("There was an error running server");
}
