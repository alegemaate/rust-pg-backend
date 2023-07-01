use crate::{
  models::CreateProductDto,
  ops::product::{create, delete, find_all, find_by_id},
};
use actix_web::{web, Error, HttpResponse};

use crate::{db::establish_connection, http_exception::HttpException};

pub async fn get_products() -> Result<HttpResponse, Error> {
  let conn = &mut establish_connection();

  let results = match find_all(conn) {
    Ok(res) => res,
    Err(_) => {
      return Err(
        HttpException::InternalServerError {
          message: String::from("Db error"),
        }
        .into(),
      )
    }
  };

  Ok(HttpResponse::Ok().json(results))
}

pub async fn add_product(_new_product: web::Json<CreateProductDto>) -> Result<HttpResponse, Error> {
  let conn = &mut establish_connection();

  let result = match create(&_new_product, conn) {
    Ok(res) => res,
    Err(_) => {
      return Err(
        HttpException::InternalServerError {
          message: String::from("Db error"),
        }
        .into(),
      )
    }
  };

  Ok(HttpResponse::Ok().json(result))
}

pub async fn get_product_detail(_id: web::Path<String>) -> Result<HttpResponse, Error> {
  let product_id = match _id.parse::<i32>() {
    Ok(val) => val,
    Err(e) => {
      return Err(
        HttpException::BadRequestException {
          message: String::from(e.to_string()),
        }
        .into(),
      )
    }
  };

  let conn = &mut establish_connection();

  let result = match find_by_id(product_id, conn) {
    Ok(product) => product,
    Err(_) => {
      return Err(
        HttpException::InternalServerError {
          message: String::from("Db error"),
        }
        .into(),
      )
    }
  };

  return match result {
    Some(result) => Ok(HttpResponse::Ok().json(result)),
    None => Err(
      HttpException::NotFoundException {
        message: String::from("Product Not Found"),
      }
      .into(),
    ),
  };
}

pub async fn remove_product(_id: web::Path<String>) -> Result<HttpResponse, Error> {
  let product_id = match _id.parse::<i32>() {
    Ok(val) => val,
    Err(e) => {
      return Err(
        HttpException::BadRequestException {
          message: String::from(e.to_string()),
        }
        .into(),
      )
    }
  };

  let conn = &mut establish_connection();

  return match delete(product_id, conn) {
    Ok(result) => Ok(HttpResponse::Ok().json(result)),
    Err(_) => Err(
      HttpException::InternalServerError {
        message: String::from("Db error"),
      }
      .into(),
    ),
  };
}

#[cfg(test)]
mod tests {
  use crate::app_config::config_app;
  use actix_web::dev::Service;
  use actix_web::{
    http::{header, StatusCode},
    test, App,
  };

  #[actix_web::test]
  async fn test_add_product() {
    let app = test::init_service(App::new().configure(config_app)).await;

    let payload = r#"{"id":12345,"product_type":"fancy","name":"test"}"#.as_bytes();

    let req = test::TestRequest::post()
      .uri("/products")
      .insert_header((header::CONTENT_TYPE, "application/json"))
      .set_payload(payload)
      .to_request();

    let resp = app.call(req).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
  }
}
