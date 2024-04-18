use verify_role::verify_permissions;

use actix_web::{
  delete, get, post, put, web, Responder, Result as Res
};
use crate::{
  models::{salesman::*, QueryUuid}, service, DbPool, JwtAdminData, Response
};

#[get("/get_list")]
#[verify_permissions(salesman, query)]
pub async fn get_list(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  pager: web::Query<SalesmanQueryPager>,
) -> Res<impl Responder> {
  let res = service::salesman::get_salesman_list(
    &pool,
    pager.into_inner()
  ).await;

  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(list) => {
      Response::ok_pager(list)
    },
  })
}

#[post("/insert")]
#[verify_permissions(salesman, insert)]
async fn add_one_salesman(
  target_salesman: web::Json<AddSalesmanDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::salesman::insert_salesman(
    &pool,
    target_salesman.into_inner(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "添加成功"),
    Err(e) => Response::server_error(e),
  })
}

#[put("/update")]
#[verify_permissions(salesman, query)]
async fn update_one_salesman(
  target_salesman: web::Json<UpdateSalesmanDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::salesman::update_salesman(
    &pool,
    target_salesman.into_inner()
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(e) => Response::server_error(e),
  })
}

#[delete("/delete")]
#[verify_permissions(salesman, delete)]
async fn delete_one_role(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target_salesman: web::Query<QueryUuid>,
) -> Res<impl Responder> {
  let res = service::role::delete_role_by_id(
    &pool,
    target_salesman.id.clone()
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "删除成功"),
    Err(e) => Response::server_error(e),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    .service(add_one_salesman)
    .service(update_one_salesman)
    .service(delete_one_role)
    ;
}