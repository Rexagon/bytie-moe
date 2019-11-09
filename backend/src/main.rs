use actix_files;
use env_logger;
use actix_web::{Responder, middleware::Logger, web, App, HttpServer};


#[get("/")]
fn index() -> impl Responder { "Hello World!" }


fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
    })
        .bind("0.0.0.0:8000")?
        .run()
}
