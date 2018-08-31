table! {
    prices (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    product_prices (id) {
        id -> Int4,
        product_id -> Int4,
        price_id -> Int4,
        price -> Int4,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        stock -> Nullable<Float8>,
    }
}

joinable!(product_prices -> prices (price_id));
joinable!(product_prices -> products (product_id));

allow_tables_to_appear_in_same_query!(
    prices,
    product_prices,
    products,
);
