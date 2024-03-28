use serde::Serialize;
use actix_web:: {
  body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder
};

use super::paginated;

#[derive(Serialize)]
pub struct Pager {
  pub page: i64,
  pub per_page: i64,
  pub total: i64,
  pub last_page: i64,
}

#[derive(Serialize)]
pub struct ResponseWrapperList<'a, T> {
  pub code: i32,
  pub message: &'a str,
  pub data: Vec<T>,
  pub pager: Pager,
}

#[derive(Serialize)]
pub struct ResponseWrapper<'a, T> {
  pub code: i32,
  pub message: &'a str,
  pub data: Option<T>,
}

pub struct Response;
impl Response {
  pub fn ok<'a, T>(data: T, message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: 200,
      message,
      data: Some(data),
    }
  }
  
  pub fn server_error<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: 503,
      message,
      data: None,
    }
  }
  
  pub fn client_error<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: 400,
      message,
      data: None,
    }
  }
  
  pub fn ok_list<'a, T>(pager: paginated::ResponseList<T>) -> ResponseWrapperList<'a, T> {
    let pages = Pager {
      page: pager.page,
      per_page: pager.per_page,
      total: pager.total,
      last_page: pager.last_page,
    };
    ResponseWrapperList {
      code: 200,
      message: "",
      data: pager.data,
      pager: pages,
    }
  }
}

impl <'a, T> Responder for ResponseWrapper<'a, T> where T: Serialize {
  type Body = BoxBody;
  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let body = serde_json::to_string(&self).unwrap();
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .body(body)
  }
}

impl <'a, T> Responder for ResponseWrapperList<'a, T> where T: Serialize {
  type Body = BoxBody;
  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let body = serde_json::to_string(&self).unwrap();
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .body(body)
  }
}