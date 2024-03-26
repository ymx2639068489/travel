use actix_web::{
  get,
  web,
  Result,
};

#[get("/login")]
async fn login() -> Result<String> {
  Ok(format!("back"))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(login);
}