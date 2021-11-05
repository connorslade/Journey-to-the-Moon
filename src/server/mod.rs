use afire::Header;
use afire::Logger;
use afire::Server;

use crate::VERSION;

mod routes;
mod serve_static;

pub fn start_server(ip: &str, port: u16) {
    let mut server = Server::new(ip, port);

    server.add_default_header(Header::new("Version", VERSION));

    Logger::new().attach(&mut server);
    serve_static::add_route(&mut server);
    routes::add_routes(&mut server);

    server.start().unwrap();
}
