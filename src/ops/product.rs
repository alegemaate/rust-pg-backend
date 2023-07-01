use crate::models::*;
use diesel::prelude::*;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Product>, DbError> {
  use crate::schema::products::dsl::*;

  let response = products.load::<Product>(conn)?;

  return Ok(response);
}

pub fn find_by_id(_id: i32, conn: &mut PgConnection) -> Result<Option<Product>, DbError> {
  use crate::schema::products::dsl::*;

  let response = products.find(_id).first(conn).optional()?;

  return Ok(response);
}

pub fn create(params: &CreateProductDto, conn: &mut PgConnection) -> Result<Product, DbError> {
  use crate::schema::products;

  let response = diesel::insert_into(products::table)
    .values(params)
    .returning(Product::as_returning())
    .get_result(conn)?;

  return Ok(response);
}

pub fn delete(_id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
  use crate::schema::products::dsl::*;

  let response = diesel::delete(products.filter(id.eq(_id))).execute(conn)?;

  return Ok(response);
}
