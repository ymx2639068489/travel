
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{r2d2, MysqlConnection};
mod controller;
use actix_cors::Cors;
pub mod utils;
pub mod config;
pub mod service;
pub use utils::JwtUserData;
pub use utils::JwtAdminData;
pub mod schema;
pub mod models;
pub mod dao;
pub use models::{
  ResponseList,
  Response,
  QueryPager,
};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;


// 初始化环境，并把连接池和env config返回
fn init_env() -> DbPool {
  dotenv::dotenv().ok();
  std::env::set_var("RUST_LOG", "info");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  let env: config::Config = config::Config::init();

  let pool: DbPool = r2d2::Pool::builder()
    .build(r2d2::ConnectionManager::<MysqlConnection>::new(&env.database_url))
    .expect("database url error");
  pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let pool = init_env();
  let port =  std::env::var("PORT")
      .expect("DATABASE_URL must be set");

  println!("🚀 Server started successfully: http://0.0.0.0:{}", port);
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .wrap(Logger::default())
      .wrap(
        Cors::permissive()
      )
      .service(web::scope("/v1").configure(controller::front::init_routes))
      .service(web::scope("/v2").configure(controller::back::init_routes))
  })
  .bind(("0.0.0.0", port.parse().unwrap()))?
  .run()
  .await
}