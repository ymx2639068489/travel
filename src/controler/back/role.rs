
use actix_web::{
  delete, get, post, put, web, Responder, Result as Res
};
use serde::Deserialize;

use crate::{
  models::role::*,
  service,
  JwtAdminData,
  DbPool,
  Response,
};

use verify_role::verify_permissions;

#[get("/get_all")]
#[verify_permissions(role, query)]
async fn get_all(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder>{
  let res = service::role::get_role_by_page(&pool).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(res) => Response::ok_list(res)
  })
}

#[post("/insert")]
#[verify_permissions(role, insert)]
async fn add_one_role(
  role: web::Json<AddRoleDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::role::add_one_role(
    &pool,
    role.to_role_dto(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "添加成功"),
    Err(e) => Response::server_error(e),
  })
}

#[put("/update")]
#[verify_permissions(role, query)]
async fn update_one_role(
  role: web::Json<UpdateRoleDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::role::update_one_role(&pool, role.into_inner()).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(e) => Response::server_error(e),
  })
}

#[derive(Deserialize)]
struct TargetRole {
  id: String,
}
#[delete("/delete")]
#[verify_permissions(role, delete)]
async fn delete_one_role(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target_role: web::Query<TargetRole>,
) -> Res<impl Responder> {
  let res = service::role::delete_role_by_id(
    &pool,
    target_role.id.clone()
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "删除成功"),
    Err(e) => Response::server_error(e),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all)
    .service(add_one_role)
    .service(update_one_role)
    .service(delete_one_role)
    ;
}