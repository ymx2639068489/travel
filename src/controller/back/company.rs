use actix_web::{
  get, post, web, Responder, Result as Res, delete
};
use verify_role::verify_permissions;

use crate::{
  models::{company::*, QueryUuid}, service, DbPool, JwtAdminData, Response
};

#[get("/get_all")]
#[verify_permissions(company, query)]
async fn get_all_compnay(pool: web::Data<DbPool>, jwt: JwtAdminData) -> Res<impl Responder> {
  let res = service::company::get_all_company(&pool).await;
  Ok(match res {
    Ok(list) => Response::ok_list(list),
    Err(e) => Response::server_error_list(e),
  })
}

#[post("/insert")]
#[verify_permissions(company, insert)]
async fn add_one(
  company: web::Json<AddCompanyDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::company::insert_one_company(
    &pool,
    company.to_company_dto(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "新增成功"),
    Err(e) => Response::server_error(e),
  })
}


#[delete("/delete")]
#[verify_permissions(company, delete)]
async fn delete_one_company(
  company: web::Query<QueryUuid>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::company::delete_company(
    &pool,
    company.id.clone(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "删除成功"),
    Err(e) => Response::server_error(e),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all_compnay)
    .service(add_one)
    .service(delete_one_company)
    ;
}