use actix_web::{dev::Server, web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;
use std::net::TcpListener;

//use zero2prod::run;
use super::routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection1 = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection1.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
