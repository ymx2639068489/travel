use actix_web::{
  get,
  post,
  web,
  Result as Res,
  Responder,
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


#[post("/login")]
async fn login(login_user: web::Json<AdminLogin>, pool: web::Data<DbPool>) -> Res<impl Responder> {
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
#[verify_permissions(admin, query)]
async fn getinfo(
  jwt_admin_data: JwtAdminData,
  pool: web::Data<DbPool>
) -> Res<impl Responder> {
  let mut conn = pool.get().expect("");
  let q_user = web::block(
    move || service::admin::query_admin_by_id(&mut conn, &jwt_admin_data.id)
  ).await?;
  Ok(match q_user {
    Ok(user) => Response::ok(user, "获取成功"),
    Err(_) => Response::client_error("查无此管理")
  })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(login)
    .service(getinfo)
    ;
}