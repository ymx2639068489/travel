
use actix_web::{get, post, put, web, Responder, Result as Res};
use crate::{
  models::{
    user::*,
    RemoveImportInformation
  }, service, DbPool, JwtUserData, Response
};

#[post("/login")]
async fn login(
  user: web::Json<LoginUserDTO>,
  pool: web::Data<DbPool>,
) -> Res<impl Responder> {
  let res = service::user::user_login(
    &pool,
    user.into_inner(),
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(jwt) => Response::ok(jwt, "登录成功")
  })
}

#[get("/getinfo")]
async fn get_info(
  jwt: JwtUserData,
  pool: web::Data<DbPool>,
) -> Res<impl Responder> {
  let res = service::user::query_user_by_id(
    &pool,
    jwt.id
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(user) => Response::ok(user.remove_import_information(), "")
  })
}

#[post("/register")]
async fn register(
  user: web::Json<RegisterUserDTO>,
  pool: web::Data<DbPool>,
) -> Res<impl Responder> {
  let res = service::user::add_one_user(
    &pool,
    user.into_inner()
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(_) => Response::ok("", "注册成功")
  })
}


#[put("/update")]
async fn update_profile(
  mut user: web::Json<UpdateUserDTO>,
  pool: web::Data<DbPool>,
  ud: JwtUserData
) -> Res<impl Responder> {
  // 防止非法
  user.id = ud.id;
  let res = service::user::update_user(&pool, user.into_inner())
    .await;
  Ok(match res {
    Ok(n) => Response::ok(n, "更新成功"),
    Err(_) => Response::server_error("Error updating user")
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(login)
    .service(get_info)
    .service(register)
    .service(update_profile)
    ;
}