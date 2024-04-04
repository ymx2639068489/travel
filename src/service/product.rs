use actix_web::web;

use crate::{models::product::*, ResponseList, dao};

pub async fn front_get_product_list<'a>(
  pool: &web::Data<crate::DbPool>,
  pager: FrontProductQueryDTO,
) -> Result<ResponseList<ProductJoinDTO>, &'a str> {
let mut conn = pool.get().unwrap();
let res = web::block(move ||
  dao::product::front_query_product_list(&mut conn, pager)
).await;

match res {
  Err(e) => {
    eprint!("{}", e);
    Err("数据库查询错误")
  },
  Ok(res) => match res {
    Ok(res) => Ok(res),
    Err(e) => {
      eprint!("{}", e);
      Err("查询错误")
    }
  }
}
}

pub async fn get_prudoct_list<'a>(
  pool: &web::Data<crate::DbPool>,
  pager: ProductQueryDTO,
) -> Result<ResponseList<ProductJoinDTO>, &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move ||
    dao::product::query_product_list(&mut conn, pager)
  ).await;

  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库查询错误")
    },
    Ok(res) => match res {
      Ok(res) => Ok(res),
      Err(e) => {
        eprint!("{}", e);
        Err("查询错误")
      }
    }
  }
}

pub async fn update_product<'a>(
  pool: &web::Data<crate::DbPool>,
  target_product: UpdateProductDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move ||
    dao::product::update_product(&mut conn, &target_product)
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

pub async fn add_one_product<'a>(
  pool: &web::Data<crate::DbPool>,
  target_product: AddProductDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();

  let res = web::block(move || {
    let target = target_product.to_product_dto();
    dao::product::insert_product(&mut conn, &target)
  }).await;

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

pub async fn delete_product<'a>(
  pool: &web::Data<crate::DbPool>,
  id: String,
) -> Result<(), &'a str> {
  let mut conn = pool.get().expect("");

  let res = web::block(move ||
    dao::product::delete_product(& mut conn, id.clone())
  ).await;
  match res {
    Ok(res) => match res {
      Ok(f) => match f {
        true => Ok(()),
        false => Err("删除失败"),
      }
      Err(e) => {
        eprint!("{}", e);
        Err("数据库错误")
      }
    }
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    }
  }
}
