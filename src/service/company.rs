
use diesel::{prelude::*, QueryDsl, QueryResult, RunQueryDsl};
use crate::models::company::*;

type Conn = diesel::MysqlConnection;


use crate::schema::company::dsl::*;
pub fn query_all_company(conn: &mut Conn) -> QueryResult<Vec<CompanyDTO>> {
  crate::schema::company::table
   .load::<CompanyDTO>(conn)
}

pub fn add_company(conn: &mut Conn, target_company: &CompanyDTO) -> QueryResult<usize> {
  diesel::insert_into(company)
   .values(target_company)
   .execute(conn)
}

pub fn update_company(conn: &mut Conn, target_company: &CompanyDTO) -> QueryResult<usize> {
  let target = company.filter(id.eq(target_company.id.clone()));
  diesel::update(target)
   .set(target_company)
   .execute(conn)
}
