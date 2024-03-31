use actix_web::{
  get,
  web,
  post,
  put,
  Responder,
  Result as Res,
};
use crate::{
  models::role::*,
  service,
  JwtAdminData,
  DbPool,
  QueryPager,
  Response,
};
use verify_role::verify_permissions;

#[get("/get_all")]
#[verify_permissions(role, query)]
async fn get_all(
  pager: web::Query<QueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder>{
  let res = service::role::get_role_by_page(
    &pool,
    pager.page,
    pager.per_page
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(res) => Response::ok_pager(res)
  })
}

#[post("/insert")]
#[verify_permissions(role, insert)]
async fn add_one(
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
    Err(_) => Response::server_error("添加失败"),
  })
}

#[put("/update")]
#[verify_permissions(role, query)]
async fn update_one(
  role: web::Json<UpdateRoleDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::role::update_one_role(&pool, role.into_inner()).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all)
    .service(add_one)
    .service(update_one)
    ;
}