use actix_web::{
  get, post, web, put, Responder, Result as Res
};
use verify_role::verify_permissions;

use crate::{
  models::company::*, service, DbPool, Response, JwtAdminData
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
  let res =service::company::insert_one_company(
    &pool,
    company.to_company_dto(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "新增成功"),
    Err(_) => Response::server_error("插入失败"),
  })
}

/**
 * 好像没有更新的必要
 */
// #[put("/update")]
// #[verify_permissions(company, update)]
// async fn update_one(
//   company: web::Json<CompanyDTO>,
//   pool: web::Data<DbPool>,
//   jwt: JwtAdminData,
// ) -> Res<impl Responder> {
//   let res = service::company::update_company(
//     &pool,
//     company.into_inner()
//   ).await;
//   Ok(match res {
//     Ok(_) => Response::ok("", "更新成功"),
//     Err(_) => Response::server_error("更新失败"),
//   })
// }


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all_compnay)
    .service(add_one)
    // .service(update_one)
    ;
}