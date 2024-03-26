use serde::Serialize;
use actix_web:: {
  body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder
};
#[derive(Serialize)]
pub struct ResponseWrapper<'a, T> {
  pub code: i32,
  pub message: &'a str,
  pub data: Option<T>,
}

impl <'a, T> ResponseWrapper<'a, T> {
  pub fn ok(data: T, message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: 200,
      message,
      data: Some(data)
    }
  }

  pub fn server_error(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: 503,
      message,
      data: None,
    }
  }

  pub fn client_error(message: &'a str) -> ResponseWrapper<'a, T> {
    ResponseWrapper {
      code: 400,
      message,
      data: None,
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