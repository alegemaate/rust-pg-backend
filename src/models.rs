use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
  pub id: i32,
  pub product_type: Option<String>,
  pub name: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::products)]
pub struct CreateProductDto {
  pub product_type: String,
  pub name: String,
}

#[derive(Deserialize, Serialize, Queryable)]
pub struct Part {
  pub id: Option<i64>,
  pub part_type: Option<String>,
  pub name: Option<String>,
}
