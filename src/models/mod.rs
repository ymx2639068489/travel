pub mod user;
pub mod role;
pub mod admin;
pub mod company;
pub mod base_product;
pub mod product;
pub mod salesman;
pub mod order;
mod response;
mod paginated;
use serde::Deserialize;

pub use response::Response;
pub use paginated::{
  ResponseList,
  QueryPager,
};
#[derive(Debug, Deserialize)]
pub struct QueryUuid {
  pub id: String,
}
#[derive(Debug, Deserialize)]
pub struct QueryId {
  pub id: i32,
}
pub trait RemoveImportInformation {
  fn remove_import_information(self) -> Self;
}