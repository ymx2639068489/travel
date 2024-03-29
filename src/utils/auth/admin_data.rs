
use crate::utils::auth::back_auth;
use actix_web::{dev::Payload, error, Error, FromRequest, HttpRequest};

use std::future::{ready, Ready};


#[derive(Debug, Clone)]
pub struct JwtAdminData {
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
#[derive(Debug)]
enum RuleValue {
  Query     = 0b000001,
  QueryAll  = 0b000010,
  Insert    = 0b000100,
  Update    = 0b001000,
  Delete    = 0b010000,
  SetRule   = 0b100000,
}

fn verity(rule_value: i32, rule: &str) -> bool {
  let y = match rule {
    "query" => RuleValue::Query,
    "query_all" => RuleValue::QueryAll,
    "insert" => RuleValue::Insert,
    "update" => RuleValue::Update,
    "delete" => RuleValue::Delete,
    "set_rule" => RuleValue::SetRule,
    _ => panic!("Invalid rule"),
  };
  let y = y as i32;
  (rule_value & y) == y
}

impl JwtAdminData {
  pub fn new(claims: super::back_auth::Claims) -> JwtAdminData {
    JwtAdminData {
      id: claims.id.clone(),
      company_id: claims.company_id.clone(),
      admin_value: claims.admin_value,
      operator_value: claims.operator_value,
      role_value: claims.role_value,
      company_value: claims.company_value,
      salesman_value: claims.salesman_value,
      sales_records_value: claims.sales_records_value,
      product_value: claims.product_value,
      custom_value: claims.custom_value,
    }
  }

  pub fn validate_role(self: &Self, table: &str, rule_value: &str) -> bool {
    println!("table is '{}', rule_value is '{}'", table, rule_value);
    match table {
      "admin" => verity(self.admin_value, rule_value),
      "operator" => verity(self.operator_value, rule_value),
      "role" => verity(self.role_value, rule_value),
      "company" => verity(self.company_value, rule_value),
      "salesman" => verity(self.salesman_value, rule_value),
      "sales_records" => verity(self.sales_records_value, rule_value),
      "product" => verity(self.product_value, rule_value),
      "custom" => verity(self.custom_value, rule_value),
      _ => panic!("Invalid role: {}", table),
    }
  }
}

impl FromRequest for JwtAdminData {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
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
        let result = back_auth::validate_token(token);
        match result {
          Ok(data) => {
            // TODO: 验证用户是否有访问该接口的权限
            Ok(JwtAdminData::new(data.claims as super::back_auth::Claims))
          },
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