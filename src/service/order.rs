use std::collections::HashMap;

use actix_web::web;
// use futures::future::join_all;
use crate::{
  dao, models::{order::*, user::{RegisterUserDTO, UserDTO}}, DbPool, ResponseList
};

pub async fn get_list<'a>(
  pool: &web::Data<DbPool>,
  pager: OrderQueryPager,
) -> Result<ResponseList<JoinOrderDTO>, &'a str>{
  let mut conn = pool.get().expect("");

  let res = web::block(move ||
    dao::order::query_order_list(&mut conn, pager)
  ).await;

  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库查询错误")
    },
    Ok(res) => {
      match res {
        Ok(res) => Ok(res),
        Err(e) => {
          eprintln!("{:?}", e);
          Err("查询错误")
        }
      }
    }
  }
}

pub async fn get_total_number<'a>(
  pool: &web::Data<DbPool>,
  pager: OrderQueryPager,
) -> Result<i64, &'a str>{
  let mut conn = pool.get().expect("");

  let res = web::block(move ||
    dao::order::query_order_sum_people(&mut conn, pager)
  ).await;

  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库查询错误")
    },
    Ok(res) => {
      match res {
        Ok(res) => Ok(res),
        Err(e) => {
          eprintln!("{:?}", e);
          Err("查询错误")
        }
      }
    }
  }
}


pub async fn insert_order<'a>(
  pool: &web::Data<DbPool>,
  order_list: Vec<ReqAddOrderDTO>,
) -> Result<Vec<ReqAddOrderDTO>, &'a str> {
  // 销售员,产品必须存在
  // 客户不存在则创建 -> 使用insert_or_ignore_into
  // 1. 前置工作：将所有不存在的用户插入到数据库中
  {
    let mut custom_list = Vec::new();
  
    for order in order_list.iter() {
      custom_list.push(RegisterUserDTO {
        name: order.custom_name.clone(),
        phone: order.custom_phone.clone(),
        password: String::from("1"),
        level: Some(1),
      })
    }
  
    let mut conn = pool.get().expect("");
    let res = web::block(move || {
      dao::user::insert_custom_list(&mut conn, custom_list)
    }).await;
  
    if let Err(e) = res {
      eprintln!("{:?}", e);
      return Err("数据库错误");
    }
  }

  // 2. 查询用户、销售员、产品，并用hashmap存储下来
  let mut custom_map = HashMap::new();
  let mut product_map = HashMap::new();
  let mut salesman_map = HashMap::new();
  
  struct DataList {
    // id, phone, id_number
    custom: Vec<(i32, String, String)>,
    // id, username, phone, company_name
    salesman: Vec<(i32, String, String, String)>,
    // id, name
    product: Vec<(String, String)>,
  }

  let mut res_list = DataList {
    custom: Vec::new(),
    salesman: Vec::new(),
    product: Vec::new(),
  };
  {
    {
      let mut custom_phone_list: Vec<String> = vec![];
      let mut product_id_list: Vec<String> = vec![];
      let mut salesman_phone_list: Vec<String> = vec![];
    
      for order in order_list.iter() {
        custom_phone_list.push(order.custom_phone.clone());
        product_id_list.push(order.product_id.clone());
        salesman_phone_list.push(order.salesman_phone.clone());
      }
      // 查询客户
      let mut conn = pool.get().expect("");
      let custom_future = web::block(move || {
        dao::user::query_custom_id_and_phone_by_phone(&mut conn, custom_phone_list)
      });
      // 查询产品
      let mut conn = pool.get().expect("");
      let product_future = web::block(move || {
        dao::product::query_by_id_list(&mut conn, product_id_list)
      });
      // 查询销售员
      let mut conn = pool.get().expect("");
      let salesman_future = web::block(move || {
        dao::salesman::query_salesman_list_by_phone(&mut conn, salesman_phone_list)
      });
      // 让上面三个异步函数同时进行节约时间
      let (custom_list, product_list, salesman_list) = futures::future::join3(
        custom_future,
        product_future,
        salesman_future
      ).await;
      if let Ok(custom_list) = custom_list {
        if let Ok(custom_list) = custom_list {
          for item in custom_list.iter() {
            res_list.custom.push((
              item.0,
              item.1.clone().unwrap(),
              item.2.clone().unwrap(),
            ))
          }
        }
      }
      if let Ok(product_list) = product_list {
        if let Ok(product_list) = product_list {
          for item in product_list.iter() {
            res_list.product.push((item.1.clone().unwrap(), item.1.clone().unwrap()))
          }
        }
      }
      if let Ok(salesman_list) = salesman_list {
        if let Ok(salesman_list) = salesman_list {
          for item in salesman_list.iter() {
            res_list.salesman.push((
              item.0,
              item.1.clone(),
              item.2.clone(),
              item.3.clone().unwrap(),
            ))
          }
        }
      }
  
    }
    for item in res_list.custom.iter() {
      custom_map.entry(item.1.as_str()).or_insert(item);
    }
    for item in res_list.salesman.iter() {
      salesman_map.entry(item.2.as_str()).or_insert(item);
    }
    for item in res_list.product.iter() {
      product_map.entry(item.0.as_str()).or_insert(item);
    }
  }
  let mut err_list = Vec::new();
  let mut ok_list = Vec::new();
  // 3. 开始遍历订单，检查数据，并转换为对应的dto
  for order in order_list.iter() {
    let target_custom = custom_map.get(order.custom_phone.as_str()).unwrap();
    let target_product = product_map.get(order.product_id.as_str()).unwrap();
    let target_salesman = salesman_map.get(order.salesman_phone.as_str()).unwrap();

    // check
    // 用户手机号与身份证对不上 or 销售员手机号与销售员姓名对不上 or 产品id与产品名称对不上
    if target_custom.2 != order.custom_id_number
      || target_salesman.1 != order.salesman_name
      || target_product.1 != order.product_name
    {
      err_list.push(order.clone());
      continue;
    }
    // 数据对的上，可以进行插入
    ok_list.push(order.to_add_order_dto(
      target_custom.0,
      target_salesman.0,
      target_product.0.clone(),
      target_salesman.3.clone(),
    ));
  }
  // 4. 合理的插入到数据库中，不合理的数据返回
  let mut conn = pool.get().expect("");
  web::block(move || dao::order::insert_order_list(&mut conn, ok_list)).await;
  // 5. 返回不合理数据
  Ok(err_list)
}
