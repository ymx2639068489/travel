// use actix_web::{
//   get,
//   web,
//   Result as Res,
//   Responder,
// };
// use crate::{
//   models::admin::*,
//   DbPool
// };
// #[get("/login")]
// async fn login(loginUser: web::Json<AdminLogin>, pool: web::Data<DbPool>) -> Res<impl Responder> {
//   let password = user.password.clone();
//   // 通过手机号查询用户
//   let q_user = web::block(move || {
//     let mut conn = pool.get()
//       .expect("couldn't get db connection");
//     service::user::query_user_by_phone(&mut conn, &user.phone)
//   })
//   .await?;
// }

// #[get("/getinfo")]
// async fn getinfo() -> Res<impl Responder> {

// }


// pub fn init_routes(cfg: &mut web::ServiceConfig) {
  // cfg.service(login);
// }