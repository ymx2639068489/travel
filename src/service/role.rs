
use diesel::{prelude::*, QueryResult};
use crate::models::role::*;

type Conn = diesel::MysqlConnection;

use crate::{Paginate, ResponseList};
use crate::schema::role::dsl::*;

pub fn get_role_by_page(conn: &mut Conn, page: i64, per_page: i64) -> ResponseList<RoleDTO> {
  crate::schema::role::table
    .into_boxed()
    .page(Some(page))
    .per_page(Some(per_page))
    .paginate::<RoleDTO>(conn)
    .unwrap()
}

pub fn add_one_role(conn: &mut Conn, target_role: &RoleDTO) -> QueryResult<usize> {
  diesel::insert_into(role)
    .values(target_role)
    .execute(conn)
}

pub fn update_one_role(conn: &mut Conn, target_role: &UpdateRoleDTO) -> QueryResult<usize> {
  let target = role.filter(id.eq(target_role.id.clone()));
  diesel::update(target)
    .set(target_role)
    .execute(conn)
}

pub fn delete_one_role(conn: &mut Conn, target_id: String) -> QueryResult<usize> {
  let target = role.filter(id.eq(target_id));
  diesel::delete(target)
    .execute(conn)
}