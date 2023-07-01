use actix_web::{web, Error, HttpResponse};

use crate::{db::establish_connection, http_exception::HttpException, models::CreatePartDto, ops};

// _query: web::Query<Option<Part>>
pub async fn get_parts(product_id: web::Path<String>) -> Result<HttpResponse, Error> {
  let part_product_id =
    product_id
      .parse::<i32>()
      .map_err(|e| HttpException::BadRequestException {
        message: String::from(e.to_string()),
      })?;

  let conn = &mut establish_connection();

  let results =
    ops::part::find_all(part_product_id, conn).map_err(|_| HttpException::InternalServerError {
      message: String::from("Db error"),
    })?;

  Ok(HttpResponse::Ok().json(results))
}

pub async fn add_part(
  product_id: web::Path<String>,
  new_part: web::Json<CreatePartDto>,
) -> Result<HttpResponse, Error> {
  let part_product_id =
    product_id
      .parse::<i32>()
      .map_err(|e| HttpException::BadRequestException {
        message: String::from(e.to_string()),
      })?;

  let conn = &mut establish_connection();

  let result = ops::part::create(part_product_id, &new_part, conn).map_err(|err| {
    HttpException::InternalServerError {
      message: err.to_string(),
    }
  })?;

  Ok(HttpResponse::Ok().json(result))
}

pub async fn get_part_detail(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
  let (_product_id, _id) = path.into_inner();

  let part_id = _id
    .parse::<i32>()
    .map_err(|e| HttpException::BadRequestException {
      message: String::from(e.to_string()),
    })?;

  let part_product_id =
    _product_id
      .parse::<i32>()
      .map_err(|e| HttpException::BadRequestException {
        message: String::from(e.to_string()),
      })?;

  let conn = &mut establish_connection();

  let result = ops::part::find_by_id(part_product_id, part_id, conn)
    .map_err(|_| HttpException::InternalServerError {
      message: String::from("Db error"),
    })?
    .ok_or_else(|| HttpException::NotFoundException {
      message: String::from("Part Not Found"),
    })?;

  Ok(HttpResponse::Ok().json(result))
}

pub async fn remove_part(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
  let (_product_id, _id) = path.into_inner();

  let part_id = _id
    .parse::<i32>()
    .map_err(|e| HttpException::BadRequestException {
      message: String::from(e.to_string()),
    })?;

  let part_product_id =
    _product_id
      .parse::<i32>()
      .map_err(|e| HttpException::BadRequestException {
        message: String::from(e.to_string()),
      })?;

  let conn = &mut establish_connection();

  let result = ops::part::delete(part_product_id, part_id, conn).map_err(|_| {
    HttpException::InternalServerError {
      message: String::from("Db error"),
    }
  })?;

  Ok(HttpResponse::Ok().json(result))
}
