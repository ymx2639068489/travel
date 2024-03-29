use actix_web::{
  get,
  web,
  post,
  put,
  Responder,
  Result as Res,
};
use verify_role::verify_permissions;
use crate::{
  models::role::*, service, utils::auth::JwtAdminData, DbPool, QueryPager, Response
};

#[get("/get_all")]
#[verify_permissions(role, query)]
async fn get_all(
  jwt_admin_data: JwtAdminData,
  pager: web::Query<QueryPager>,
  pool: web::Data<DbPool>,
) -> Res<impl Responder>{
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || service::role::get_role_by_page(&mut conn, pager.page, pager.per_page)
  ).await?;

  Ok(Response::ok_pager(res))
}

#[post("/insert")]
async fn add_one(role: web::Json<AddRoleDTO>, pool: web::Data<DbPool>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || service::role::add_one_role(&mut conn, &role.to_role_dto())
  ).await?;
  Ok(match res {
    Ok(_) => Response::ok("", "添加成功"),
    Err(_) => Response::server_error("添加失败"),
  })
}

#[put("/update")]
async fn update_one(role: web::Json<UpdateRoleDTO>, pool: web::Data<DbPool>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || service::role::update_one_role(&mut conn, &role)).await?;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}
// #[delete("/delete")]
// async fn delete_one() {

// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all)
    .service(add_one)
    .service(update_one)
    ;
}