use actix_web::{
  get, put, web, Responder, Result as Res
};
use crate::{
  models::{
    product::*,
    QueryPager
  },
  service,
  DbPool,
  JwtAdminData,
  Response, ResponseList,
};
use verify_role::verify_permissions;
/**
 * 获取管理员列表
 */
#[get("/get_list")]
#[verify_permissions(product, query)]
async fn get_product(
  pager: web::Query<QueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::product::get_all_prudoct(
    &pool,
    pager.page,
    pager.per_page,
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(res) => {
      Response::ok_pager(ResponseList {
        page: res.page,
        per_page: res.per_page,
        total: res.total,
        last_page: res.last_page,
        data: res.data
          .iter()
          .map(|x| x.to_res_dto())
          .collect(),
      })
    },
  })
}


/**
 * 更新管理员
 */
#[put("/update")]
#[verify_permissions(product, update)]
async fn update_product(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  update_product_dto: web::Json<ReqUpdateProductDTO>,
) -> Res<impl Responder> {
  let res = service::product::update_product(
    &pool,
    update_product_dto.to_update_product_dto(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_product)
    .service(update_product)
    ;
}