use actix_web::{
  get, post, put, web, Responder, Result as Res
};
use crate::{
  models::{
    base_product::*,
    QueryPager
  },
  service,
  DbPool,
  JwtAdminData,
  Response,
};
use verify_role::verify_permissions;
/**
 * 获取管理员列表
 */
#[get("/get_list")]
#[verify_permissions(product, query)]
async fn get_base_product(
  pager: web::Query<QueryPager>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::base_product::get_all_base_prudoct(
    &pool,
    pager.page,
    pager.per_page,
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(list) => Response::ok_pager(list),
  })
}


/**
 * 更新管理员
 */
#[put("/update")]
#[verify_permissions(product, update)]
async fn update_base_product(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  update_base_product_dto: web::Json<UpdateBaseProductDTO>,
) -> Res<impl Responder> {
  let res = service::base_product::update_base_product(
    &pool,
    update_base_product_dto.into_inner()
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}

#[post("/insert")]
#[verify_permissions(product, insert)]
async fn add_one_base_product(
  target_base_product: web::Json<AddBaseProductDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res =service::base_product::insert_base_product(
    &pool,
    target_base_product.into_inner(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "新增成功"),
    Err(_) => Response::server_error("插入失败"),
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_base_product)
    .service(update_base_product)
    .service(add_one_base_product)
    ;
}