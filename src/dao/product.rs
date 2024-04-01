
use diesel::{
  QueryDsl, QueryResult,
  prelude::*,
};

type Conn = diesel::MysqlConnection;

use crate::{
  models::product::{*},
  utils::sql_response::diesel_to_res,
  Paginate, ResponseList
};

pub fn query_all_product(
  conn: &mut Conn,
  page: i64,
  per_page: i64,
) -> ResponseList<ProductDTO> {
  crate::schema::product::table
    .into_boxed()
    .page(Some(page))
    .per_page(Some(per_page))
    .paginate::<ProductDTO>(conn)
    .unwrap()
}
pub fn update_product(
  conn: &mut Conn,
  target_product: &UpdateProductDTO,

) -> QueryResult<bool> {
    use crate::schema::product::dsl::*;
    let target = 
      product.filter(id.eq(target_product.id.clone()));
    diesel_to_res(
      diesel::update(target)
      .set(target_product)
      .execute(conn)
    )

}
// pub fn update_base_product(
//   conn: &mut Conn,
//   target_base_product: &UpdateBaseProductDTO,
// ) -> QueryResult<bool> {
//   use crate::schema::base_product::dsl::*;
//   let target = base_product.filter(id.eq(target_base_product.id.clone()));
//   diesel_to_res(
//     diesel::update(target)
//    .set(target_base_product)
//    .execute(conn)
//   )
// }

// pub fn add_base_product(
//   conn: &mut Conn,
//   target_base_product: &BaseProductDTO,
// ) -> QueryResult<bool> {
//   use crate::schema::base_product::dsl::*;
//   diesel_to_res(
//     diesel::insert_into(base_product)
//       .values(target_base_product)
//       .execute(conn)
//   )
// }
// // TODO: 删除前需要查询是否已经售卖过产品或已推出过这类产品了
// // 若已售卖，则不允许删除
// pub fn delete_base_product(
//   conn: &mut Conn,
//   target_id: String,
// ) -> QueryResult<bool> {
//   use crate::schema::base_product::dsl::*;
//   let target = base_product.filter(id.eq(target_id.clone()));
//   diesel_to_res(
//     diesel::delete(target)
//       .execute(conn)
//   )
// }