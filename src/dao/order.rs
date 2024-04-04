use crate::{
  models::{
    order::*,
    product::ProductDTO,
    salesman::SalesmanDTO,
    user::UserDTO
  },
  schema::{custom, product, salesman},
  // utils::sql_response::diesel_to_res,
  ResponseList
};
use diesel::{prelude::*, QueryResult};
type Conn = diesel::MysqlConnection;
use crate::schema::custom_salesman::dsl::*;
/**
 * 查询总数和list
 */
pub fn query_order_list(
  conn: &mut Conn,
  pager: OrderQueryPager,
) -> QueryResult<ResponseList<JoinOrderDTO>> {
  let get_sql = |pager: OrderQueryPager| {
    let mut sql = crate::schema::custom_salesman::table
      .into_boxed()
      .inner_join(product::table)
      .inner_join(salesman::table)
      .inner_join(custom::table)
      .select((
        OrderDTO::as_select(),
        ProductDTO::as_select(),
        SalesmanDTO::as_select(),
        UserDTO::as_select(),
      ));
    sql
  };

  let list = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<(OrderDTO, ProductDTO, SalesmanDTO, UserDTO)>(conn)?;

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
      .map(|(o, p, s, u)| o.to_join_order_dto(
        p.clone(),
        s.clone(),
        u.clone(),
      ))
      .collect(),
  })
}


/**
 * 查询满足条件的总人数
 */
pub fn query_order_sum_people(
  conn: &mut Conn,
  pager: OrderQueryPager,
) -> QueryResult<i32> {
  let get_sql = |pager: OrderQueryPager| {
    let mut sql = crate::schema::custom_salesman::table
      .into_boxed()
      .inner_join(product::table)
      .inner_join(salesman::table)
      .inner_join(custom::table)
      .select(diesel::dsl::sum(people_number));
      // ;
    sql
  };

  // let res = get_sql(pager.clone())
  //   .first(conn);

  // match res {
  //   Err(e) => Err(e),
  //   Ok(res) => {
  //     if let Some(res) = res {
  //       Ok(res)
  //     } else {
  //       Ok(0)
  //     }
  //   }
  // }
  Ok(1)
}

// pub fn delete_admin_by_id(
//   conn: &mut Conn,
//   target_id: &String
// ) -> QueryResult<bool> {
//   // use crate::schema::admin::dsl::*;
//   // let target = admin.filter(id.eq(target_id));
//   // diesel_to_res(diesel::delete(target)
//   //  .execute(conn))
// }

// pub fn insert_one_order(
//   conn: &mut Conn,
//   // target_admin: &AdminDTO
// ) -> QueryResult<bool>{
//   // use crate::schema::admin::dsl::*;
//   // diesel_to_res(diesel::insert_into(admin)
//   //  .values(target_admin)
//   //  .execute(conn))
// }