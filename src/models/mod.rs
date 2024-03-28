pub mod user;
pub mod role;
pub mod admin;
pub mod company;

mod response;
mod paginated;
pub use response::Response;
pub use paginated::{
  ResponseList,
  Paginate,
  QueryPager,
};