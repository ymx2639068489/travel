use actix_web::{
  delete, get, post, web, Responder, Result as Res
};
use crate::{
  models::{order::*, QueryId},
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
#[verify_permissions(sales_records, query)]
async fn get_list(
  pager: web::Query<OrderQueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::order::get_list(
    &pool,
    pager.into_inner()
  ).await;
  Ok(match res {
    Ok(res) => {
      let ResponseList { page, page_size, total, data } = res;
      let list = data
        .iter()
        .map(|item| item.to_res_dto())
        .collect();
      Response::ok_pager(ResponseList {
        data: list,
        page,
        page_size,
        total
      })
    },
    Err(e) => Response::client_error(e),
  })
}

#[get("/get_people_sum")]
#[verify_permissions(sales_records, query)]
async fn get_total_people_number(
  pager: web::Query<OrderQueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::order::get_total_number(
    &pool,
    pager.into_inner()
  ).await;
  Ok(match res {
    Ok(res) => Response::ok(res, ""),
    Err(e) => Response::client_error(e),
  })
}


#[post("/upload")]
#[verify_permissions(sales_records, insert)]
async fn upload_order(
  order_list: web::Json<Vec<ReqAddOrderDTO>>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::order::insert_order(
    &pool,
    order_list.into_inner(),
  ).await;
  Ok(match res {
    Ok(res) => Response::ok_list(res),
    Err(e) => Response::client_error(e),
  })
}

#[delete("/delete")]
#[verify_permissions(sales_records, delete)]
async fn delete_item(
  jwt: JwtAdminData,
  pool: web::Data<DbPool>,
  target_order_id: web::Query<QueryId>
) -> Res<impl Responder> {
  let res = service::order::delete_item_order(
    &pool,
    target_order_id.id,
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "删除成功"),
    Err(e) => Response::client_error(e),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    .service(get_total_people_number)
    .service(delete_item)
    .service(upload_order)
    ;
}
