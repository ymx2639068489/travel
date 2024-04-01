
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

#[derive(Deserialize, Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::admin)]
#[diesel(belongs_to(crate::schema::role::dsl::role, foreign_key = role_id))]
#[diesel(belongs_to(crate::schema::company::dsl::company, foreign_key = company_id))]
pub struct AdminDTO {
  pub id: String,
  pub role_id: Option<String>,
  pub company_id: Option<String>,
  pub username: Option<String>,
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
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::admin)]
pub struct UpdateAdminDTO {
  pub id: String,
  pub role_id: Option<String>,
  pub company_id: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
  pub avatar: Option<String>,
  pub nickname: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AdminJoinDTO {
  pub id: String,
  pub role: RoleDTO,
  pub company: CompanyDTO,
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

impl AdminDTO {
  pub fn to_response_admin_dto(&self, role_dto: RoleDTO, company_dto: CompanyDTO) -> AdminJoinDTO {
    AdminJoinDTO {
      id: self.id.clone(),
      role: role_dto,
      company: company_dto,
      username: self.username.clone(),
      password: self.password.clone(),
      avatar: self.avatar.clone(),
      nickname: self.nickname.clone(),
    }
  }
}