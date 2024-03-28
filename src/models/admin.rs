
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AdminLogin {
  pub username: String,
  pub password: String,
}

pub struct AdminDTO {
  pub id: String,
  pub role_id: String,
  pub company_id: String,
  pub username: String,
  pub password: String,
  pub avatar: String,
  pub nickname: String,
}