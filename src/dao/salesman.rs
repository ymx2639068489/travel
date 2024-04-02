
use diesel::{
  QueryDsl, QueryResult,
  prelude::*,
};

type Conn = diesel::MysqlConnection;

use crate::{
  models::{company::CompanyDTO, salesman::*},
  schema::salesman::dsl::*,
  utils::sql_response::diesel_to_res,
  ResponseList,
};

pub fn query_salesman_list(
  conn: &mut Conn,
  pager: SalesmanQueryPager
) -> QueryResult<ResponseList<JoinSalesmanDTO>> {
  let get_sql = |pager: SalesmanQueryPager| {
    // 联合查询
    let mut sql = crate::schema::salesman::table
      .into_boxed()
      .inner_join(crate::schema::company::table)
      .select((SalesmanDTO::as_select(), CompanyDTO::as_select()));
    // 添加条件
    if let Some(target_phone) = pager.phone {
      sql = sql.filter(phone.like(format!("%{}%", target_phone)));
    }
    if let Some(target_username) = pager.username {
      sql = sql.filter(username.like(format!("%{}%", target_username)));
    }
    if let Some(target_company_id) = pager.company_id {
      sql = sql.filter(company_id.eq(target_company_id));
    }
    sql
  };

  // 查询结果
  let list = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<(SalesmanDTO, CompanyDTO)>(conn)?;

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
      .map(|(s, c)| s.to_join_dto(c.clone()))
      .collect(),
  })
}

pub fn update_salesman(
  conn: &mut Conn,
  target_salesman: &UpdateSalesmanDTO,
) -> QueryResult<bool> {
  let target = salesman.filter(id.eq(target_salesman.id));
  diesel_to_res(
    diesel::update(target)
      .set(target_salesman)
      .execute(conn)
  )
}

pub fn insert_one_salesman(
  conn: &mut Conn,
  target_salesman: &AddSalesmanDTO,
) -> QueryResult<bool> {
  diesel_to_res(
    diesel::insert_into(salesman)
      .values(target_salesman)
      .execute(conn)
  )
}

pub fn delete_one_salesman(
  conn: &mut Conn,
  target_id: i32,
) -> QueryResult<bool> {
  let target = salesman.filter(id.eq(target_id));
  diesel_to_res(
    diesel::delete(target)
      .execute(conn)
  )
}
