use actix_web::web;

use crate::{
  dao, models::user::*, DbPool, ResponseList
};

pub async fn query_user_by_phone<'a>(
  pool: &web::Data<DbPool>,
  phone: String,
) -> Result<UserDTO, &'a str> {
  let mut conn = pool.get()
    .expect("couldn't get db connection");
  let res = web::block(move || 
    dao::user::query_user_by_phone(&mut conn, &phone)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprint!("{}", e);
        Err("未查询到用户")
      },
      Ok(user) => Ok(user),
    }
  }

}

pub async fn user_login<'a>(
  pool: &web::Data<DbPool>,
  login_user: LoginUserDTO,
) -> Result<String, &'a str> {
  let res = query_user_by_phone(pool, login_user.phone)
    .await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err(e)
    },
    Ok(user) => {
      if user.password.eq(&login_user.password) {
        Ok(crate::utils::auth::front_auth::create_jwt(&user.id))
      } else {
        Err("密码错误")
      }
    }
  }
}

pub async fn query_user_by_id<'a>(
  pool: &web::Data<DbPool>,
  id: i32,
) -> Result<UserDTO, &'a str> {
  let mut conn = pool.get()
   .expect("couldn't get db connection");
  let res = web::block(move || {
    dao::user::query_user_by_id(&mut conn, &id)
  })
  .await;
  
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
      Ok(user) => Ok(user),
    }
  }
}

/**
 * 在导入时识别数据添加客户
 * 或在用户注册时使用
 */
pub async fn add_one_user<'a>(
  pool: &web::Data<DbPool>,
  user: RegisterUserDTO,
) -> Result<(), &'a str> {

  // 先查询用户信息，再插入
  let phe = user.phone.clone();
  let res = query_user_by_phone(
    pool,
    phe,
  ).await;
  match res {
    Ok(_) => Err("用户已注册"),
    // 没有找到
    Err(_) => {
      let mut conn = pool.get()
        .expect("couldn't get db connection");
      let res = web::block(move || dao::user::insert_user(&mut conn, &user))
        .await;
      match res {
        Err(e) => {
          eprint!("{}", e);
          Err("数据库错误")
        },
        Ok(res) => match res {
          Ok(res) => match res {
            true => Ok(()),
            false => Err("插入失败"),
          },
          Err(e) => {
            eprint!("{}", e);
            Err("数据库错误")
          },
        }
      }
    }
  }
}

/**
 * 后台更新或用户更新
 */
pub async fn update_user<'a>(
  pool: &web::Data<DbPool>,
  user: UpdateUserDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get()
    .expect("couldn't get db connection");
  let res = web::block(move ||
    dao::user::update_profile(&mut conn, &user)
  ).await;
  match res {
    Err(e) => {
      eprint!("{}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Ok(res) => match res {
        true => Ok(()),
        false => Err("更新失败"),
      },
      Err(e) => {
        eprint!("{}", e);
        Err("数据库错误")
      },
    }
  }
}

/**
 * 后台获取用户列表
 */
pub async fn get_list<'a>(
  pool: &web::Data<DbPool>,
  pager: UserQueryDTO,
) -> Result<ResponseList<UserDTO>, &'a str> {
  let mut conn = pool.get().expect("");
  let res = web::block(move ||
    dao::user::query_user_list(&mut conn, pager)
  ).await;

  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库错误")
    },
    Ok(res) => match res {
      Err(e) => {
        eprintln!("{:?}", e);
        Err("数据库错误")
      },
      Ok(res) => Ok(res),
    }
  }

}