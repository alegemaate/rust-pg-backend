-- Your SQL goes here
CREATE TABLE parts (
  id SERIAL PRIMARY KEY,
  product_id integer NOT NULL REFERENCES products(id),
  part_type VARCHAR,
  name VARCHAR
);
