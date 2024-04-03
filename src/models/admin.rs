
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
  company::CompanyDTO,
  role::RoleDTO,
  RemoveImportInformation,
};
#[derive(Deserialize)]
pub struct AdminLogin {
  pub username: String,
  pub password: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::admin)]
#[diesel(belongs_to(crate::schema::role::dsl::role, foreign_key = role_id))]
#[diesel(belongs_to(crate::schema::company::dsl::company, foreign_key = company_id))]
pub struct AdminDTO {
  pub id: String,
  pub role_id: Option<String>,
  pub company_id: Option<String>,
  pub username: Option<String>,
  pub phone: String,
  pub password: String,
  pub avatar: Option<String>,
  pub nickname: Option<String>,
}

impl RemoveImportInformation for AdminDTO {
  fn remove_import_information(mut self) -> Self {
    self.password = "".to_string();
    self
  }
}

impl AdminDTO {
  pub fn to_response_admin_dto(&self, role_dto: RoleDTO, company_dto: CompanyDTO) -> AdminJoinDTO {
    AdminJoinDTO {
      id: self.id.clone(),
      role: role_dto,
      company: company_dto,
      phone: self.phone.clone(),
      username: self.username.clone(),
      password: self.password.clone(),
      avatar: self.avatar.clone(),
      nickname: self.nickname.clone(),
    }
  }
}
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::admin)]
pub struct UpdateAdminDTO {
  pub id: String,
  pub role_id: Option<String>,
  pub company_id: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
  pub phone: Option<String>,
  pub avatar: Option<String>,
  pub nickname: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AdminJoinDTO {
  pub id: String,
  pub role: RoleDTO,
  pub company: CompanyDTO,
  pub phone: String,
  pub username: Option<String>,
  pub password: String,
  pub avatar: Option<String>,
  pub nickname: Option<String>,
}

impl RemoveImportInformation for AdminJoinDTO {
  fn remove_import_information(mut self) -> Self {
      self.password = "".to_string();
      self
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdminQueryPager {
  pub page: i64,
  pub page_size: i64,
  pub username: Option<String>,
  pub nickname: Option<String>,
  pub phone: Option<String>,
  pub company_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddAdminDTO {
  pub role_id: String,
  pub company_id: String,
  pub username: String,
  pub phone: String,
  pub nickname: String,
}

impl AddAdminDTO {
  pub fn to_admin_dto(&self) -> AdminDTO {
    AdminDTO {
      id: uuid::Uuid::new_v4().to_string(),
      password: "123456".to_string(),
      avatar: Some("https://ts2.cn.mm.bing.net/th?id=OIP-C.tm6WK2JPevj3uX9Y7AH9oAHaHa&w=250&h=250&c=8&rs=1&qlt=90&o=6&pid=3.1&rm=2".to_string()),
      role_id: Some(self.role_id.clone()),
      company_id: Some(self.company_id.clone()),
      username: Some(self.username.clone()),
      phone: self.phone.clone(),
      nickname: Some(self.nickname.clone()),
    }
  }
}
