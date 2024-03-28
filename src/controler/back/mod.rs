use actix_web::web;

mod admin;
mod role;
mod company;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    // .service(web::scope("/admin").configure(admin::init_routes))
    .service(web::scope("/role").configure(role::init_routes))
    .service(web::scope("/company").configure(company::init_routes))
  ;
}
