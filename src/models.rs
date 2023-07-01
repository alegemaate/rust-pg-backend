use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Identifiable, Queryable, PartialEq, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
  pub id: i32,
  pub product_type: Option<String>,
  pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct ProductQuery {
  pub product_type: Option<String>,
  pub name: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::products)]
pub struct CreateProductDto {
  pub product_type: String,
  pub name: String,
}

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations, PartialEq, Selectable)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = crate::schema::parts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Part {
  pub id: i32,
  pub product_id: i32,
  pub part_type: Option<String>,
  pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePartDto {
  pub part_type: String,
  pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::parts)]
pub struct CreatePart {
  pub product_id: i32,
  pub part_type: String,
  pub name: String,
}
