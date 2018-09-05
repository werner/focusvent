table! {
    costs (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    prices (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    product_costs (id) {
        id -> Int4,
        product_id -> Int4,
        cost_id -> Int4,
        supplier_id -> Int4,
        cost -> Int4,
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

table! {
    suppliers (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        company_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

joinable!(product_costs -> costs (cost_id));
joinable!(product_costs -> products (product_id));
joinable!(product_costs -> suppliers (supplier_id));
joinable!(product_prices -> prices (price_id));
joinable!(product_prices -> products (product_id));

allow_tables_to_appear_in_same_query!(
    costs,
    prices,
    product_costs,
    product_prices,
    products,
    suppliers,
);
