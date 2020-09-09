extern crate http;

use http::{Request, Response, StatusCode};

fn main() {
    let _request = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "awesome/1.0")
        .body(())
        .unwrap();

    let _response = Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", "https://www.rust-lang.org/install.html")
        .body(())
        .unwrap();
}
