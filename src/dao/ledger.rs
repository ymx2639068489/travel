use crate::{
  models::ledger::*,
  schema::ledger::dsl::*,
  ResponseList,
  utils::sql_response::diesel_to_res,
};
use diesel::{prelude::*, QueryResult};
type Conn = diesel::MysqlConnection;
/**
 * 添加一个台账记录
 */
pub fn add_one_ledger(
  conn: &mut Conn,
  target_ledger: LedgerDTO,
) -> QueryResult<bool> {
  diesel_to_res(diesel::insert_into(ledger)
    .values(target_ledger)
    .execute(conn)
  )
}
/**
 * 获取台账记录，分页
 */
pub fn query_ledger_list(
  conn: &mut Conn,
  pager: LedgerQueryPager,
) -> QueryResult<ResponseList<LedgerDTO>> {
  let get_sql = |pager: &LedgerQueryPager| {
    let mut sql = crate::schema::ledger::table
      .into_boxed();

    if let Some(target_product_name) = &pager.product_name {
      sql = sql.filter(product_name.like(format!("%{}%", target_product_name)));
    }
    if let Some(target_people_number_l) = pager.people_number_l {
      sql = sql.filter(people_number.ge(target_people_number_l));
    }
    if let Some(target_people_number_r) = pager.people_number_r {
      sql = sql.filter(people_number.le(target_people_number_r));
    }
    if let Some(target_start_time_l) = pager.start_time_l {
      sql = sql.filter(start_time.ge(target_start_time_l));
    }
    if let Some(target_start_time_r) = pager.start_time_r {
      sql = sql.filter(start_time.le(target_start_time_r));
    }
    if let Some(target_end_time_l) = pager.end_time_l {
      sql = sql.filter(end_time.ge(target_end_time_l));
    }
    if let Some(target_end_time_r) = pager.end_time_r {
      sql = sql.filter(end_time.le(target_end_time_r));
    }
    if let Some(target_product_type) = &pager.product_type {
      sql = sql.filter(product_type.eq(format!("{}", target_product_type)));
    }
    if let Some(target_duration) = pager.duration {
      sql = sql.filter(duration.eq(target_duration));
    }
    if let Some(target_executor) = &pager.executor {
      sql = sql.filter(executor.like(format!("%{}%",target_executor)));
    }
    sql
  };
  let list = get_sql(&pager)
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<LedgerDTO>(conn)?;

  let count = get_sql(&pager)
    .count()
    .get_result(conn)
    .expect("");
  Ok(ResponseList {
    page: pager.page,
    page_size: pager.page_size,
    total: count,
    data: list,
  })
}

/**
 * 更新台账配置信息
 */
pub fn update_ledger(
  conn: &mut Conn,
  target_ledger: UpdateLedgerDTO,
) -> QueryResult<bool> {
  let target = ledger.filter(id.eq(target_ledger.id.clone()));
  diesel_to_res(diesel::update(target)
    .set(target_ledger)
    .execute(conn)
  )
}

pub fn query_item_ledger(
  conn: &mut Conn,
  target_product_id: String,
) -> QueryResult<LedgerDTO> {
  crate::schema::ledger::table
    .filter(crate::schema::ledger::dsl::id.eq(target_product_id))
    .first::<LedgerDTO>(conn)
}
