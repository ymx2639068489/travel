
use diesel::{
  QueryDsl, QueryResult,
  prelude::*,
};

type Conn = diesel::MysqlConnection;

use crate::{
  models::base_product::*,
  utils::sql_response::diesel_to_res,
  ResponseList,
  schema::base_product::dsl::*,
};

pub fn query_all_base_product(
  conn: &mut Conn,
  pager: BaseProductQueryDTO,
) -> QueryResult<ResponseList<BaseProductDTO>> {
  let get_sql = |pager: &BaseProductQueryDTO| {
    let mut sql = crate::schema::base_product::table
      .into_boxed();
    if let Some(target_name) = &pager.name {
      sql = sql.filter(name.like(format!("%{}%", target_name)));
    }
    sql
  };

  let total = get_sql(&pager)
    .count()
    .get_result(conn)
    .expect("");

  let list = get_sql(&pager)
    .limit(pager.page)
    .offset((pager.page - 1) * pager.page_size)
    .load::<BaseProductDTO>(conn)?;

  Ok(ResponseList {
    data: list,
    page: pager.page,
    page_size: pager.page_size,
    total,
  })
}

pub fn update_base_product(
  conn: &mut Conn,
  target_base_product: &UpdateBaseProductDTO,
) -> QueryResult<bool> {
  let target = base_product.filter(id.eq(target_base_product.id.clone()));
  diesel_to_res(
    diesel::update(target)
   .set(target_base_product)
   .execute(conn)
  )
}

pub fn add_base_product(
  conn: &mut Conn,
  target_base_product: &BaseProductDTO,
) -> QueryResult<bool> {
  diesel_to_res(
    diesel::insert_into(base_product)
      .values(target_base_product)
      .execute(conn)
  )
}
// TODO: 删除前需要查询是否已经售卖过产品或已推出过这类产品了
// 若已售卖，则不允许删除
// 没必要进行查询，直接删，若有数据库自己会报错
pub fn delete_base_product(
  conn: &mut Conn,
  target_id: &String,
) -> QueryResult<bool> {
  let target = base_product.filter(id.eq(target_id.clone()));
  diesel_to_res(
    diesel::delete(target)
      .execute(conn)
  )
}