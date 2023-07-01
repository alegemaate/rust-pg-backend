use crate::models::*;
use diesel::prelude::*;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all(filters: ProductQuery, conn: &mut PgConnection) -> Result<Vec<Product>, DbError> {
  use crate::schema::products::dsl;

  let mut query = dsl::products.into_boxed();

  if filters.name.is_some() {
    query = query.filter(dsl::name.eq(filters.name));
  }

  if filters.product_type.is_some() {
    query = query.filter(dsl::product_type.eq(filters.product_type));
  }

  let response = query.load(conn)?;

  Ok(response)
}

pub fn find_by_id(_id: i32, conn: &mut PgConnection) -> Result<Option<Product>, DbError> {
  use crate::schema::products::dsl::*;

  let response = products.find(_id).first(conn).optional()?;

  Ok(response)
}

pub fn create(params: &CreateProductDto, conn: &mut PgConnection) -> Result<Product, DbError> {
  use crate::schema::products;

  let response = diesel::insert_into(products::table)
    .values(params)
    .returning(Product::as_returning())
    .get_result(conn)?;

  Ok(response)
}

pub fn delete(_id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
  use crate::schema::products::dsl::*;

  let response = diesel::delete(products.filter(id.eq(_id))).execute(conn)?;

  Ok(response)
}
