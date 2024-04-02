
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Queryable, Serialize, Insertable)]
#[diesel(table_name = crate::schema::base_product)]
pub struct BaseProductDTO {
  pub id: String,
  pub create_at: chrono::NaiveDateTime,
  pub name: Option<String>,
  pub file_list: Option<String>,
  pub notes: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddBaseProductDTO {
  pub name: Option<String>,
  pub file_list: Option<String>,
  pub notes: Option<String>,
}

impl AddBaseProductDTO {
  pub fn to_base_product_dto(&self) -> BaseProductDTO {
    BaseProductDTO {
      id: uuid::Uuid::new_v4().to_string(),
      create_at: chrono::Utc::now().naive_local(),
      name: self.name.clone(),
      file_list: self.file_list.clone(),
      notes: self.notes.clone(),
    }
  }
}

#[derive(Deserialize, Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::base_product)]
pub struct UpdateBaseProductDTO {
  pub id: String,
  pub name: Option<String>,
  pub file_list: Option<String>,
  pub notes: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct BaseProductQueryDTO {
  pub page: i64,
  pub page_size: i64,
  pub name: Option<String>,
}