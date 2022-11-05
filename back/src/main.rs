mod controller;
mod lib;

use crate::lib::{config::CONFIG, cors::cors};

use actix_web::{get, middleware::Logger, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertion) {
        dotenv::dotenv().ok();
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("into"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            .configure(controller::init)
            .service(index)
    })
    .bind(format!("{}:{}", CONFIG.host, CONFIG.port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    format!("HELLO WORLD!")
}
