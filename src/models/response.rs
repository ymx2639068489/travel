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
pub struct ResponseWrapper<'a, T> {
  pub code: i32,
  pub message: &'a str,
  pub data: Option<T>,
  pub pager: Option<Pager>,
  pub list: Option<Vec<T>>,
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
      pager: None,
      list: None,
    }
  }
  
  pub fn server_error<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: SERVER_ERROR_CODE,
      message,
      data: None,
      pager: None,
      list: None,
    }
  }
  
  pub fn client_error<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: CLIENT_ERROR_CODE,
      message,
      data: None,
      pager: None,
      list: None,
    }
  }
  
  pub fn ok_pager<'a, T>(pager: paginated::ResponseList<T>) -> ResponseWrapper<'a, T> {
    let pages = Pager {
      page: pager.page,
      per_page: pager.per_page,
      total: pager.total,
      last_page: pager.last_page,
    };
    ResponseWrapper {
      code: SUCCESS_CODE,
      message: "",
      list: Some(pager.data),
      pager: Some(pages),
      data: None,
    }
  }
  pub fn ok_list<'a, T>(data: Vec<T>) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: SUCCESS_CODE,
      message: "",
      list: Some(data),
      pager: None,
      data: None,
    }
  }
  pub fn server_error_list<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: SERVER_ERROR_CODE,
      message,
      list: None,
      pager: None,
      data: None,
    }
  }
  
  pub fn client_error_list<'a, T>(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: CLIENT_ERROR_CODE,
      message,
      data: None,
      pager: None,
      list: None,
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
