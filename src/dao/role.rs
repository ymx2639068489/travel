
use diesel::{prelude::*, QueryResult};

type Conn = diesel::MysqlConnection;

use crate::{
  schema::role::dsl::*,
  models::role::*,
  utils::sql_response::diesel_to_res,
};

pub fn get_role_by_page(conn: &mut Conn) -> QueryResult<Vec<RoleDTO>> {
  crate::schema::role::table
    .load::<RoleDTO>(conn)
}

pub fn add_one_role(conn: &mut Conn, target_role: &RoleDTO) -> QueryResult<bool> {
  diesel_to_res(diesel::insert_into(role)
    .values(target_role)
    .execute(conn))
}

pub fn update_one_role(conn: &mut Conn, target_role: &UpdateRoleDTO) -> QueryResult<bool> {
  let target = role.filter(id.eq(target_role.id.clone()));
  diesel_to_res(diesel::update(target)
    .set(target_role)
    .execute(conn))
}
/**
 * 若有管理员与之对应，则会返回错误
 */
pub fn delete_one_role(conn: &mut Conn, target_id: String) -> QueryResult<bool> {
  let target = role.filter(id.eq(target_id));
  diesel_to_res(diesel::delete(target)
    .execute(conn))
}