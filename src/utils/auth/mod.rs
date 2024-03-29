
pub mod front_auth;
pub mod back_auth;
mod user_data;
mod admin_data;
pub use user_data::JwtUserData;
pub use admin_data::JwtAdminData;