use crate::{
  models::ledger::*,
  schema::ledger::dsl::*,
  ResponseList,
  utils::sql_response::diesel_to_res,
};
use diesel::{prelude::*, QueryResult};
type Conn = diesel::MysqlConnection;

pub fn add_one_ledger(
  conn: &mut Conn,
  target_ledger: LedgerDTO,
) -> QueryResult<bool> {
  diesel_to_res(diesel::insert_into(ledger)
    .values(target_ledger)
    .execute(conn)
  )
}

pub fn get_ledger_list(
  conn: &mut Conn,
  pager: LedgerQueryPager,
) -> QueryResult<ResponseList<ResLedgerDTO>> {
  let get_sql = |pager: LedgerQueryPager| {
    let mut sql = crate::schema::ledger::table
      .into_boxed();

    if let Some(target_product_name) = pager.product_name {
      sql = sql.filter(product_name.like(format!("%{}%", target_product_name)));
    }

    sql
  };
  let list = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<LedgerDTO>(conn)?;

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
      .map(|l| l.to_res_dto())
      .collect(),
  })
}