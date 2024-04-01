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
  pager: AdminQueryPager,
) -> ResponseList<AdminDTO> {
  use crate::schema::admin::dsl::*;
  let mut sql = crate::schema::admin::table
    .into_boxed();
  if let Some(target_username) = pager.username {
    println!("{}", target_username);
    sql = sql.filter(username.like("%".to_owned() + &target_username + "%"));
  }
  if let Some(target_nickname) = pager.nickname {
    println!("{}", target_nickname);
    sql = sql.filter(nickname.like("%".to_owned() + &target_nickname + "%"));
  }
  if let Some(target_phone) = pager.phone {
    sql = sql.filter(phone.like("%".to_owned() + &target_phone + "%"));
  }
  if let Some(target_company_id) = pager.company_id {
    sql = sql.filter(company_id.eq(target_company_id));
  }
  sql
    .page(Some(pager.page))
    .per_page(Some(pager.per_page))
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