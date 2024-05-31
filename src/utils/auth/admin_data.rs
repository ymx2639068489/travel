
use crate::{utils::auth::back_auth, DbPool};
use actix_web::{dev::Payload, error, Error, FromRequest, HttpRequest};

use std::future::{ready, Ready};
use crate::service;

#[derive(Debug, Clone)]
pub struct JwtAdminData {
  pub id: String,
}
// 权限映射表
#[derive(Debug)]
enum RuleValue {
  Query     = 0b000001,
  QueryAll  = 0b000010,
  Insert    = 0b000100,
  Update    = 0b001000,
  Delete    = 0b010000,
  SetRule   = 0b100000,
}
/**
 * 利用二进制与或非进行校验权限
 */
fn verity(rule_value: i32, rule: &str) -> bool {
  // 操作通过映射表转换为二进制
  let y = match rule {
    "query" => RuleValue::Query,
    "query_all" => RuleValue::QueryAll,
    "insert" => RuleValue::Insert,
    "update" => RuleValue::Update,
    "delete" => RuleValue::Delete,
    "set_rule" => RuleValue::SetRule,
    _ => panic!("Invalid rule"),
  } as i32;
  // 权限判断关键代码
  (rule_value & y) == y
}

impl JwtAdminData {
  // jwt 中存储管理员的id
  pub fn new(id: String) -> JwtAdminData {
    JwtAdminData {
      id,
    }
  }

  pub async fn validate_role(
    self: &Self,
    pool: &actix_web::web::Data<DbPool>,
    table: &str,
    rule_value: &str
  ) -> bool {
    // 1. 先查询用户，因为可能jwt中的数据已经非法了
    let admin_info = service::admin::get_admin_by_id(
      pool,
      self.id.clone()
    ).await;
    match admin_info {
      Ok(admin) => {
        // 判断是那个表
        match table {
          "admin" => verity(admin.role.admin_value, rule_value),
          "operator" => verity(admin.role.operator_value, rule_value),
          "role" => verity(admin.role.role_value, rule_value),
          "company" => verity(admin.role.company_value, rule_value),
          "salesman" => verity(admin.role.salesman_value, rule_value),
          "sales_records" => verity(admin.role.sales_records_value, rule_value),
          "product" => verity(admin.role.product_value, rule_value),
          "custom" => verity(admin.role.custom_value, rule_value),
          "base_product" => verity(admin.role.base_product_value, rule_value),
          "ledger" => verity(admin.role.ledger_value, rule_value),
          _ => panic!("Invalid role: {}", table),
        }
      },
      Err(_) => return false, 
    }
  }
}

impl FromRequest for JwtAdminData {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    ready({
      // 获取请求头中的token
      let auth = req.headers().get("Authorization");
      if let Some(val) = auth {
        // 解析token，转换为字符串后分割字符串，然后得到数组第二项，即为jwt
        let token = val
          .to_str()
          .unwrap()
          .split("Bearer ")
          .collect::<Vec<&str>>()
          .pop()
          .unwrap();
        // 验证jwt, 解密jwt
        let result = back_auth::validate_token(token);
        match result {
          Ok(data) => {
            // 验证成功，返回ok
            Ok(JwtAdminData::new(data.claims.id.clone()))
          },
          Err(e) => {
            // 失败，返回err
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