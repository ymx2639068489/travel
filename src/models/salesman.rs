use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::company::CompanyDTO;


#[derive(Debug, Clone, Serialize, Insertable, Selectable, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::salesman)]
#[diesel(belongs_to(crate::schema::role::dsl::company, foreign_key = company_id))]
pub struct SalesmanDTO {
  pub id: i32,
  pub company_id: Option<String>,
  pub username: String,
  pub phone: String,
}
impl SalesmanDTO {
  pub fn to_join_dto(&self, company: CompanyDTO) -> JoinSalesmanDTO {
    JoinSalesmanDTO {
      id: self.id,
      company,
      username: self.username.clone(),
      phone: self.phone.clone(),
    }
  }
}
#[derive(Debug, Serialize)]
pub struct JoinSalesmanDTO {
  pub id: i32,
  pub company: super::company::CompanyDTO,
  pub username: String,
  pub phone: String,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::salesman)]
pub struct AddSalesmanDTO {
  pub company_id: Option<String>,
  pub username: String,
  pub phone: String,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::salesman)]
pub struct UpdateSalesmanDTO {
  pub id: i32,
  pub company_id: Option<String>,
  pub username: Option<String>,
  pub phone: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SalesmanQueryPager {
  pub page: i64,
  pub page_size: i64,
  pub company_id: Option<String>,
  pub username: Option<String>,
  pub phone: Option<String>,
}