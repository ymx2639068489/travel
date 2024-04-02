use actix_web::{
  delete, get, post, put, web, Responder, Result as Res
};
use crate::{
  models::{product::*, QueryUuid},
  service,
  DbPool,
  JwtAdminData,
  Response, ResponseList,
};
use verify_role::verify_permissions;
/**
 * 获取产品列表
 */
#[get("/get_list")]
#[verify_permissions(product, query)]
async fn get_product(
  pager: web::Query<ProductQueryDTO>,
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
) -> Res<impl Responder> {
  let res = service::product::get_prudoct_list(
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


/**
 * 更新产品
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

/**
 * 配置添加产品
 */
#[post("/insert")]
#[verify_permissions(product, insert)]
async fn add_one_product(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  product_dto: web::Json<AddProductDTO>,
) -> Res<impl Responder> {
  let res = service::product::add_one_product(
    &pool,
    product_dto.into_inner(),
  ).await;
  Ok(match res {
    Ok(_) => Response::ok("", "更新成功"),
    Err(_) => Response::server_error("更新失败"),
  })
}

#[delete("/delete")]
#[verify_permissions(product, delete)]
async fn delete_product(
  pool: web::Data<DbPool>,
  jwt: JwtAdminData,
  target: web::Query<QueryUuid>,
) -> Res<impl Responder> {
  let res = service::product::delete_product(
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
    .service(get_product)
    .service(update_product)
    .service(add_one_product)
    .service(delete_product)
    ;
}