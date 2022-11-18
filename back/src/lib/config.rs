use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub auth_origin: String,
    pub frontend_origins: Vec<String>,
}

impl Config {
    fn new() -> Self {
        let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
        let port = std::env::var("PORT").unwrap_or("9099".into());
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let auth_origin = std::env::var("AUTH_ORIGIN").unwrap_or("http://127.0.0.1:9099".into());
        let frontend_origins: Vec<String> = std::env::vars()
            .into_iter()
            .filter(|v| v.0.starts_with("FRONTEND_ORIGIN_"))
            .map(|v| v.1)
            .collect();

        Self {
            host,
            port,
            database_url,
            auth_origin,
            frontend_origins,
        }
    }
}
