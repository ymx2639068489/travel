use crate::{
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
    .select((AdminDTO::as_select(), RoleDTO::as_select(), CompanyDTO::as_select()))
    .filter(admin::username.eq(target_username))
    .first::<(AdminDTO, RoleDTO, CompanyDTO)>(conn)?;
  Ok(q_admin.to_response_admin_dto(q_role, q_company))
}

pub fn query_admin_list(
  conn: &mut Conn,
  pager: AdminQueryPager,
) -> QueryResult<ResponseList<AdminJoinDTO>> {
  let offset = (pager.page - 1) * pager.page_size;
  use crate::schema::admin::dsl::*;

  let get_sql = |pager: AdminQueryPager| {
    let mut sql = crate::schema::admin::table
      .into_boxed()
      .inner_join(role::table)
      .inner_join(company::table)
      .select((AdminDTO::as_select(), RoleDTO::as_select(), CompanyDTO::as_select()));
  
    if let Some(target_username) = pager.username {
      sql = sql.filter(username.like(format!("%{}%", target_username)));
    }
    if let Some(target_nickname) = pager.nickname {
      sql = sql.filter(nickname.like(format!("%{}%", target_nickname)));
    }
    if let Some(target_phone) = pager.phone {
      sql = sql.filter(phone.like(format!("%{}%", target_phone)));
    }
    if let Some(target_company_id) = pager.company_id {
      sql = sql.filter(company_id.eq(target_company_id));
    }
    sql
  };

  let list = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset(offset)
    .load::<(AdminDTO, RoleDTO, CompanyDTO)>(conn)?;

  let count = get_sql(pager.clone())
    .count()
    .get_result(conn)
    .expect("");

  Ok(ResponseList {
    page: pager.page,
    page_size: pager.page_size,
    total: count,
    data: list
      .iter()
      .map(|(a, c, r)| a.to_response_admin_dto(
        c.clone(),
        r.clone(),
      ))
      .collect(),
  })
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

pub fn query_admin_by_role_id(
  conn: &mut Conn,
  target_role_id: String
) -> QueryResult<Vec<AdminDTO>> {
  use crate::schema::admin::dsl::*;

  crate::schema::admin::table.filter(
    role_id.eq(target_role_id)
  ).load::<AdminDTO>(conn)
}

pub fn insert_one_admin(
  conn: &mut Conn,
  target_admin: &AdminDTO
) -> QueryResult<bool>{
  use crate::schema::admin::dsl::*;
  diesel_to_res(diesel::insert_into(admin)
   .values(target_admin)
   .execute(conn))
}