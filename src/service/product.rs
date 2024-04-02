// use actix_web::web;

// use crate::{models::product::*, ResponseList, dao};


// pub async fn get_all_prudoct<'a>(
//   pool: &web::Data<crate::DbPool>,
//   page: i64,
//   per_page: i64
// ) -> Result<ResponseList<ProductDTO>, &'a str> {
//   let mut conn = pool.get().unwrap();
//   let res = web::block(move ||
//     dao::product::query_product_list(&mut conn, page, per_page)
//   ).await;
//   match res {
//     Err(e) => {
//       eprint!("{}", e);
//       Err("数据库查询错误")
//     },
//     Ok(res) => {
//       Ok(res)
//     }
//   }
// }

// pub async fn update_product<'a>(
//   pool: &web::Data<crate::DbPool>,
//   target_product: UpdateProductDTO,
// ) -> Result<(), &'a str> {
//   let mut conn = pool.get().unwrap();
//   let res = web::block(move ||
//     dao::product::update_product(&mut conn, &target_product)
//   ).await;

//   match res {
//     Err(e) => {
//       eprint!("{}", e);
//       Err("数据库错误")
//     },
//     Ok(res) => match res {
//       Err(e) => {
//         eprint!("{}", e);
//         Err("数据库错误")
//       },
//       Ok(res) => match res {
//         true => Ok(()),
//         false => Err("更新失败"),
//       }
//     }
//   }
// }



