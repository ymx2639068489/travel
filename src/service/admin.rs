
use actix_web::web;
use crate::{
  models::admin::*,
  dao,
  utils,
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