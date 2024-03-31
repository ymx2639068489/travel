use actix_web::{
  get,
  post,
  web,
  Responder,
  Result as Res
};
use crate::{
  models::{admin::*, RemoveImportInformation}, service, DbPool, JwtAdminData, Response
};

#[post("/login")]
async fn login(admin_login: web::Json<AdminLogin>, pool: web::Data<DbPool>) -> Res<impl Responder> {
  let res = service::admin::admin_login(&admin_login, pool)
    .await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(jwt) => Response::ok(jwt, "登录成功"),
  })
}

#[get("/getinfo")]
async fn getinfo(pool: web::Data<DbPool>, jwt: JwtAdminData) -> Res<impl Responder> {
  let res = service::admin::get_admin_by_id(
    &pool,
    jwt.id.clone()
  ).await;
  Ok(match res {
    Ok(user) => Response::ok(user.remove_import_information(), "获取成功"),
    Err(_) => Response::client_error("error")
  })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(login)
    .service(getinfo)
    ;
}