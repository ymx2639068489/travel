
use diesel::{prelude::*, QueryResult};
use crate::models::user::*;

type Conn = diesel::MysqlConnection;
use crate::schema::custom::dsl::*;
pub fn query_user_by_phone(conn: &mut Conn, phe: &String) -> QueryResult<UserDTO> {
  custom
    .filter(phone.eq(phe))
    .first::<UserDTO>(conn)
}

pub fn query_user_by_id(conn: &mut Conn, user_id: &i32) -> QueryResult<UserDTO> {
  
  custom
    .filter(id.eq(user_id))
    .first::<UserDTO>(conn)
}

pub fn query_user_no_pw_by_id(conn: &mut Conn, user_id: &i32) -> QueryResult<UserDTO> {
  query_user_by_id(conn, user_id)
}

pub fn update_profile(conn: &mut Conn, user_id: i32, user: &UpdateUserDTO) -> QueryResult<usize> {
  let target = custom.filter(id.eq(user_id));
  diesel::update(target)
    .set(user)
    .execute(conn)
}

pub fn insert_user(coon: &mut Conn, user: &RegisterUserDTO) -> QueryResult<usize> {
  diesel::insert_into(custom)
   .values(user)
   .execute(coon)
}