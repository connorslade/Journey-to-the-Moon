use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

use crate::DATA;

pub fn add_route(server: &mut Server) {
    server.route(Method::GET, "/api/option", |req| {
        let data = unsafe { DATA.as_ref().unwrap() };

        let q = match req.query.get("q") {
            Some(q) => q,
            None => quick_res!(400, "Parameter 'q' not defiend"),
        };

        let parts = q.split('-').collect::<Vec<&str>>();

        let resp = format!("{:?}", data);
        Response::new(200, &resp, vec![Header::new("Content-Type", "text/plain")])
    });
}
