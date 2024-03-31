
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(table_name = crate::schema::role)]
pub struct RoleDTO {
  pub id: String,
  pub rolename: String,
  pub description: Option<String>,
  pub router: Option<String>,
  pub admin_value: i32,
  pub operator_value: i32,
  pub role_value: i32,
  pub company_value: i32,
  pub salesman_value: i32,
  pub sales_records_value: i32,
  pub product_value: i32,
  pub custom_value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoleDTO {
  pub rolename: String,
  pub description: Option<String>,
  pub router: Option<String>,
  pub admin_value: i32,
  pub operator_value: i32,
  pub role_value: i32,
  pub company_value: i32,
  pub salesman_value: i32,
  pub sales_records_value: i32,
  pub product_value: i32,
  pub custom_value: i32,
}

impl AddRoleDTO {
  pub fn to_role_dto(&self) -> RoleDTO {
    RoleDTO {
      id: uuid::Uuid::new_v4().to_string(),
      rolename: self.rolename.clone(),
      description: self.description.clone(),
      router: self.router.clone(),
      admin_value: self.admin_value,
      operator_value: self.operator_value,
      role_value: self.role_value,
      company_value: self.company_value,
      salesman_value: self.salesman_value,
      sales_records_value: self.sales_records_value,
      product_value: self.product_value,
      custom_value: self.custom_value,
    }
  }
}

/**
 * 更新时，id必须传，其他的随意
 */
#[derive(Identifiable, Deserialize, Debug, AsChangeset, Clone)]
#[diesel(table_name= crate::schema::role)]
pub struct UpdateRoleDTO {
  pub id: String,
  pub rolename: Option<String>,
  pub description: Option<String>,
  pub router: Option<String>,
  pub admin_value: Option<i32>,
  pub operator_value: Option<i32>,
  pub role_value: Option<i32>,
  pub company_value: Option<i32>,
  pub salesman_value: Option<i32>,
  pub sales_records_value: Option<i32>,
  pub product_value: Option<i32>,
  pub custom_value: Option<i32>,
}