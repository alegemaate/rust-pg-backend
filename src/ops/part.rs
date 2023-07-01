use crate::models::*;
use diesel::prelude::*;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all(product_id: i32, conn: &mut PgConnection) -> Result<Vec<Part>, DbError> {
  use crate::schema::parts::dsl;

  let response = dsl::parts
    .filter(dsl::product_id.eq(product_id))
    .load(conn)?;

  return Ok(response);
}

pub fn find_by_id(
  product_id: i32,
  id: i32,
  conn: &mut PgConnection,
) -> Result<Option<Part>, DbError> {
  use crate::schema::parts::dsl;

  let response = dsl::parts
    .filter(dsl::id.eq(id))
    .filter(dsl::product_id.eq(product_id))
    .first(conn)
    .optional()?;

  return Ok(response);
}

pub fn create(
  product_id: i32,
  params: &CreatePartDto,
  conn: &mut PgConnection,
) -> Result<Part, DbError> {
  use crate::schema::parts;

  let part = CreatePart {
    product_id,
    part_type: params.part_type.clone(),
    name: params.name.clone(),
  };

  let response = diesel::insert_into(parts::table)
    .values(part)
    .returning(Part::as_returning())
    .get_result(conn)?;

  return Ok(response);
}

pub fn delete(product_id: i32, id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
  use crate::schema::parts::dsl;

  let response = diesel::delete(
    dsl::parts
      .filter(dsl::id.eq(id))
      .filter(dsl::product_id.eq(product_id)),
  )
  .execute(conn)?;

  return Ok(response);
}
