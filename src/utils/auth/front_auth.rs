
// use actix_web::web::Header;

use chrono::Utc;
use jsonwebtoken::{errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
// 尽量复杂，否则可能会被爆破出来
const JWT_SECRET: &[u8] = b"as1dnlKj";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
  iss: String,
  pub exp: usize,
  pub id: i32,
}
impl Claims {
  pub fn new(id: &i32, exp: usize) -> Claims {
    Claims {
      iss: "test".to_owned(),
      id: *id,
      exp,
    }
  }
}
// 用户端的用户id为i32类型，管理员端得用string类型
pub fn create_jwt(id: &i32) -> String {
  let expiration = Utc::now()
    .checked_add_signed(
      chrono::Duration::try_seconds(60 * 60 * 24)
        .expect("Invalid expiration")
    )
    .expect("valid timestamp")
    .timestamp();
  let header = Header::new(Algorithm::HS512);
  let claims = Claims::new(id, expiration as usize);

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