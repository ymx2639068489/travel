use actix_web::web;

mod admin;
mod role;
mod company;
mod base_product;
mod product;
mod salesman;
mod custom;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/admin").configure(admin::init_routes))
    .service(web::scope("/role").configure(role::init_routes))
    .service(web::scope("/company").configure(company::init_routes))
    .service(web::scope("/base_product").configure(base_product::init_routes))
    // .service(web::scope("/product").configure(product::init_routes))
    .service(web::scope("/salesman").configure(salesman::init_routes))
    .service(web::scope("/custom").configure(custom::init_routes))
    
  ;
}
