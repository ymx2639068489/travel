use crate::{
  models::{admin::*, company::*, role::*},
  schema::{admin, company, role},
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