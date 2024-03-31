
use diesel::{prelude::*, QueryResult};
use crate::{
  schema::custom::dsl::*,
  models::user::*,
  utils::sql_response::diesel_to_res,
};
type Conn = diesel::MysqlConnection;
/**
 * 通过手机号查询用户
 */
pub fn query_user_by_phone(conn: &mut Conn, phe: &String) -> QueryResult<UserDTO> {
  custom
    .filter(phone.eq(phe))
    .first::<UserDTO>(conn)
}
/**
 * 通过id查询用户
 */
pub fn query_user_by_id(conn: &mut Conn, user_id: &i32) -> QueryResult<UserDTO> {
  custom
    .filter(id.eq(user_id))
    .first::<UserDTO>(conn)
}
/**
 * 通过id查询用户,但过滤掉密码
 */
// pub fn query_user_no_pw_by_id(conn: &mut Conn, user_id: &i32) -> QueryResult<UserDTO> {
//   query_user_by_id(conn, user_id)
// }
/**
 * 更新用户信息
 */
pub fn update_profile(conn: &mut Conn, user_id: i32, target_user: &UpdateUserDTO) -> QueryResult<bool> {
  let target = custom.filter(id.eq(user_id));
  diesel_to_res(
    diesel::update(target)
      .set(target_user)
      .execute(conn)
  )
}
/**
 * 新增一个用户
 */
pub fn insert_user(coon: &mut Conn, user: &RegisterUserDTO) -> QueryResult<bool> {
  diesel_to_res(diesel::insert_into(custom)
   .values(user)
   .execute(coon))
}

