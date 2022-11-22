mod lib;
mod user;
mod user_habits;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    user::init(cfg);
    user_habits::init(cfg);
}
