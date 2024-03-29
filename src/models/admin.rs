
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{admin, role, company};
use crate::models::{
  company::CompanyDTO,
  role::RoleDTO,
};
#[derive(Deserialize)]
pub struct AdminLogin {
  pub username: String,
  pub password: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, Queryable, Selectable, Associations, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(table_name = admin)]
#[diesel(belongs_to(role::dsl::role, foreign_key = role_id))]
#[diesel(belongs_to(company::dsl::company, foreign_key = company_id))]
pub struct AdminDTO {
  pub id: String,
  pub role_id: Option<String>,
  pub company_id: Option<String>,
  pub username: Option<String>,
  pub password: String,
  pub avatar: Option<String>,
  pub nickname: Option<String>,
}

#[derive(Serialize)]
pub struct AdminJoinDTO {
  pub id: String,
  pub role: RoleDTO,
  pub company: CompanyDTO,
  pub username: Option<String>,
  pub password: String,
  pub avatar: Option<String>,
  pub nickname: Option<String>,
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