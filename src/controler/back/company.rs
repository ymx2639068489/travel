use actix_web::{
  get, post, web, put, Responder, Result as Res
};

use crate::{
  models::company::*, service, DbPool, Response
};


#[get("/get_all")]
async fn get_all_compnay(pool: web::Data<DbPool>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    service::company::query_all_company(&mut conn)
  }).await?;
  Ok(match res {
    Ok(list) => Response::ok_list(list),
    Err(e) => {
      eprintln!("{:?}", e);
      Response::server_error_list("error get")
    }
  })
}

#[post("/insert")]
async fn add_one(pool: web::Data<DbPool>, company: web::Json<AddCompanyDTO>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    service::company::add_company(&mut conn, &company.to_company_dto())
  }).await?;

  Ok(match res {
    Ok(size) => {
      if size == 0 {
        Response::server_error("插入失败")
      } else {
        Response::ok("", "插入成功")
      }
    },
    Err(e) => {
      eprintln!("{:?}", e);
      Response::server_error("error insert")
    }
  })
}


#[put("/update")]
async fn update_one(pool: web::Data<DbPool>, company: web::Json<CompanyDTO>) -> Res<impl Responder> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    service::company::update_company(&mut conn, &company)
  }).await?;
  Ok(match res {
    Ok(size) => {
      if size == 0 {
        Response::server_error("更新失败")
      } else {
        Response::ok("", "更新成功")
      }
    },
    Err(e) => {
      eprintln!("{:?}", e);
      Response::server_error("error insert")
    }
  })

}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(get_all_compnay);
}