use bigdecimal::BigDecimal;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

use super::{
  product::ProductJoinDTO,
  salesman::SalesmanDTO,
  user::UserDTO
};

/**
 * 与数据库对应
 */
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::order)]
#[diesel(belongs_to(crate::schema::custom::dsl::custom, foreign_key = custom_id))]
#[diesel(belongs_to(crate::schema::salesman::dsl::salesman, foreign_key = salesman_id))]
#[diesel(belongs_to(crate::schema::product::dsl::product, foreign_key = product_id))]
pub struct OrderDTO {
  pub id: i32,
  pub custom_id: Option<i32>,
  pub salesman_id: Option<i32>,
  pub product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub order_time: chrono::NaiveDateTime,
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
    target_product: ProductJoinDTO,
    target_salesman: SalesmanDTO,
    target_custom: UserDTO,
  ) -> JoinOrderDTO {
    JoinOrderDTO {
      id: self.id,
      custom: target_custom,
      salesman: target_salesman,
      product: target_product,
      create_at: self.create_at,
      order_time: self.order_time,
      company: self.company.clone(),
      order_id: self.order_id.clone(),
      pay_method: self.pay_method.clone(),
      money: self.money.clone(),
      people_number: self.people_number,
      rebate: self.rebate.clone(),
    }
  }
  pub fn to_join_order_user_dto(&self, target_product: ProductJoinDTO) -> JoinOrderUserDTO {
    JoinOrderUserDTO {
      id: self.id,
      product: target_product,
      create_at: self.create_at,
      order_time: self.order_time,
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
 * 返回json结果的联合order
 */
#[derive(Clone, Debug, Serialize)]
pub struct ResJoinOrderDTO {
  pub id: i32,
  pub custom: crate::models::user::UserDTO,
  pub salesman: crate::models::salesman::SalesmanDTO,
  pub product: crate::models::product::ResProductJoinDTO,
  pub create_at: chrono::NaiveDateTime,
  pub order_time: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: String,
  pub people_number: i32,
  pub rebate: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ResJoinOrderUserDTO {
  pub id: i32,
  pub product: crate::models::product::ResProductJoinDTO,
  pub create_at: chrono::NaiveDateTime,
  pub order_time: chrono::NaiveDateTime,
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
  pub product: crate::models::product::ProductJoinDTO,
  pub create_at: chrono::NaiveDateTime,
  pub order_time: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: BigDecimal,
  pub people_number: i32,
  pub rebate: Option<String>,
}
impl JoinOrderDTO {
  pub fn to_res_dto(&self) -> ResJoinOrderDTO {
    ResJoinOrderDTO {
      id: self.id,
      custom: self.custom.clone(),
      salesman: self.salesman.clone(),
      product: self.product.to_res_dto(),
      create_at: self.create_at,
      order_time: self.order_time,
      company: self.company.clone(),
      order_id: self.order_id.clone(),
      pay_method: self.pay_method.clone(),
      money: self.money.to_string(),
      people_number: self.people_number,
      rebate: self.rebate.clone(),
    }
  }
}
#[derive(Debug, Clone)]
pub struct JoinOrderUserDTO {
  pub id: i32,
  pub product: crate::models::product::ProductJoinDTO,
  pub create_at: chrono::NaiveDateTime,
  pub order_time: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: BigDecimal,
  pub people_number: i32,
  pub rebate: Option<String>,
}

impl JoinOrderUserDTO {
  pub fn to_res_dto(&self) -> ResJoinOrderUserDTO {
    ResJoinOrderUserDTO {
      id: self.id,
      product: self.product.to_res_dto(),
      create_at: self.create_at,
      order_time: self.order_time,
      company: self.company.clone(),
      order_id: self.order_id.clone(),
      pay_method: self.pay_method.clone(),
      money: self.money.to_string(),
      people_number: self.people_number,
      rebate: self.rebate.clone(),
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrontQueryOrderDTO {
  pub product_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderQueryPager {
  pub page: i64,
  pub page_size: i64,
  // 用户名
  pub custom_name: Option<String>,
  pub custom_id: Option<i32>,
  // 销售员
  pub salesman_name: Option<String>,
  pub salesman_id: Option<i32>,
  // 产品
  pub product_name: Option<String>,
  pub product_id: Option<String>,
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
  // 订单时间范围（用户购买时间）
  pub order_time_l: Option<chrono::NaiveDateTime>,
  pub order_time_r: Option<chrono::NaiveDateTime>,
  // 开始时间范围 （产品开始和结束时间）
  pub start_time_l: Option<chrono::NaiveDateTime>,
  pub start_time_r: Option<chrono::NaiveDateTime>,
  // 结束时间范围
  pub end_time_l: Option<chrono::NaiveDateTime>,
  pub end_time_r: Option<chrono::NaiveDateTime>,
}

/**
 * 添加销售记录
 */
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::order)]
pub struct AddOrderDTO {
  pub custom_id: Option<i32>,
  pub salesman_id: Option<i32>,
  pub product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub order_time: chrono::NaiveDateTime,
  pub company: String,
  pub order_id: String,
  pub pay_method: String,
  pub money: BigDecimal,
  pub people_number: i32,
  pub rebate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReqAddOrderDTO {
  // 订单号
  pub order_id: String,
  // 支付方式
  pub pay_method: String,
  // 成交价格
  pub money: String,
  // 返点
  pub rebate: String,
  // 购买份数
  pub people_number: i32,
  // 下单时间
  pub order_time: chrono::NaiveDateTime,
  // 销售员姓名
  pub salesman_name: String,
  // 销售员电话
  pub salesman_phone: String,
  // 客户姓名
  pub custom_name: String,
  // 客户电话
  pub custom_phone: String,
  // 客户证件类型
  pub custom_id_type: String,
  // 客户证件号码
  pub custom_id_number: String,
  // 产品名称
  pub product_name: String,
  // 产品id
  pub product_id: String,
}
impl ReqAddOrderDTO {
  pub fn to_add_order_dto(
    &self, custom_id: i32,
    salesman_id: i32,
    product_id: String,
    company: String
  ) -> AddOrderDTO {
    AddOrderDTO {
      custom_id: Some(custom_id),
      salesman_id: Some(salesman_id),
      product_id: Some(product_id),
      create_at: crate::utils::now_to_naive_date_time(),
      order_time: self.order_time,
      company,
      order_id: self.order_id.clone(),
      pay_method: self.pay_method.clone(),
      money: self.money.parse::<BigDecimal>().unwrap(),
      people_number: self.people_number,
      rebate: Some(self.rebate.clone()),
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReqBuyProductDTO {
  pub custom_id: Option<i32>,
  // pub salesmn_id: Option<i32>, 用户端默认系统售出
  pub product_id: String,
  // pub company: String, 用户端默认系统售出
  // pub order_id: String, 自动生成
  // pub pay_method: String, 网页端支付
  pub money: String,
  pub people_number: i32, // 购买份数
  // pub rebate: Option<String>, 系统售出没有返利
}
impl ReqBuyProductDTO {
  pub fn to_buy_product_dto(&self) -> AddOrderDTO {
    AddOrderDTO {
      custom_id: self.custom_id.clone(),
      // 默认系统售出，id为1
      salesman_id: Some(1),
      product_id: Some(self.product_id.clone()),
      create_at: crate::utils::now_to_naive_date_time(),
      order_time: crate::utils::now_to_naive_date_time(),
      // 系统所属公司
      company: String::from("系统售出"),
      order_id: uuid::Uuid::new_v4().to_string(),
      pay_method: String::from("系统售出支付"),
      money: self.money.parse::<BigDecimal>().unwrap(),
      people_number: self.people_number,
      rebate: Some(String::from("0")),
    }
  }
}
