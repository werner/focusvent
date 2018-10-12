table! {
    clients (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        company_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

table! {
    costs (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    currencies (id) {
        id -> Int4,
        value -> Varchar,
        symbol -> Varchar,
        decimal_point -> Varchar,
        default_currency -> Bool,
        in_use -> Bool,
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
        code -> Nullable<Varchar>,
    }
}

table! {
    sale_products (id) {
        id -> Int4,
        sale_id -> Int4,
        product_id -> Int4,
        tax -> Int4,
        amount -> Float8,
        price -> Int4,
        discount -> Int4,
        subtotal -> Int4,
        sub_total_without_discount -> Int4,
        discount_calculated -> Int4,
        taxes_calculated -> Int4,
        total -> Int4,
        observation -> Nullable<Text>,
    }
}

table! {
    sales (id) {
        id -> Int4,
        client_id -> Int4,
        sale_date -> Date,
        sub_total -> Int4,
        sub_total_without_discount -> Int4,
        discount_calculated -> Int4,
        taxes_calculated -> Int4,
        total -> Int4,
        observation -> Nullable<Text>,
        currency_id -> Int4,
    }
}

table! {
    suppliers (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        company_name -> Varchar,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

table! {
    taxes (id) {
        id -> Int4,
        name -> Varchar,
        percentage -> Int4,
    }
}

joinable!(product_costs -> costs (cost_id));
joinable!(product_costs -> products (product_id));
joinable!(product_costs -> suppliers (supplier_id));
joinable!(product_prices -> prices (price_id));
joinable!(product_prices -> products (product_id));
joinable!(sale_products -> products (product_id));
joinable!(sale_products -> sales (sale_id));
joinable!(sales -> clients (client_id));
joinable!(sales -> currencies (currency_id));

allow_tables_to_appear_in_same_query!(
    clients,
    costs,
    currencies,
    prices,
    product_costs,
    product_prices,
    products,
    sale_products,
    sales,
    suppliers,
    taxes,
);
