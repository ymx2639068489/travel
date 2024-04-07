use crate::{
  models::{
    order::*,
    product::ProductDTO,
    salesman::SalesmanDTO,
    user::UserDTO
  },
  schema::{custom, product, salesman},
  utils::sql_response::diesel_to_res,
  ResponseList
};
use diesel::prelude::*;
type Conn = diesel::MysqlConnection;
use crate::schema::order::dsl::*;

/**
 * 查询总数和list
 */
pub fn query_order_list(
  conn: &mut Conn,
  pager: OrderQueryPager,
) -> QueryResult<ResponseList<JoinOrderDTO>> {
  let get_sql = |pager: OrderQueryPager| {
    let mut sql = crate::schema::order::table
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

    if let Some(target_custom_name) = pager.custom_name {
      sql = sql.filter(custom::dsl::name.like(format!("%{}%", target_custom_name)));
    }
    if let Some(target_salesman_name) = pager.salesman_name {
      sql = sql.filter(salesman::dsl::username.like(format!("%{}%", target_salesman_name)));
    }
    if let Some(target_company) = pager.company_name {
      sql = sql.filter(company.eq(target_company));
    }
    if let Some(target_order_id) = pager.order_id {
      sql = sql.filter(order_id.eq(target_order_id));
    }
    if let Some(target_duration) = pager.duration {
      sql = sql.filter(product::dsl::duration.eq(target_duration));
    }
    if let Some(target_pay_method) = pager.pay_method {
      sql = sql.filter(pay_method.eq(target_pay_method));
    }
    if let Some(target_create_at_l) = pager.create_at_l {
      sql = sql.filter(create_at.ge(target_create_at_l));
    }
    if let Some(target_create_at_r) = pager.create_at_r {
      sql = sql.filter(create_at.le(target_create_at_r));
    }
    if let Some(target_order_time_l) = pager.order_time_l {
      sql = sql.filter(order_time.ge(target_order_time_l));
    }
    if let Some(target_order_time_r) = pager.order_time_r {
      sql = sql.filter(order_time.le(target_order_time_r));
    }
    if let Some(target_start_time_l) = pager.start_time_l {
      sql = sql.filter(product::dsl::start_time.ge(target_start_time_l));
    }
    if let Some(target_start_time_r) = pager.start_time_r {
      sql = sql.filter(product::dsl::start_time.le(target_start_time_r));
    }
    if let Some(target_end_time_l) = pager.start_time_l {
      sql = sql.filter(product::dsl::end_time.ge(target_end_time_l));
    }
    if let Some(target_end_time_r) = pager.start_time_r {
      sql = sql.filter(product::dsl::end_time.le(target_end_time_r));
    }
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
 * 查询产品的所有销售记录，进行汇总
 */
pub fn query_all_order_by_product_id(
  conn: &mut Conn,
  target_product_id: String,
) -> QueryResult<Vec<OrderDTO>> {
  crate::schema::order::table
    .filter(product_id.eq(target_product_id))
    .load::<OrderDTO>(conn)
}

/**
 * 查询满足条件的总人数
 */
pub fn query_order_sum_people(
  conn: &mut Conn,
  pager: OrderQueryPager,
) -> QueryResult<i64> {
  
  let get_sql = move || {
    let mut sql = crate::schema::order::table
      .into_boxed()
      .inner_join(product::table)
      .inner_join(salesman::table)
      .inner_join(custom::table)
      .select(diesel::dsl::sum(people_number));
    if let Some(target_custom_name) = pager.custom_name {
      sql = sql.filter(custom::dsl::name.like(format!("%{}%", target_custom_name)));
    }
    if let Some(target_salesman_name) = pager.salesman_name {
      sql = sql.filter(salesman::dsl::username.like(format!("%{}%", target_salesman_name)));
    }
    if let Some(target_company) = pager.company_name {
      sql = sql.filter(company.eq(target_company));
    }
    if let Some(target_order_id) = pager.order_id {
      sql = sql.filter(order_id.eq(target_order_id));
    }
    if let Some(target_duration) = pager.duration {
      sql = sql.filter(product::dsl::duration.eq(target_duration));
    }
    if let Some(target_pay_method) = pager.pay_method {
      sql = sql.filter(pay_method.eq(target_pay_method));
    }
    if let Some(target_create_at_l) = pager.create_at_l {
      sql = sql.filter(create_at.ge(target_create_at_l));
    }
    if let Some(target_create_at_r) = pager.create_at_r {
      sql = sql.filter(create_at.le(target_create_at_r));
    }
    if let Some(target_order_time_l) = pager.order_time_l {
      sql = sql.filter(order_time.ge(target_order_time_l));
    }
    if let Some(target_order_time_r) = pager.order_time_r {
      sql = sql.filter(order_time.le(target_order_time_r));
    }
    if let Some(target_start_time_l) = pager.start_time_l {
      sql = sql.filter(product::dsl::start_time.ge(target_start_time_l));
    }
    if let Some(target_start_time_r) = pager.start_time_r {
      sql = sql.filter(product::dsl::start_time.le(target_start_time_r));
    }
    if let Some(target_end_time_l) = pager.start_time_l {
      sql = sql.filter(product::dsl::end_time.ge(target_end_time_l));
    }
    if let Some(target_end_time_r) = pager.start_time_r {
      sql = sql.filter(product::dsl::end_time.le(target_end_time_r));
    }
    sql
  };
  let res = get_sql().first(conn);
  match res {
    Ok(res) =>
      if let Some(number) = res {
        Ok(number)
      } else {
        Ok(0)
      }
    ,
    Err(e) => {
      println!("{:?}", e);
      Err(e.into())
    }
  }
}

/**
 * 批量导入订单
 */
pub fn insert_order_list(
  conn: &mut Conn,
  target_order: Vec<AddOrderDTO>,
) -> QueryResult<bool>{
  diesel_to_res(diesel::insert_into(order)
   .values(target_order)
   .execute(conn))
}

// /**
//  * 事务消费产品，生成订单记录
//  */
// pub fn insert_order(
//   conn: &mut Conn,
//   target_order: AddOrderDTO,
// ) -> QueryResult<bool> {
//   // diesel::
//   // conn.transaction(f)
// }