
use actix_web::{get, post, put, web, Responder, Result as Res};
use crate::{
  JwtUserData,
  DbPool,
  Response,
  service,
  models::user::*,
  utils::auth::front_auth,
};
use add_pool_args::add_pool_args;
#[post("/login")]
#[add_pool_args]
async fn login(user: web::Json<LoginUserDTO>) -> Res<impl Responder> {
  let password = user.password.clone();
  // 通过手机号查询用户
  let q_user = web::block(move || {
    let mut conn = pool.get()
      .expect("couldn't get db connection");
    service::user::query_user_by_phone(&mut conn, &user.phone)
  })
  .await?;
  
  // 判断是否查询到了
  Ok(match q_user {
    Ok(q_user) => {
      // 判断密码是否正确
      if password.eq(&q_user.password) {
        // 生成token
        let token = front_auth::create_jwt(&q_user.id);
        Response::ok(token, "登录成功")
      } else {
        Response::client_error("密码错误")
      }
    }
    Err(_) => Response::client_error("该手机号码未注册")
  })
}

#[get("/info")]
#[add_pool_args]
async fn get_info(user: JwtUserData) -> Res<impl Responder> {
  let userinfo = web::block(move || {
    let mut conn = pool.get()
     .expect("couldn't get db connection");
    service::user::query_user_by_id(&mut conn, &user.id)
  })
  .await?;
  
  Ok(if let Ok(userinfo) = userinfo {
    Response::ok(userinfo.get_info(), "获取成功")
  } else {
    Response::server_error("获取失败")
  })
}

#[post("/register")]
#[add_pool_args]
async fn register(user: web::Json<RegisterUserDTO>) -> Res<impl Responder> {

  let mut conn = pool.get()
    .expect("couldn't get db connection");

  // 先查询用户信息，再插入
  let phe = user.phone.clone();
  let q_user = web::block(move || service::user::query_user_by_phone(&mut conn, &phe))
    .await?;

  if let Ok(_) = q_user {
    return Ok(Response::client_error("手机号已被注册"));
  }
  let mut conn = pool.get()
    .expect("couldn't get db connection");
  let flag = web::block(move || service::user::insert_user(&mut conn, &user))
    .await?;

  Ok(
    match flag {
      Ok(n) => Response::ok(n, "注册成功"),
      Err(_) => Response::server_error("Error inserting user")
    }
  )
}


#[put("/update_profile")]
#[add_pool_args]
async fn update_profile(
  mut user: web::Json<UpdateUserDTO>,
  ud: JwtUserData
) -> Res<impl Responder> {
  let mut conn = pool.get()
    .expect("couldn't get db connection");
  // 防止非法
  user.id = Some(ud.id);
  let flag = web::block(move || service::user::update_profile(&mut conn, ud.id, &user))
    .await?;
  Ok(
    match flag {
      Ok(n) => Response::ok(n, "更新成功"),
      Err(_) => Response::server_error("Error updating user")
    }
  )
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(login)
    .service(get_info)
    .service(register)
    .service(update_profile)
    ;
}