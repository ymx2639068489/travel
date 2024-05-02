
use actix_web::{get, web, Responder, Result as Res};
use crate::{
  models::order::FrontQueryOrderDTO,
  service, DbPool, JwtUserData, Response
};
#[get("/get_list")]
async fn get_list(
  user: JwtUserData,
  pool: web::Data<DbPool>,
  target_product_id: web::Query<FrontQueryOrderDTO>,
) -> Res<impl Responder> {
  let res = service::order::get_user_order_list(
    &pool,
    user.id,
    target_product_id.into_inner().product_id,
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(res) => {
      let list = res
        .iter()
        .map(|item| item.to_res_dto())
        .collect();
      Response::ok_list(list)
    }
  })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_list)
    ;
}