use actix_web::{
  get, post, web, Responder, Result as Res
};
use crate::{
  models::order::*,
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
      // println!("{:?}", list);
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
    Ok(res) => Response::ok(res, ""),
    Err(e) => Response::client_error(e),
  })
}
/**
 * 更新产品
 */
// #[put("/update")]
// #[verify_permissions(product, update)]
// async fn update_product(
//   pool: web::Data<DbPool>,
//   jwt: JwtAdminData,
//   update_product_dto: web::Json<ReqUpdateProductDTO>,
// ) -> Res<impl Responder> {
//   let res = service::product::update_product(
//     &pool,
//     update_product_dto.to_update_product_dto(),
//   ).await;
//   Ok(match res {
//     Ok(_) => Response::ok("", "更新成功"),
//     Err(_) => Response::server_error("更新失败"),
//   })
// }

// #[delete("/delete")]
// #[verify_permissions(product, delete)]
// async fn delete_product(
//   pool: web::Data<DbPool>,
//   jwt: JwtAdminData,
//   target: web::Query<QueryUuid>,
// ) -> Res<impl Responder> {
//   let res = service::product::delete_product(
//     &pool,
//     target.id.clone(),
//   ).await;
//   Ok(match res {
//     Ok(_) => Response::ok("", "删除成功"),
//     Err(e) => Response::server_error(e),
//   })
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    .service(get_total_people_number)
    ;
}
