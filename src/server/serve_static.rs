use afire::Header;
use afire::Response;
use afire::Server;
use std::fs;

/// Dir to find files to serve
const DATA_DIR: &str = "data/web/static";

pub fn add_route(server: &mut Server) {
    server.all(|req| {
        let mut path = format!("{}{}", DATA_DIR, req.path.replace("/..", ""));

        // Add Index.html if path ends with /
        if path.ends_with('/') {
            path.push_str("index.html");
        }

        // Also add '/index.html' if path dose not end with a file
        if !path.split('/').last().unwrap_or_default().contains('.') {
            path.push_str("/index.html");
        }

        // Try to read File
        match fs::read(&path) {
            // If its found send it as response
            Ok(content) => Response::new()
                .bytes(content)
                .header(Header::new("Content-Type", get_type(&path))),

            // If not send 404.html
            Err(_) => Response::new()
                .status(404)
                .text(
                    fs::read_to_string("template/404.html")
                        .unwrap_or_else(|_| "Not Found :/".to_owned())
                        .replace("{{PAGE}}", &req.path),
                )
                .header(Header::new("Content-Type", "text/html")),
        }
    });
}

/// Get MMIE type from file extition
fn get_type(path: &str) -> &str {
    match path.split('.').last() {
        Some(ext) => match ext {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "ico" => "image/x-icon",
            "svg" => "image/svg+xml",
            "glb" => "model/gltf-binary",
            _ => "application/octet-stream",
        },

        None => "application/octet-stream",
    }
}
