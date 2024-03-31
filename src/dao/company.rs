use diesel::{prelude::*, QueryDsl, QueryResult, RunQueryDsl};

type Conn = diesel::MysqlConnection;

use crate::{
  schema::company::dsl::*,
  models::company::*,
  utils::sql_response::diesel_to_res,
};

pub fn query_all_company(conn: &mut Conn) -> QueryResult<Vec<CompanyDTO>> {
  crate::schema::company::table
   .load::<CompanyDTO>(conn)
}

pub fn add_company<'a>(conn: &'a mut Conn, target_company: &'a CompanyDTO) -> QueryResult<bool> {
  diesel_to_res(diesel::insert_into(company)
   .values(target_company)
   .execute(conn))
}

pub fn update_company(conn: &mut Conn, target_company: &CompanyDTO) -> QueryResult<bool> {
  let target = company.filter(id.eq(target_company.id.clone()));
  diesel_to_res(diesel::update(target)
   .set(target_company)
   .execute(conn))
}
