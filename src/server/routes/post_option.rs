use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

pub fn add_route(server: &mut Server) {
    server.route(Method::POST, "/api/option", |_req| {
        Response::new(200, "Hi", vec![Header::new("Content-Type", "text/plain")])
    });
}
