

use actix_web::web;
use crate::{
  dao,
  models::salesman::*,
  ResponseList
};

pub async fn get_salesman_list<'a>(
  pool: &web::Data<crate::DbPool>,
  pager: SalesmanQueryPager,
) -> Result<ResponseList<JoinSalesmanDTO>, &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move ||
    dao::salesman::query_salesman_list(&mut conn, pager)
  ).await;
  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库查询错误")
    },
    Ok(res) => match res {
      Ok(list) => Ok(list),
      Err(e) => {
        eprintln!("{:?}", e);
        Err("数据库查询错误")
      }
    }
  }
}

pub async fn update_salesman<'a>(
  pool: &web::Data<crate::DbPool>,
  target_salesman: UpdateSalesmanDTO
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || 
    dao::salesman::update_salesman(&mut conn, &target_salesman)
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
      Ok(res) => if res { Ok(()) } else { Err("更新失败") },
    }
  }
}


pub async fn delete_salesman<'a>(
  pool: &web::Data<crate::DbPool>,
  target_salesman_id: i32,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || 
    dao::salesman::delete_one_salesman(&mut conn, target_salesman_id)
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
      Ok(res) => if res { Ok(()) } else { Err("更新失败") },
    }
  }
}

pub async fn insert_salesman<'a>(
  pool: &web::Data<crate::DbPool>,
  target_salesman: AddSalesmanDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || 
    dao::salesman::insert_one_salesman(&mut conn, &target_salesman)
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
      Ok(res) => if res { Ok(()) } else { Err("更新失败") },
    }
  }
}