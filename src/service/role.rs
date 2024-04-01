use actix_web::web;
use log::info;
use crate::{
  dao, models::role::*, DbPool
};

pub async fn get_role_by_page<'a>(pool: &web::Data<DbPool>) -> Result<Vec<RoleDTO>, &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || dao::role::get_role_by_page(&mut conn)
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
      Ok(list) => Ok(list)
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

pub async fn delete_role_by_id<'a>(
  pool: &web::Data<DbPool>,
  id: String,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();
  let res = web::block(
    move || dao::role::delete_one_role(&mut conn, id)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("server error")
    },
    Ok(res) => {
      match res {
        Ok(flag) => match flag {
          true => Ok(()),
          false => Err("删除失败, 请检查id"),
        },
        Err(e) => {
          info!("{}", e);
          Err("删除失败，请检查是否有管理员与之关联")
        }
      }
    }
  }
}