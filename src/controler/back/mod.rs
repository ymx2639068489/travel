use actix_web::web;

mod admin;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/admin").configure(admin::init_routes));
}
