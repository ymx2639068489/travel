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
  pub data: Option<Vec<T>>,
  pub pager: Option<Pager>,
}

#[derive(Serialize)]
pub struct ResponseWrapper<'a, T> {
  pub code: i32,
  pub message: &'a str,
  pub data: Option<T>,
}

const SUCCESS_CODE: i32 = 200;
const SERVER_ERROR_CODE: i32 = 503;
const CLIENT_ERROR_CODE: i32 = 400;
pub struct Response;
impl Response {
  pub fn ok<'a, T>(data: T, message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: SUCCESS_CODE,
      message,
      data: Some(data),
    }
  }
  
  pub fn server_error<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: SERVER_ERROR_CODE,
      message,
      data: None,
    }
  }
  
  pub fn client_error<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: CLIENT_ERROR_CODE,
      message,
      data: None,
    }
  }
  
  pub fn ok_pager<'a, T>(pager: paginated::ResponseList<T>) -> ResponseWrapperList<'a, T> {
    let pages = Pager {
      page: pager.page,
      per_page: pager.per_page,
      total: pager.total,
      last_page: pager.last_page,
    };
    ResponseWrapperList {
      code: SUCCESS_CODE,
      message: "",
      data: Some(pager.data),
      pager: Some(pages),
    }
  }
  pub fn ok_list<'a, T>(data: Vec<T>) -> ResponseWrapperList<'a, T> {
    ResponseWrapperList {
      code: SUCCESS_CODE,
      message: "",
      data: Some(data),
      pager: None,
    }
  }
  pub fn server_error_list<'a, T>(message: &'a str) -> ResponseWrapperList<'a, T> {
    ResponseWrapperList {
      code: SERVER_ERROR_CODE,
      message,
      data: None,
      pager: None,
    }
  }
  
  pub fn client_error_list<'a, T>(message: &'a str) -> ResponseWrapperList<'a, T> {
    ResponseWrapperList {
      code: CLIENT_ERROR_CODE,
      message,
      data: None,
      pager: None,
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