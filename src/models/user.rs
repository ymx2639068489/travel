

use diesel::{prelude::Insertable, Queryable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};

use super::RemoveImportInformation;

// 完整的结构体
#[derive(Queryable, Debug, Serialize, Clone)]
pub struct UserDTO {
  pub id: i32,
  pub name: String,
  pub phone: Option<String>,
  pub password: String,
  pub id_type: Option<String>,
  pub id_number: Option<String>,
  pub level: Option<i32>,
}
// 返回用户信息需要的结构体（不能返回密码）
#[derive(Serialize)]
pub struct UserInfoDTO {
  pub id: i32,
  pub name: String,
  pub phone: Option<String>,
  pub id_type: Option<String>,
  pub id_number: Option<String>,
  pub level: Option<i32>,
}

impl UserDTO {
  // 两者直接转换
  pub fn get_info(&self) -> UserInfoDTO {
    UserInfoDTO {
      id: self.id,
      name: self.name.clone(),
      phone: self.phone.clone(),
      id_type: self.id_type.clone(),
      id_number: self.id_number.clone(),
      level: self.level.clone(),
    }
  }
}
impl RemoveImportInformation for UserDTO {
  fn remove_import_information(mut self) -> Self {
    self.password = "".to_string();
    if let Some(id_number) = self.id_number {
      self.id_number = Some(crate::utils::hide_id_number(id_number));
    }
    self
  }
}


#[derive(Deserialize, Debug)]
pub struct LoginUserDTO {
  pub phone: String,
  pub password: String,
}

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::custom)]
pub struct RegisterUserDTO {
  pub name: String,
  pub phone: String,
  pub password: String,
  pub level: Option<i32>,
}


#[derive(Deserialize, Debug, Identifiable, AsChangeset)]
#[diesel(table_name= crate::schema::custom)]
pub struct UpdateUserDTO {
  pub id: Option<i32>,
  pub name: Option<String>,
  pub phone: Option<String>,
  pub password: Option<String>,
  pub id_type: Option<String>,
  pub id_number: Option<String>,
}
