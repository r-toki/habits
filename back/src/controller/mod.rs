mod daily_records;
mod habits;
mod lib;
mod me;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    me::init(cfg);
    habits::init(cfg);
    daily_records::init(cfg);
}
