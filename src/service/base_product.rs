use actix_web::web;

use crate::{models::base_product::*, ResponseList, dao};


pub async fn get_all_base_prudoct<'a>(
  pool: &web::Data<crate::DbPool>,
  page: i64,
  per_page: i64
) -> Result<ResponseList<BaseProductDTO>, &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move ||
    dao::base_product::query_all_base_product(&mut conn, page, per_page)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库查询错误")
    },
    Ok(res) => {
      Ok(res)
    }
  }
}

pub async fn update_base_product<'a>(
  pool: &web::Data<crate::DbPool>,
  target_base_product: UpdateBaseProductDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move ||
    dao::base_product::update_base_product(&mut conn, &target_base_product)
  ).await;

  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprint!("{}", e);
        Err("数据库错误")
      },
      Ok(res) => match res {
        true => Ok(()),
        false => Err("更新失败"),
      }
    }
  }
}


pub async fn insert_base_product<'a>(
  pool: &web::Data<crate::DbPool>,
  target_base_product: AddBaseProductDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let bp = target_base_product.to_base_product_dto();
  let res = web::block(move ||
    dao::base_product::add_base_product(&mut conn, &bp)
  ).await;

  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprint!("{}", e);
        Err("数据库错误")
      },
      Ok(res) => match res {
        true => Ok(()),
        false => Err("添加失败"),
      }
    }
  }
}


