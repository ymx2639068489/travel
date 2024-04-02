
use diesel::{
  QueryDsl, QueryResult,
  prelude::*,
};

type Conn = diesel::MysqlConnection;

use crate::{
  models::product::{*},
  utils::sql_response::diesel_to_res,
  ResponseList,
  schema::product::dsl::*,
};

pub fn query_product_list(
  conn: &mut Conn,
  pager: ProductQueryDTO,
) -> QueryResult<ResponseList<ProductDTO>> {
  let get_sql = |pager: ProductQueryDTO| {
    let mut sql = crate::schema::product::table
      .into_boxed();
    if let Some(target_base_product_id) = pager.base_product_id {
      sql = sql.filter(base_product_id.eq(target_base_product_id));
    }
    if let Some(target_start_time_l) = pager.start_time_l {
      sql = sql.filter(start_time.ge(target_start_time_l))
    }
    if let Some(target_start_time_r) = pager.start_time_r {
      sql = sql.filter(start_time.le(target_start_time_r))
    }
    if let Some(target_end_time_l) = pager.end_time_l {
      sql =  sql.filter(end_time.ge(target_end_time_l));
    }
    if let Some(target_end_time_r) = pager.end_time_r {
      sql = sql.filter(end_time.le(target_end_time_r))
    }
    if let Some(target_people_number) = pager.people_number {
      sql = sql.filter(people_number.eq(target_people_number));
    }
    if let Some(target_duration) = pager.duration {
      sql = sql.filter(duration.eq(target_duration));
    }
    if let Some(target_product_type) = pager.product_type {
      sql = sql.filter(product_type.like(format!("%{}%",target_product_type)));
    }
    if let Some(target_notes) = pager.notes {
      sql = sql.filter(notes.like(format!("%{}%",target_notes)));
    }
    sql
  };

  let list = get_sql(pager.clone())
    .limit(pager.page)
    .offset((pager.page - 1) * pager.page_size)
    .load::<ProductDTO>(conn)?;
  
  let total = get_sql(pager.clone())
    .count()
    .get_result(conn)
    .expect("");
  Ok(ResponseList{
    data: list,
    page: pager.page,
    page_size: pager.page_size,
    total,
  })

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

pub fn insert_product(
  conn: &mut Conn,
  target_product: &ProductDTO,
) -> QueryResult<bool> {
  diesel_to_res(
    diesel::insert_into(product)
     .values(target_product)
     .execute(conn)
  )
}

pub fn delete_product(
  conn: &mut Conn,
  target_id: String,
) -> QueryResult<bool> {
  let target = product.filter(id.eq(target_id.clone()));
  diesel_to_res(
    diesel::delete(target)
     .execute(conn)
  )
}
