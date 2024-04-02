use actix_web::{
  delete, get, post, put, web, Responder, Result as Res
};
use crate::{
  models::{
    base_product::*,
    QueryUuid
  }, service, DbPool, JwtAdminData, Response
};
use verify_role::verify_permissions;
/**
 * 获取基础产品列表
 */
#[get("/get_list")]
#[verify_permissions(product, query)]
async fn get_base_product_list(
  pager: web::Query<BaseProductQueryDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::base_product::get_base_prudoct_list(
    &pool,
    pager.into_inner(),
  ).await;
  Ok(match res {
    Err(e) => Response::client_error(e),
    Ok(list) => Response::ok_pager(list),
  })
}


/**
 * 更新基础产品
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


/**
 * 添加基础产品
 */
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

/**
 * 删除基本产品
 */
#[delete("/delete")]
#[verify_permissions(product, delete)]
async fn delete_one_base_product(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target: web::Query<QueryUuid>,
) -> Res<impl Responder> {
  let res = service::base_product::delete_base_product(
    &pool,
    target.id.clone(),
  ).await;

  Ok(match res {
    Ok(_) => Response::ok("", "删除成功"),
    Err(e) => Response::server_error(e),
  })

}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_base_product_list)
    .service(update_base_product)
    .service(add_one_base_product)
    ;
}