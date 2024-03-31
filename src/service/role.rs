use actix_web::web;
use crate::{
  dao, models::role::*, DbPool, ResponseList
};

pub async fn get_role_by_page<'a>(
  pool: &web::Data<DbPool>,
  page: i64,
  per_page: i64
) -> Result<ResponseList<RoleDTO>, &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || dao::role::get_role_by_page(&mut conn, page, per_page)
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

pub async fn add_one_role<'a>(
  pool: &web::Data<DbPool>,
  role: RoleDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || dao::role::add_one_role(&mut conn, &role)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    },
    Ok(res) => {
      match res {
        Ok(flag) => match flag {
          true => Ok(()),
          false => Err("添加失败"),
        },
        Err(e) => {
          eprint!("{}", e);
          Err("数据库错误")
        }
      }
    }
  }
}

pub async fn update_one_role<'a>(
  pool: &web::Data<DbPool>,
  role: UpdateRoleDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || dao::role::update_one_role(&mut conn, &role)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    },
    Ok(res) => {
      match res {
        Ok(flag) => match flag {
          true => Ok(()),
          false => Err("添加失败"),
        },
        Err(e) => {
          eprint!("{}", e);
          Err("数据库错误")
        }
      }
    }
  }
}