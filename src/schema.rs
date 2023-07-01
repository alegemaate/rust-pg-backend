// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        product_type -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
    }
}
