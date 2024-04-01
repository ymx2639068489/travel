
use diesel::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, Queryable, Serialize)]
// #[diesel(table_name = crate::schema::base_product)]
pub struct BaseProductDTO {
  pub id: String,
  pub create_at: chrono::NaiveDateTime,
  pub name: Option<String>,
  pub file_list: Option<String>,
  pub notes: Option<String>,
}

// pub struct AddBaseProductDTO {
//   pub name: Option<String>,
//   pub file_list: Option<String>,
//   pub notes: Option<String>,
// }

// impl AddBaseProductDTO {
//   pub fn to_base_product_dto(&self) -> BaseProductDTO {
//     BaseProductDTO {
//       id: uuid::Uuid::new_v4().to_string(),
//       // create_at: chrono::Utc::now().naive_local(),
//       name: self.name.clone(),
//       file_list: self.file_list.clone(),
//       notes: self.notes.clone(),
//     }
//   }
// }
