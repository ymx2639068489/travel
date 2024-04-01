pub mod user;
pub mod role;
pub mod admin;
pub mod company;
pub mod base_product;
pub mod product;

mod response;
mod paginated;
pub use response::Response;
pub use paginated::{
  ResponseList,
  Paginate,
  QueryPager,
};

pub trait RemoveImportInformation {
  fn remove_import_information(self) -> Self;
}