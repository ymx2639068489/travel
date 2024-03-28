pub mod user;
pub mod role;
pub mod admin;

mod response;
mod paginated;
pub use response::Response;
pub use paginated::{
  ResponseList,
  Paginate,
  QueryPager,
};