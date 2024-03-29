
use chrono::Utc;
use jsonwebtoken::{errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::models::admin::AdminJoinDTO;
// 尽量复杂，否则可能会被爆破出来
const JWT_SECRET: &[u8] = b"jgtbskxt";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
  iss: String,
  pub exp: usize,
  pub id: String,
  pub company_id: String,
  pub admin_value: i32,
  pub operator_value: i32,
  pub role_value: i32,
  pub company_value: i32,
  pub salesman_value: i32,
  pub sales_records_value: i32,
  pub product_value: i32,
  pub custom_value: i32,
}
impl Claims {
  pub fn new(admin: &AdminJoinDTO, exp: usize) -> Claims {
    Claims {
      iss: "test".to_owned(),
      exp,
      id: admin.id.clone(),
      company_id: admin.company.id.clone(),
      admin_value: admin.role.admin_value,
      operator_value: admin.role.operator_value,
      role_value: admin.role.role_value,
      company_value: admin.role.company_value,
      salesman_value: admin.role.salesman_value,
      sales_records_value: admin.role.sales_records_value,
      product_value: admin.role.product_value,
      custom_value: admin.role.custom_value,
    }
  }
}
// 用户端的用户id为i32类型，管理员端得用string类型
pub fn create_jwt(admin: &AdminJoinDTO) -> String {
  let expiration = Utc::now()
    .checked_add_signed(
      chrono::Duration::try_seconds(3600)
        .expect("Invalid expiration")
    )
    .expect("valid timestamp")
    .timestamp();
  let header = Header::new(Algorithm::HS512);
  let claims = Claims::new(admin, expiration as usize);

  jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
    .map(|s| format!("Bearer {}", s))
    .unwrap()
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
  let validation = Validation::new(Algorithm::HS512);
  let key = DecodingKey::from_secret(JWT_SECRET);
  let data = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;
  Ok(data)
}