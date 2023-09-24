mod api;

use api::ai::{start_explore, initialize};
use actix_web::{HttpServer, App, middleware::Logger, http::header};
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    
    
    HttpServer::new(move || {
        let logger=Logger::default();
        let cors= Cors::default().allow_any_origin().allowed_methods(vec!["GET", "POST"]).allowed_header(http::header::CONTENT_TYPE).send_wildcard();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .service(start_explore)
            .service(initialize)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}