mod api;

use api::ai::start_explore;
use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger).service(start_explore)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}