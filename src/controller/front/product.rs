
use actix_web::{get, post, web, Responder, Result as Res};
use crate::{
  models::{order::ReqBuyProductDTO, product::*, QueryUuid},
  service, DbPool, JwtUserData, Response, ResponseList
};
#[get("/get_list")]
async fn get_list(
  pool: web::Data<DbPool>,
  pager: web::Query<FrontProductQueryDTO>,
) -> Res<impl Responder> {
  let res = service::product::front_get_product_list(
    &pool,
    pager.into_inner()
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
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
    }
  })
}

#[post("/buy_product")]
async fn buy_product(
  user: JwtUserData,
  pool: web::Data<DbPool>,
  mut dto: web::Json<ReqBuyProductDTO>,
) -> Res<impl Responder> {
  dto.custom_id = Some(user.id);
  let res = service::order::consumer_product(
    pool,
    dto.to_buy_product_dto(),
  ).await;

  Ok(match res {
    Ok(_) => Response::ok("", "购买成功"),
    Err(e) => Response::client_error(e),
  })
}

#[get("/details")]
async fn get_details(
  pool: web::Data<DbPool>,
  pager: web::Query<QueryUuid>,
) -> Res<impl Responder> {
  let res = service::product::get_product_by_id(
    &pool,
    pager.id.clone(),
  ).await;

  Ok(match res {
    Ok(data) => Response::ok(data.to_res_dto(), "获取成功"),
    Err(e) => Response::client_error(e),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    .service(buy_product)
    .service(get_details)
    ;
}