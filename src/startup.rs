use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

//use zero2prod::run;
use super::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
