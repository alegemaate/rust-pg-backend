// @generated automatically by Diesel CLI.

diesel::table! {
    parts (id) {
        id -> Int4,
        product_id -> Int4,
        part_type -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        product_type -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(parts -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    parts,
    products,
);
