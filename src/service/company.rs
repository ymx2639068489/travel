

use actix_web::web;
use crate::{
  dao,
  models::company::*
};

pub async fn get_all_company<'a>(
  pool: &web::Data<crate::DbPool>,
) -> Result<Vec<CompanyDTO>, &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move ||
    dao::company::query_all_company(&mut conn)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库查询错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprint!("{}", e);
        Err("数据库查询错误")
      },
      Ok(list) => Ok(list),
    }
  }
}
pub async fn insert_one_company<'a>(
  pool: &web::Data<crate::DbPool>,
  company: CompanyDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || 
    dao::company::add_company(&mut conn, &company)
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
      Ok(res) => if res { Ok(()) } else { Err("插入失败") },
    }
  }
}

pub async fn update_company<'a>(
  pool: &web::Data<crate::DbPool>,
  company: CompanyDTO
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || 
    dao::company::update_company(&mut conn, &company)
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


pub async fn delete_company<'a>(
  pool: &web::Data<crate::DbPool>,
  target_company_id: String
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(move || 
    dao::company::delete_company(&mut conn, target_company_id)
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