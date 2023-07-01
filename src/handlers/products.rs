use crate::{
  models::{CreateProductDto, ProductQuery},
  ops,
};
use actix_web::{web, Error, HttpResponse};

use crate::{db::establish_connection, http_exception::HttpException};

pub async fn get_products(query: web::Query<ProductQuery>) -> Result<HttpResponse, Error> {
  let conn = &mut establish_connection();

  let results = ops::product::find_all(query.into_inner(), conn).map_err(|_| {
    HttpException::InternalServerError {
      message: String::from("Db error"),
    }
  })?;

  Ok(HttpResponse::Ok().json(results))
}

pub async fn add_product(_new_product: web::Json<CreateProductDto>) -> Result<HttpResponse, Error> {
  let conn = &mut establish_connection();

  let result =
    ops::product::create(&_new_product, conn).map_err(|_| HttpException::InternalServerError {
      message: String::from("Db error"),
    })?;

  Ok(HttpResponse::Ok().json(result))
}

pub async fn get_product_detail(_id: web::Path<String>) -> Result<HttpResponse, Error> {
  let product_id = _id
    .parse::<i32>()
    .map_err(|e| HttpException::BadRequestException {
      message: String::from(e.to_string()),
    })?;

  let conn = &mut establish_connection();

  let result = ops::product::find_by_id(product_id, conn)
    .map_err(|_| HttpException::InternalServerError {
      message: String::from("Db error"),
    })?
    .ok_or_else(|| HttpException::NotFoundException {
      message: String::from("Product Not Found"),
    })?;

  Ok(HttpResponse::Ok().json(result))
}

pub async fn remove_product(_id: web::Path<String>) -> Result<HttpResponse, Error> {
  let product_id = _id
    .parse::<i32>()
    .map_err(|e| HttpException::BadRequestException {
      message: String::from(e.to_string()),
    })?;

  let conn = &mut establish_connection();

  let result =
    ops::product::delete(product_id, conn).map_err(|_| HttpException::InternalServerError {
      message: String::from("Db error"),
    })?;

  Ok(HttpResponse::Ok().json(result))
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
