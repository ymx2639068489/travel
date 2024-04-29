
#[derive(Debug, Clone)]
pub struct Config {
  pub database_url: String,
}

impl Config {
  pub fn init() -> Config {
    let database_url = std::env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);
    Config {
      database_url,
    }
  }
}
