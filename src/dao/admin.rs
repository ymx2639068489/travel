// use crate::dao::admin::admin::dsl::admin;
use crate::{
  Paginate,
  models::{admin::*, company::*, role::*},
  schema::{admin, company, role}, ResponseList,
  utils::sql_response::diesel_to_res,
};
use diesel::{prelude::*, QueryResult};

type Conn = diesel::MysqlConnection;

pub fn query_admin_by_username(
  conn: &mut Conn,
  target_username: &String
) -> QueryResult<AdminJoinDTO> {
  let (q_admin, q_role, q_company) = admin::table
    .inner_join(role::table)
    .inner_join(company::table)
    .filter(admin::username.eq(target_username))
    .select((AdminDTO::as_select(), RoleDTO::as_select(), CompanyDTO::as_select()))
    .first::<(AdminDTO, RoleDTO, CompanyDTO)>(conn)?;
  Ok(q_admin.to_response_admin_dto(q_role, q_company))
}

pub fn query_admin_list(
  conn: &mut Conn,
  page: i64,
  per_page: i64,
) -> ResponseList<AdminDTO> {
  crate::schema::admin::table
    .into_boxed()
    .page(Some(page))
    .per_page(Some(per_page))
    .paginate::<AdminDTO>(conn)
    .unwrap()
}

pub fn query_admin_by_id(
  conn: &mut Conn,
  target_id: &String
) -> QueryResult<AdminJoinDTO> {
  let (q_admin, q_role, q_company) = admin::table
    .inner_join(role::table)
    .inner_join(company::table)
    .filter(admin::id.eq(target_id))
    .select((AdminDTO::as_select(), RoleDTO::as_select(), CompanyDTO::as_select()))
    .first::<(AdminDTO, RoleDTO, CompanyDTO)>(conn)?;
  Ok(q_admin.to_response_admin_dto(q_role, q_company))
}

pub fn update_admin_by_id(
  conn: &mut Conn,
  target_admin: &UpdateAdminDTO,
) -> QueryResult<bool> {
  use crate::schema::admin::dsl::*;
  let target = admin.filter(id.eq(target_admin.id.clone()));
  diesel_to_res(diesel::update(target)
   .set(target_admin)
   .execute(conn))
}

pub fn delete_admin_by_id(
  conn: &mut Conn,
  target_id: &String
) -> QueryResult<bool> {
  use crate::schema::admin::dsl::*;
  let target = admin.filter(id.eq(target_id));
  diesel_to_res(diesel::delete(target)
   .execute(conn))
}