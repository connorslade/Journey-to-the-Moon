use afire::Server;

mod get_option;
mod post_option;

pub fn add_routes(server: &mut Server) {
    get_option::add_route(server);
    post_option::add_route(server);
}
