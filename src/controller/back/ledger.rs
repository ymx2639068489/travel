use actix_web::{
  put, get, post, web, Responder, Result as Res
};
use crate::{
  models::ledger::*,
  service,
  DbPool,
  JwtAdminData,
  Response,
  ResponseList,
};
use verify_role::verify_permissions;
/**
 * 获取产品列表
 */
#[get("/get_list")]
#[verify_permissions(ledger, query)]
async fn get_list(
  pager: web::Query<LedgerQueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::ledger::get_ledger_list(
    &pool,
    pager.into_inner()
  ).await;
  Ok(match res {
    Ok(res) => {
      let ResponseList { page, page_size, total, data } = res;
      Response::ok_pager(ResponseList {
        data: data
          .iter()
          .map(|item| item.to_res_dto())
          .collect(),
        page,
        page_size,
        total
      })
    },
    Err(e) => Response::client_error(e),
  })
}

#[post("/add_ledger")]
#[verify_permissions(ledger, insert)]
async fn add_one_ledger(
  order_list: web::Json<ReqAddLedgerDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::ledger::add_one_ledger(
    &pool,
    order_list.into_inner(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "统计成功"),
    Err(e) => Response::client_error(e),
  })
}

#[put("/update")]
#[verify_permissions(ledger, update)]
async fn update_one_ledger(
  order_list: web::Json<ReqUpdateLedgerDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::ledger::update_ledger(
    &pool,
    order_list.into_inner(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "统计成功"),
    Err(e) => Response::client_error(e),
  })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    .service(add_one_ledger)
    ;
}
