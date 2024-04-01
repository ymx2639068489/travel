use actix_web::web;

mod user;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/user").configure(user::init_routes));
}