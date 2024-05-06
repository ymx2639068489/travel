use actix_web::web;
use bigdecimal::BigDecimal;

use crate::{
  dao, models::{
    ledger::*,
    order::OrderDTO,
    product::ProductJoinDTO
  },
  DbPool,
  ResponseList
};

pub async fn get_ledger_list<'a>(
  pool: &web::Data<DbPool>,
  pager: LedgerQueryPager,
) -> Result<ResponseList<LedgerDTO>, &'a str> {
  let mut conn = pool.get().expect("");
  let res = web::block(move ||
    dao::ledger::query_ledger_list(&mut conn, pager)
  ).await;
  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库查询错误")
    },
    Ok(res) => match res {
      Ok(res) => Ok(res),
      Err(e) => {
        eprintln!("{:?}", e);
        Err("数据库查询错误")
      },
    }
  }
}

pub async fn add_one_ledger<'a>(
  pool: &web::Data<DbPool>,
  target_ledger: ReqAddLedgerDTO,
) -> Result<(), &'a str> {

  // 0. 通过id查询产品信息
  let mut conn = pool.get().unwrap();
  let product_id = target_ledger.id.clone();
  let target_product: ProductJoinDTO = web::block(move || {
    dao::product::query_product_by_id(&mut conn, product_id)
  }).await.unwrap().unwrap();

  // 旅行团尚未结束
  if target_product.end_time > chrono::Local::now().naive_local() {
    return Err("当前产品未结束、不能生成台账记录");
  }

  // 1. 通过id查询产品的销售记录
  let mut conn = pool.get().unwrap();
  let product_id = target_ledger.id.clone();
  let target_order_list: Vec<OrderDTO> = web::block(move || {
    dao::order::query_all_order_by_product_id(&mut conn, product_id)
  }).await.unwrap().unwrap();

  // 2. 进行统计数据，汇总实现ledger实例
  let mut revenue = "0".parse::<BigDecimal>().unwrap();
  let mut people_number = 0;
  for order in target_order_list {
    revenue += order.money;
    people_number += order.people_number;
  }
  let ledger_item: LedgerDTO = LedgerDTO {
    id: target_ledger.id,
    product_name: target_product.base_product.name.unwrap(),
    start_time: target_product.start_time,
    end_time: target_product.end_time,
    product_type: target_product.product_type,
    duration: target_product.duration,
    pay_status: target_ledger.pay_status,
    executor: target_ledger.executor,
    notes: target_ledger.notes,
    people_number,
    revenue,
    cost: target_ledger.cost.parse::<BigDecimal>().unwrap(),
  };
  // 3. 插入记录
  let mut conn = pool.get().unwrap();
  let res = web::block(move || {
    dao::ledger::add_one_ledger(&mut conn, ledger_item)
  }).await.unwrap();

  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库错误")
    },
    Ok(_) => Ok(())
  }
}


pub async fn update_ledger<'a>(
  pool: &web::Data<DbPool>,
  target_ledger: ReqUpdateLedgerDTO,
) -> Result<(), &'a str> {
  let mut conn = pool.get().unwrap();

  let res = web::block(move || {
    dao::ledger::update_ledger(&mut conn, target_ledger.to_update_ledger_dto())
  }).await.unwrap();

  match res {
    Err(e) => {
      eprintln!("{:?}", e);
      Err("数据库错误")
    },
    Ok(_) => Ok(())
  }

}
