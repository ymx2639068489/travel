
use diesel::{
  QueryDsl, QueryResult, RunQueryDsl
};

type Conn = diesel::MysqlConnection;

use crate::{models::base_product::*, Paginate, ResponseList};

pub fn query_all_base_product(
  conn: &mut Conn,
  page: i64,
  per_page: i64,
) -> ResponseList<BaseProductDTO> {
  crate::schema::base_product::table
    .into_boxed()
    .page(Some(page))
    .per_page(Some(per_page))
    .paginate::<BaseProductDTO>(conn)
    .unwrap()
}
