use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = crate::schema::product)]

pub struct ProductDTO {
  pub id: String,
  pub base_product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub price: Option<BigDecimal>,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: chrono::NaiveDateTime,
  pub people_number: i32,
  pub duration: i32,
  pub product_type: String,
  pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResProductDTO {
  pub id: String,
  pub base_product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub price: Option<String>,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: chrono::NaiveDateTime,
  pub people_number: i32,
  pub duration: i32,
  pub product_type: String,
  pub notes: Option<String>,
}
impl ProductDTO {
  pub fn to_res_dto(&self) -> ResProductDTO {
    let mut res : String = String::from("0");
    if let Some(price) = &self.price {
      res = price.to_engineering_notation();
    }
    ResProductDTO {
      id: self.id.clone(),
      base_product_id: self.base_product_id.clone(),
      create_at: self.create_at,
      price: Some(res),
      start_time: self.start_time,
      end_time: self.end_time,
      people_number: self.people_number,
      duration: self.duration,
      product_type: self.product_type.clone(),
      notes: self.notes.clone(),
    }
  }
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::product)]
pub struct UpdateProductDTO {
  pub id: String,
  pub price: Option<BigDecimal>,
  pub start_time: Option<chrono::NaiveDateTime>,
  pub end_time: Option<chrono::NaiveDateTime>,
  pub people_number: Option<i32>,
  pub duration: Option<i32>,
  pub product_type: Option<String>,
  pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct ReqUpdateProductDTO {
  pub id: String,
  pub price: Option<String>,
  pub start_time: Option<chrono::NaiveDateTime>,
  pub end_time: Option<chrono::NaiveDateTime>,
  pub people_number: Option<i32>,
  pub duration: Option<i32>,
  pub product_type: Option<String>,
  pub notes: Option<String>,
}
impl ReqUpdateProductDTO {
  pub fn to_update_product_dto(&self) -> UpdateProductDTO {
    let mut res: BigDecimal = "0".parse().unwrap();
    if let Some(price) = &self.price {
      res = price.parse().unwrap();
    }
    UpdateProductDTO {
      id: self.id.clone(),
      price: Some(res),
      start_time: self.start_time.clone(),
      end_time: self.end_time.clone(),
      people_number: self.people_number.clone(),
      duration: self.duration.clone(),
      product_type: self.product_type.clone(),
      notes: self.notes.clone(),
    }
  }
}