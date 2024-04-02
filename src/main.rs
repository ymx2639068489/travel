
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{r2d2, MysqlConnection};
mod controler;
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
// fn init_env() -> (config::Config, DbPool) {
fn init_env() -> DbPool {
  dotenv::dotenv().ok();
  std::env::set_var("RUST_LOG", "info");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  let env: config::Config = config::Config::init();

  let pool: DbPool = r2d2::Pool::builder()
    .build(r2d2::ConnectionManager::<MysqlConnection>::new(&env.database_url))
    .expect("database url error");
  // let pool = diesel::mysql::MysqlConnection::establish(&env.database_url)
  //   .unwrap_or_else(|_| panic!("error connecting to {}", env.database_url));
  // (env, pool)
  pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let pool = init_env();

  println!("🚀 Server started successfully: http://localhost:8080");
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .wrap(Logger::default())
      .service(web::scope("/v1").configure(controler::front::init_routes))
      .service(web::scope("/v2").configure(controler::back::init_routes))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}