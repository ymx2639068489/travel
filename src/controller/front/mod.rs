use actix_web::web;

mod user;
mod product;
mod order;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/user").configure(user::init_routes))
    .service(web::scope("/product").configure(product::init_routes))
    .service(web::scope("/order").configure(order::init_routes))
    ;
}