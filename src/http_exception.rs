use std::fmt::Debug;

use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};

pub trait ErrorAttributes {
  fn error_name(&self) -> String;
  fn error_code(&self) -> StatusCode;
}

#[derive(Display, Debug)]
pub enum HttpException {
  #[display(fmt = "{}", _0)]
  BadRequestException(String),
  #[display(fmt = "{}", _0)]
  UnauthorizedException(String),
  #[display(fmt = "{}", _0)]
  ForbiddenException(String),
  #[display(fmt = "{}", _0)]
  NotFoundException(String),
  #[display(fmt = "{}", _0)]
  MethodNotAllowedException(String),
  #[display(fmt = "{}", _0)]
  InternalServerError(String),
  #[display(fmt = "{}", _0)]
  NotImplemented(String),
}

#[derive(Deserialize, Serialize)]
pub struct HttpError {
  pub name: String,
  pub code: u16,
  pub message: String,
}

impl error::ResponseError for HttpException {
  fn error_response(&self) -> HttpResponse {
    let status = self.status_code();

    let body = HttpError {
      name: self.error_name(),
      code: status.as_u16(),
      message: self.to_string(),
    };

    HttpResponse::build(status).json(body)
  }

  fn status_code(&self) -> StatusCode {
    self.error_code()
  }
}

impl ErrorAttributes for HttpException {
  fn error_name(&self) -> String {
    match *self {
      HttpException::BadRequestException { .. } => String::from("Bad Request"),
      HttpException::UnauthorizedException { .. } => String::from("Unauthorized"),
      HttpException::ForbiddenException { .. } => String::from("Forbidden"),
      HttpException::NotFoundException { .. } => String::from("Not Found"),
      HttpException::MethodNotAllowedException { .. } => String::from("Method Not Allowed"),
      HttpException::InternalServerError { .. } => String::from("Internal Server Error"),
      HttpException::NotImplemented { .. } => String::from("Bad Request"),
    }
  }

  fn error_code(&self) -> StatusCode {
    match *self {
      HttpException::BadRequestException { .. } => StatusCode::BAD_REQUEST,
      HttpException::UnauthorizedException { .. } => StatusCode::UNAUTHORIZED,
      HttpException::ForbiddenException { .. } => StatusCode::FORBIDDEN,
      HttpException::NotFoundException { .. } => StatusCode::NOT_FOUND,
      HttpException::MethodNotAllowedException { .. } => StatusCode::METHOD_NOT_ALLOWED,
      HttpException::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
      HttpException::NotImplemented { .. } => StatusCode::NOT_IMPLEMENTED,
    }
  }
}
