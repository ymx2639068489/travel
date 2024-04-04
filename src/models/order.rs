use bigdecimal::BigDecimal;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

use super::{
  product::ProductDTO,
  salesman::SalesmanDTO,
  user::UserDTO
};

/**
 * 与数据库对应
 */
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::custom_salesman)]
#[diesel(belongs_to(crate::schema::custom::dsl::custom, foreign_key = custom_id))]
#[diesel(belongs_to(crate::schema::salesman::dsl::salesman, foreign_key = salesman_id))]
#[diesel(belongs_to(crate::schema::product::dsl::product, foreign_key = product_id))]
pub struct OrderDTO {
  pub id: i32,
  pub custom_id: Option<i32>,
  pub salesman_id: Option<i32>,
  pub product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: BigDecimal,
  pub people_number: i32,
  pub rebate: Option<String>,
}

impl OrderDTO {
  pub fn to_join_order_dto(
    &self,
    target_product: ProductDTO,
    target_salesman: SalesmanDTO,
    target_custom: UserDTO,
  ) -> JoinOrderDTO {
    JoinOrderDTO {
      id: self.id,
      custom: target_custom,
      salesman: target_salesman,
      product: target_product,
      create_at: self.create_at,
      company: self.company.clone(),
      order_id: self.order_id.clone(),
      pay_method: self.pay_method.clone(),
      money: self.money.clone(),
      people_number: self.people_number,
      rebate: self.rebate.clone(),
    }
  }
}
/**
 * 添加销售记录
 */
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::custom_salesman)]
pub struct AddOrderDTO {
  pub custom_id: Option<i32>,
  pub salesman_id: Option<i32>,
  pub product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: BigDecimal,
  pub people_number: i32,
  pub rebate: Option<String>,
}

/**
 * 返回json结果的联合order
 */
#[derive(Clone, Debug, Serialize)]
pub struct ResJoinOrderDTO {
  pub id: i32,
  pub custom: crate::models::user::UserDTO,
  pub salesman: crate::models::salesman::SalesmanDTO,
  pub product: crate::models::product::ResProductDTO,
  pub create_at: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: String,
  pub people_number: i32,
  pub rebate: Option<String>,
}
/**
 * 联合order
 */
#[derive(Debug, Clone)]
pub struct JoinOrderDTO {
  pub id: i32,
  pub custom: crate::models::user::UserDTO,
  pub salesman: crate::models::salesman::SalesmanDTO,
  pub product: crate::models::product::ProductDTO,
  pub create_at: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: BigDecimal,
  pub people_number: i32,
  pub rebate: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderQueryPager {
  pub page: i64,
  pub page_size: i64,
  // 用户名
  pub custom_name: Option<String>,
  // 销售员
  pub salesman_name: Option<String>,
  // 产品
  pub product_name: Option<String>,
  // 公司名
  pub company_name: Option<String>,
  // 订单号
  pub order_id: Option<String>,
  // 团期
  pub duration: Option<i32>,
  // 支付方式
  pub pay_method: Option<String>,
  // 录入时间范围
  pub create_at_l: Option<chrono::NaiveDateTime>,
  pub create_at_r: Option<chrono::NaiveDateTime>,
  // 开始时间范围
  pub start_time_l: Option<chrono::NaiveDateTime>,
  pub start_time_r: Option<chrono::NaiveDateTime>,
  // 结束时间范围
  pub end_time_l: Option<chrono::NaiveDateTime>,
  pub end_time_r: Option<chrono::NaiveDateTime>,
}

impl JoinOrderDTO {
  pub fn to_res_dto(&self) -> ResJoinOrderDTO {
    ResJoinOrderDTO {
      id: self.id,
      custom: self.custom.clone(),
      salesman: self.salesman.clone(),
      product: self.product.to_res_dto(),
      create_at: self.create_at,
      company: self.company.clone(),
      order_id: self.order_id.clone(),
      pay_method: self.pay_method.clone(),
      money: self.money.to_string(),
      people_number: self.people_number,
      rebate: self.rebate.clone(),
    }
  }
}
pub struct ReqBuyProductDTO {
  // pub custom_id: Option<i32>,
  // pub salesmn_id: Option<i32>, 用户端默认系统售出
  pub product_id: String,
  // pub company: String, 用户端默认系统售出
  // pub order_id: String, 自动生成
  // pub pay_method: String, 网页端支付
  pub money: String,
  pub people_number: i32, // 购买份数
  // pub rebate: Option<String>, 系统售出没有返利
}

// impl ReqBuyProductDTO {
//   pub fn to_buy_product_dto(&self) -> BuyProductDTO {
//     BuyProductDTO {}
//   }
// }
