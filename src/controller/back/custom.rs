use actix_web::{
  get, put, web, Responder, Result as Res
};
use verify_role::verify_permissions;

use crate::{
  models::user::*, service, DbPool, JwtAdminData, Response
};

#[get("/get_list")]
#[verify_permissions(custom, query)]
async fn get_list(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  pager: web::Query<UserQueryDTO>,
) -> Res<impl Responder> {
  let res = service::user::get_list(
    &pool,
    pager.into_inner(),
  ).await;
  Ok(match res {
    Ok(list) => Response::ok_pager(list),
    Err(e) => Response::server_error_list(e),
  })
}

#[put("/update")]
#[verify_permissions(custom, delete)]
async fn delete_one_custom(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target_custom: web::Json<UpdateUserDTO>
) -> Res<impl Responder> {
  let res = service::user::update_user(
    &pool,
    target_custom.into_inner(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(e) => Response::server_error(e),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    .service(delete_one_custom)
    ;
}