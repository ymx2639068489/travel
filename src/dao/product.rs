use diesel::{self, prelude::*, QueryDsl, QueryResult};
type Conn = diesel::MysqlConnection;
use crate::{
  models::{base_product::BaseProductDTO, order::AddOrderDTO, product::*},
  schema::product::dsl::*,
  utils::{sql_response::diesel_to_res, str_to_naive_date_time},
  ResponseList,
};
pub fn front_query_product_list(
  conn: &mut Conn,
  pager: FrontProductQueryDTO,
) -> QueryResult<ResponseList<ProductJoinDTO>> {
  let get_sql = |pager: FrontProductQueryDTO| {
    let mut sql = crate::schema::product::table
      .into_boxed()
      .inner_join(crate::schema::base_product::table)
      .select((ProductDTO::as_select(), BaseProductDTO::as_select()))
      .filter(start_time.ge(str_to_naive_date_time(&pager.start_time_l)))
      ;
    if let Some(target_product_type) = pager.product_type {
      sql = sql.filter(product_type.like(format!("%{}%",target_product_type)));
    }
    
    sql
  };
  let list: Vec<ProductJoinDTO> = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<(ProductDTO, BaseProductDTO)>(conn)?
    .iter()
    .map(|(p, b)| p.to_product_join_dto(b.clone()))
    .collect();
  
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

pub fn query_product_list(
  conn: &mut Conn,
  pager: ProductQueryDTO,
) -> QueryResult<ResponseList<ProductJoinDTO>> {
  let get_sql = |pager: ProductQueryDTO| {
    let mut sql = crate::schema::product::table
      .into_boxed()
      .inner_join(crate::schema::base_product::table)
      .select((ProductDTO::as_select(), BaseProductDTO::as_select()))
      .order(crate::schema::product::start_time.desc());
    if let Some(target_base_product_id) = pager.base_product_id {
      sql = sql.filter(base_product_id.eq(target_base_product_id));
    }
    if let Some(target_start_time_l) = pager.start_time_l {
      sql = sql.filter(start_time.ge(str_to_naive_date_time(&target_start_time_l)));
    }
    if let Some(target_start_time_r) = pager.start_time_r {
      sql = sql.filter(start_time.le(str_to_naive_date_time(&target_start_time_r)));
    }
    if let Some(target_end_time_l) = pager.end_time_l {
      sql =  sql.filter(end_time.ge(str_to_naive_date_time(&target_end_time_l)));
    }
    if let Some(target_end_time_r) = pager.end_time_r {
      sql = sql.filter(end_time.le(str_to_naive_date_time(&target_end_time_r)));
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

  let list: Vec<ProductJoinDTO> = get_sql(pager.clone())
    .limit(pager.page_size)
    .offset((pager.page - 1) * pager.page_size)
    .load::<(ProductDTO, BaseProductDTO)>(conn)?
    .iter()
    .map(|(p, b)| p.to_product_join_dto(b.clone()))
    .collect();
  
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

/**
 * id_list: String[]
 * 传入一个id数组，查询对应的产品
 * 用于批量导入时的查询
 * 返回id name surplus(id，产品名，剩余量)
 */
pub fn query_by_id_list(
  conn: &mut Conn,
  id_list: Vec<String>
) -> QueryResult<Vec<(String, Option<String>, i32)>> {
  product
    .inner_join(crate::schema::base_product::table)
    .select((id, crate::schema::base_product::columns::name, surplus))
    .filter(id.eq_any(id_list))
    .load::<(String, Option<String>, i32)>(conn)
}

/**
 * 传入产品id，查询产品
 */
pub fn query_product_by_id(
  conn: &mut Conn,
  target_product_id: String,
) -> QueryResult<ProductJoinDTO> {
  let res: (ProductDTO, BaseProductDTO) = product
    .inner_join(crate::schema::base_product::table)
    .select((ProductDTO::as_select(), BaseProductDTO::as_select()))
    .filter(id.eq(target_product_id))
    .first::<(ProductDTO, BaseProductDTO)>(conn)?;
  Ok(res.0.to_product_join_dto(res.1))
}

/**
 * 消费产品，生成销售记录
 * order的添加与产品的剩余数量删除需要一起运行->事务
 */
pub fn consumer_product_to_order(
  pool: &actix_web::web::Data<crate::DbPool>,
  target_order: AddOrderDTO,
) -> QueryResult<bool> {
  let res = pool.get().expect("").transaction::<_, diesel::result::Error, _>(|_| {
    let product_id = target_order.product_id.clone().unwrap();
    let p = crate::schema::product::table.find(product_id)
      .first::<ProductDTO>(&mut pool.get().expect(""))?;
    if p.surplus > 0 {
      let product_id = target_order.product_id.clone().unwrap();
      diesel::insert_into(crate::schema::order::dsl::order)
        .values(&target_order)
        .execute(&mut pool.get().expect("")).expect("insert into order error");

      diesel::update(crate::schema::product::table.find(product_id))
        .set(crate::schema::product::surplus.eq(p.surplus - 1))
        .execute(&mut pool.get().expect("")).expect("update product error");

      println!("Purchase successful!");
    } else {
      println!("Product is out of stock!");
    }
    Ok(())
  });
  match res {
    Ok(_) => Ok(true),
    Err(_) => Ok(false),
  }
}