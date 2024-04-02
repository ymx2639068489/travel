use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryPager {
  pub page: i64,
  pub page_size: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseList<T> {
  pub page: i64,
  pub page_size: i64,
  pub total: i64,
  pub data: Vec<T>,
}
