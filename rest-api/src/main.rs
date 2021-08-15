mod like;
mod response;
mod tweet;

use actix_web::{middleware, App, HttpServer};
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(tweet::list)
            .service(like::list)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
