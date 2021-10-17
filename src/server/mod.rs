use afire::Level;
use afire::Logger;
use afire::Server;

mod routes;
mod serve_static;

pub fn start_server(ip: &str, port: u16) {
    let mut server = Server::new(ip, port);

    Logger::attach(&mut server, Logger::new(Level::Info, None, true));
    serve_static::add_route(&mut server);
    routes::add_routes(&mut server);

    server.start();
}
