use actix_web::{
  delete, get, post, put, web, Responder, Result as Res
};
use crate::{
  models::{
    admin::*,
    RemoveImportInformation,
  },
  service,
  DbPool,
  JwtAdminData,
  Response,
};
use verify_role::verify_permissions;
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


/**
 * 获取管理员列表
 */
#[get("/get_list")]
#[verify_permissions(admin, query)]
async fn get_admin(
  pager: web::Query<AdminQueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::admin::get_admin_list(
    &pool,
    pager.into_inner()
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(list) => Response::ok_pager(list),
  })
}


/**
 * 更新管理员
 */
#[put("/update")]
#[verify_permissions(admin, update)]
async fn update_admin(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target_admin_dto: web::Json<UpdateAdminDTO>,
) -> Res<impl Responder> {
  let res = service::admin::update_admin_by_id(
    &pool,
    target_admin_dto.into_inner()
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}

/**
 * 删除管理员
 */
#[delete("/delete")]
#[verify_permissions(admin, delete)]
async fn delete_admin(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target_admin_id: web::Query<String>,
) -> Res<impl Responder> {
  let res = service::admin::delete_admin_by_id(
    &pool,
    target_admin_id.into_inner()
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "删除成功"),
    Err(_) => Response::server_error("删除失败"),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(login)
    .service(getinfo)
    .service(get_admin)
    .service(update_admin)
    .service(delete_admin)
    ;
}