use actix_web::{
  get,
  post,
  web,
  Responder,
  Result as Res
};
use crate::{
  models::admin::*,
  service,
  utils,
  utils::auth::JwtAdminData,
  DbPool,
  Response
};
use verify_role::verify_permissions;
use add_pool_args::add_pool_args;

#[post("/login")]
#[add_pool_args]
async fn login(login_user: web::Json<AdminLogin>) -> Res<impl Responder> {
  let mut conn = pool.get().expect("");
  let username = login_user.username.clone();
  let q_user = web::block(
    move || service::admin::query_admin_by_username(&mut conn, &username)
  ).await?;
  
  Ok(match q_user {
      Ok(user) => {
        if user.password.eq(&login_user.password) {
          Response::ok(utils::auth::back_auth::create_jwt(&user), "登录成功")
        } else {
          Response::client_error("密码错误")
        }
      },
      Err(_) => Response::client_error("查无此管理")
  })
}

#[get("/getinfo")]
#[add_pool_args]
#[verify_permissions(admin, query)]
async fn getinfo() -> Res<impl Responder> {
  let mut conn = pool.get().expect("");
  let q_user = web::block(
    move || service::admin::query_admin_by_id(&mut conn, &jwt_admin_data.id)
  ).await?;
  Ok(match q_user {
    Ok(user) => Response::ok(user, "获取成功"),
    Err(_) => Response::client_error("查无此管理")
  })
  // Ok(Response::ok("", "获取成功"))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(login)
    .service(getinfo)
    ;
}