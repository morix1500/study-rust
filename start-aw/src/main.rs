use actix_web::{server, App, HttpRequest, Responder};
use actix_web::{Error, FromRequest, Path};
use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello_name(to: Path<HelloPath>) -> Result<String, Error> {
    Ok(format!("Hello, {}!", &to.name))
}

fn hello(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(hello))
            .resource("/{name}", |r| r.with(hello_name))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 30000")
    .run();
}
