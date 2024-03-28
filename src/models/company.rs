
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::company)]
pub struct CompanyDTO {
  pub id: String,
  pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddCompanyDTO {
  pub name: String,
}
impl AddCompanyDTO {
  pub fn to_company_dto(&self) -> CompanyDTO {
    CompanyDTO {
      id: uuid::Uuid::new_v4().to_string(),
      name: Some(self.name.clone()),
    }
  }
}
