use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::ledger)]
#[diesel(belongs_to(crate::schema::product::dsl::product, foreign_key = id))]
pub struct LedgerDTO {
  pub id: String,
  pub product_name: String,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: chrono::NaiveDateTime,
  pub people_number: i32,
  pub product_type: String,
  pub duration: i32,
  pub revenue: BigDecimal,
  pub cost: BigDecimal,
  pub pay_status: String,
  pub executor: String,
  pub notes: Option<String>,
}
impl LedgerDTO {
  pub fn to_res_dto(&self) -> ResLedgerDTO {
    ResLedgerDTO {
      id: self.id.clone(),
      product_name: self.product_name.clone(),
      start_time: self.start_time,
      end_time: self.end_time,
      people_number: self.people_number,
      product_type: self.product_type.clone(),
      duration: self.duration,
      revenue: self.revenue.to_string(),
      cost: self.cost.to_string(),
      pay_status: self.pay_status.clone(),
      executor: self.executor.clone(),
      notes: self.notes.clone(),
    }
  }
}
#[derive(Debug, Clone, Deserialize)]
pub struct AddLedgerDTO {
  pub id: String,
  pub pay_status: String,
  pub executor: String,
  pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LedgerQueryPager {
  pub page: i64,
  pub page_size: i64,
  pub product_name: Option<String>,
  pub people_number_l: Option<i32>,
  pub people_number_r: Option<i32>,
  // 开始时间范围 （产品开始和结束时间）
  pub start_time_l: Option<chrono::NaiveDateTime>,
  pub start_time_r: Option<chrono::NaiveDateTime>,
  // 结束时间范围
  pub end_time_l: Option<chrono::NaiveDateTime>,
  pub end_time_r: Option<chrono::NaiveDateTime>,
  pub product_type: Option<String>,
  pub duration: Option<i32>,
  pub executor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResLedgerDTO {
  pub id: String,
  pub product_name: String,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: chrono::NaiveDateTime,
  pub people_number: i32,
  pub product_type: String,
  pub duration: i32,
  pub revenue: String,
  pub cost: String,
  pub pay_status: String,
  pub executor: String,
  pub notes: Option<String>,
}
