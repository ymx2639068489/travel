
use actix_web::web;
use crate::{
  dao, models::admin::*, utils, DbPool, ResponseList
};
/**
 * 用户登录
 */
pub async fn admin_login<'a>(
  admin_login: &AdminLogin,
  pool: web::Data<crate::DbPool>,
) -> Result<String, &'a str> {
  let mut conn = pool.get().expect("");
  let adminname = admin_login.username.clone();
  let q_admin = web::block(move || 
    dao::admin::query_admin_by_username(&mut conn, &adminname)
  ).await;
  match q_admin {
    Ok(q_admin) => {
      match q_admin {
        Ok(admin) => {
          if admin.password.eq(&admin_login.password) {
            Ok(utils::auth::back_auth::create_jwt(&admin.id.clone()))
          } else {
            Err("账号密码错误")
          }
        },
        Err(_) => Err("查无此人")
      }
    },
    Err(e) => {
      eprintln!("{}", e);
      Err("数据库错误")
    }
  }
}

/**
 * 通过id获取管理员
 */
pub async fn get_admin_by_id<'a>(
  pool: &web::Data<crate::DbPool>,
  admin_id: String,
) -> Result<AdminJoinDTO, &'a str> {
  let mut conn = pool.get().expect("");
  let q_admin = web::block(move || 
    dao::admin::query_admin_by_id(&mut conn, &admin_id)
  ).await;

  match q_admin {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库查询错误")
    },
    Ok(q_admin) => {
      match q_admin {
        Ok(admin) => Ok(admin),
        Err(_) => Err("查无此管理")
      }
    }
  }
}

/**
 * 获取管理员列表
 */
pub async fn get_admin_list<'a>(
  pool: &web::Data<DbPool>,
  query_pager: AdminQueryPager
) -> Result<ResponseList<AdminDTO>, &'a str> {
  let mut conn = pool.get().expect("");
  let res = web::block(move || 
    dao::admin::query_admin_list(&mut conn, query_pager)
  ).await;
  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库查询错误")
    },
    Ok(res) => {
      Ok(res)
    }
  }
}
/**
 * 通过id更新管理员
 */
pub async fn update_admin_by_id<'a>(
  pool: &web::Data<DbPool>,
  update_admin_dto: UpdateAdminDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().expect("");
  let res = web::block(move ||
    dao::admin::update_admin_by_id(&mut conn, &update_admin_dto)
  ).await;
  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库更新错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprintln!("{:?}", e);
        Err("更新失败")
      },
      Ok(res) => match res {
        true => Ok(()),
        false => Err("更新失败"),
      }
    }
  }
}


/**
 * 通过id删除管理员
 */
pub async fn delete_admin_by_id<'a>(
  pool: &web::Data<DbPool>,
  admin_id: String,
) -> Result<(), &'a str> {
  let mut conn = pool.get().expect("");
  let res = web::block(move ||
    dao::admin::delete_admin_by_id(&mut conn, &admin_id)
  ).await;
  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprintln!("{:?}", e);
        Err("删除失败")
      },
      Ok(res) => match res {
        true => Ok(()),
        false => Err("删除失败"),
      }
    }
  }
}

/**
 * 新增一个管理员
 */
pub async fn insert_one_admin<'a>(
  pool: &web::Data<DbPool>,
  target_admin: AddAdminDTO
) -> Result<(), &'a str> {
  let mut conn = pool.get().expect("");
  let target_admin_dto = target_admin.to_admin_dto();
  let res = web::block(move ||
    dao::admin::insert_one_admin(&mut conn, &target_admin_dto)
  ).await;

  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprintln!("{:?}", e);
        Err("新增失败")
      },
      Ok(res) => match res {
        true => Ok(()),
        false => Err("新增失败"),
      }
    }
  }
}