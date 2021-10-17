use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

use crate::DATA;

pub fn add_route(server: &mut Server) {
    server.route(Method::GET, "/api/option", |req| {
        let q = match req.query.get("q") {
            Some(q) => q,
            None => quick_err!(400, "Parameter 'q' not defiend"),
        };

        let parts = q.split('-').collect::<Vec<&str>>();

        if parts[0] != "0" {
            quick_err!(400, "Invalid q parameter [0]");
        }

        let mut this_question = unsafe { DATA.as_ref().unwrap() };
        for i in parts.iter().skip(1) {
            let i: usize = match i.parse() {
                Ok(i) => i,
                Err(_) => quick_err!(400, "Invalid q parameter (Not a usize)"),
            };

            if i >= this_question.answer.len() {
                quick_err!(400, "Invalid q parameter (Index out of bounds)");
            }
            this_question = &this_question.answer[i];
        }

        let resp = format!("{}", this_question.clone().jsonify());
        Response::new(
            200,
            &resp,
            vec![Header::new("Content-Type", "application/json")],
        )
    });
}
