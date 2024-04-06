
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

use crate::utils::*;

use super::base_product::BaseProductDTO;

#[derive(Clone, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::product)]
#[diesel(belongs_to(crate::schema::base_product::dsl::base_product, foreign_key = base_product_id))]
pub struct ProductDTO {
  pub id: String,
  pub base_product_id: Option<String>,
  pub create_at: chrono::NaiveDateTime,
  pub price: Option<BigDecimal>,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: chrono::NaiveDateTime,
  pub people_number: i32,
  pub surplus: i32,
  pub duration: i32,
  pub product_type: String,
  pub notes: Option<String>,
}
impl ProductDTO {
  pub fn to_product_join_dto(&self, base_product: BaseProductDTO) -> ProductJoinDTO {
    ProductJoinDTO {
      id: self.id.clone(),
      base_product,
      create_at: self.create_at,
      price: self.price.clone(),
      start_time: self.start_time,
      end_time: self.end_time,
      people_number: self.people_number,
      duration: self.duration,
      product_type: self.product_type.clone(),
      notes: self.notes.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ProductJoinDTO {
  pub id: String,
  pub base_product: BaseProductDTO,
  pub create_at: chrono::NaiveDateTime,
  pub price: Option<BigDecimal>,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: chrono::NaiveDateTime,
  pub people_number: i32,
  pub duration: i32,
  pub product_type: String,
  pub notes: Option<String>,
}
impl ProductJoinDTO {
  pub fn to_res_dto(&self) -> ResProductJoinDTO {
    let res : String = if let Some(price) = &self.price {
      price.to_string()
    } else {
      String::from("0")
    };
    ResProductJoinDTO {
      id: self.id.clone(),
      base_product: self.base_product.clone(),
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

#[derive(Debug, Clone, Serialize)]
pub struct ResProductJoinDTO {
  pub id: String,
  pub base_product: BaseProductDTO,
  pub create_at: chrono::NaiveDateTime,
  pub price: Option<String>,
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
  pub surplus: i32,
  pub duration: i32,
  pub product_type: String,
  pub notes: Option<String>,
}

impl ProductDTO {
  pub fn to_res_dto(&self) -> ResProductDTO {
    let res : String = if let Some(price) = &self.price {
      price.to_string()
    } else {
      String::from("0")
    };
    ResProductDTO {
      id: self.id.clone(),
      base_product_id: self.base_product_id.clone(),
      create_at: self.create_at,
      price: Some(res),
      start_time: self.start_time,
      end_time: self.end_time,
      people_number: self.people_number,
      surplus: self.surplus,
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

#[derive(Debug, Clone, Deserialize)]
pub struct ProductQueryDTO {
  pub page: i64,
  pub page_size: i64,
  pub base_product_id: Option<String>,
  // 开始时间范围
  pub start_time_l: Option<chrono::NaiveDateTime>,
  pub start_time_r: Option<chrono::NaiveDateTime>,
  // 结束时间范围
  pub end_time_l: Option<chrono::NaiveDateTime>,
  pub end_time_r: Option<chrono::NaiveDateTime>,
  // 人数
  pub people_number: Option<i32>,
  // 团期
  pub duration: Option<i32>,
  // 产品类型
  pub product_type: Option<String>,
  pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrontProductQueryDTO {
  pub page: i64,
  pub page_size: i64,
  // 产品类型
  pub product_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddProductDTO {
  pub base_product_id: String,
  pub price: String,
  pub start_time: String,
  pub end_time: String,
  pub people_number: i32,
  pub duration: i32,
  pub product_type: String,
  pub notes: Option<String>,
}
impl AddProductDTO {
  pub fn to_product_dto(self) -> ProductDTO {
    ProductDTO {
      id: uuid::Uuid::new_v4().to_string(),
      base_product_id: Some(self.base_product_id),
      create_at: crate::utils::now_to_naive_date_time(),
      price: Some(self.price.parse().unwrap()),
      start_time: str_to_naive_date_time(&self.start_time),
      end_time: str_to_naive_date_time(&self.end_time),
      people_number: self.people_number,
      surplus: self.people_number,
      duration: self.duration,
      product_type: self.product_type,
      notes: self.notes,
    }
  }
}
