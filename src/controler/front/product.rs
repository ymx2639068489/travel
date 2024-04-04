
use actix_web::{get, post, web, Responder, Result as Res};
use crate::{
  models::product::*,
  service, DbPool, JwtUserData, Response, ResponseList
};
// 3140443682
#[get("/get_list")]
async fn get_list(
  _: JwtUserData,
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

// #[post("/buy_product")]
// async fn buy_product(
//   _: JwtUserData,
//   pool: web::Data<DbPool>,
//   dto: web::Json<BuyProductDTO>,
// ) {

// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    ;
}