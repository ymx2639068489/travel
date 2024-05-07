
use diesel::{prelude::*, QueryResult};
use crate::{
  models::user::*, schema::custom::dsl::*, utils::sql_response::diesel_to_res, ResponseList
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
 * 更新用户信息
 */
pub fn update_profile(conn: &mut Conn, target_user: &UpdateUserDTO) -> QueryResult<bool> {
  let target = custom.filter(id.eq(target_user.id));
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
/**
 * 新增一组用户，若冲突则忽略（通过手机号排查）
 */
pub fn insert_custom_list(coon: &mut Conn, user_list: Vec<RegisterUserDTO>) -> QueryResult<bool> {
  diesel_to_res(diesel::insert_or_ignore_into(custom)
    .values(user_list)
    .execute(coon))
}
/**
 * 获取用户列表
 */
pub fn query_user_list(
  conn: &mut Conn,
  pager: UserQueryDTO
) -> QueryResult<ResponseList<UserDTO>> {

  let get_sql = |pager: UserQueryDTO| {
    let mut sql = crate::schema::custom::table
      .into_boxed();
    if let Some(target_name) = pager.name {
      sql = sql.filter(name.like(format!("%{}%", target_name)));
    }
    if let Some(target_phone) = pager.phone {
      sql = sql.filter(phone.like(format!("%{}%", target_phone)));
    }
    if let Some(target_id_type) = pager.id_type {
      sql = sql.filter(id_type.eq(target_id_type));
    }
    if let Some(target_id_number) = pager.id_number {
      sql = sql.filter(id_number.like(format!("%{}%", target_id_number)));
    }
    if let Some(target_level) = pager.level {
      sql = sql.filter(level.eq(target_level));
    }
    sql
  };

  let list = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<UserDTO>(conn)?;
  let total = get_sql(pager.clone())
    .count()
    .get_result(conn)
    .expect("");

  Ok(ResponseList {
    data: list,
    page: pager.page,
    page_size: pager.page_size,
    total,
  })
}


/**
 * 传入一组手机号，查询出对应的所有用户id和手机号
 */
pub fn query_custom_id_and_phone_by_phone(
  conn: &mut Conn,
  phone_list: Vec<String>
) -> QueryResult<Vec<(i32, Option<String>, Option<String>)>> {
  custom
    .select((id, phone, id_number))
    .filter(phone.eq_any(phone_list).or(phone.is_null()))
    .load::<(i32, Option<String>, Option<String>)>(conn)
}