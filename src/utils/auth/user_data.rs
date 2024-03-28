
use crate::utils::auth::front_auth;
use actix_web::{dev::Payload, error, Error, FromRequest, HttpRequest};
use std::future::{ready, Ready};
#[derive(Debug)]
pub struct JwtUserData {
  pub id: i32,
}

impl FromRequest for JwtUserData {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    println!("get UserData from request");
    ready({
      let auth = req.headers().get("Authorization");
      if let Some(val) = auth {
        let token = val
          .to_str()
          .unwrap()
          .split("Bearer ")
          .collect::<Vec<&str>>()
          .pop()
          .unwrap();
        let result = front_auth::validate_token(token);
        match result {
          Ok(data) => Ok(JwtUserData { id: data.claims.id }),
          Err(e) => {
            eprintln!("{}", e);
            Err(error::ErrorBadRequest("Invalid Authorization"))
          }
        }
      } else {
        Err(error::ErrorBadRequest("Invalid Authorization"))
      }
    })
  }
}