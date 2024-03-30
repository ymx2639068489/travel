use actix_web::{
  get, post, web, put, Responder, Result as Res
};
use add_pool_args::add_pool_args;
use verify_role::verify_permissions;

use crate::{
  models::company::*, service, DbPool, Response, JwtAdminData
};


#[get("/get_all")]
#[add_pool_args]
#[verify_permissions(company, query)]
async fn get_all_compnay() -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    service::company::query_all_company(&mut conn)
  }).await?;
  Ok(match res {
    Ok(list) => Response::ok_list(list),
    Err(_) => Response::server_error_list("error get"),
  })
}

#[post("/insert")]
#[add_pool_args]
#[verify_permissions(company, insert)]
async fn add_one(company: web::Json<AddCompanyDTO>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    service::company::add_company(&mut conn, &company.to_company_dto())
  }).await?;
  Ok(match res {
    Ok(_) => Response::ok("", "新增成功"),
    Err(_) => Response::server_error("插入失败"),
  })
}


#[put("/update")]
#[add_pool_args]
#[verify_permissions(company, update)]
async fn update_one(company: web::Json<CompanyDTO>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    service::company::update_company(&mut conn, &company)
  }).await?;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all_compnay)
    .service(add_one)
    .service(update_one)
    ;
}