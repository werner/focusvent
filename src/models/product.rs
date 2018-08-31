use std::io::Read;
use std::collections::HashMap;
use diesel;
use diesel::prelude::*;
use models::db_connection::*;
use models::product_price::ProductPrice;
use models::price::Price;
use schema::product_prices::dsl::*;
use schema::prices::dsl::*;
use schema::products;
use schema::products::dsl::*;

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub stock: f64
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FullNewProduct {
    product: NewProduct,
    prices: HashMap<String, i32>
}

#[derive(Serialize, Deserialize)]
pub struct FullProduct {
    product: Product,
    prices: HashMap<String, i32>
}

impl Product {
    pub fn list(limit: i64, offset: i64) -> Result<Vec<Product>, diesel::result::Error> {
        use schema::products::dsl::*;

        let connection = establish_connection();
        products
            .limit(limit)
            .offset(offset)
            .load(&connection)
    }

    pub fn show(request_id: i32) -> Result<FullProduct, diesel::result::Error> {
        use schema::products::dsl::*;

        let connection = establish_connection();
        let mut full_product: FullProduct =
            FullProduct { 
                product: Product::blank_product(),
                prices: HashMap::new()
            };
        let vec_products = products
            .find(request_id)
            .left_join(product_prices.left_join(prices))
            .order(id.asc())
            .load::<(Product, Option<(ProductPrice, Option<Price>)>)>(&connection)?;

        for (index, db_full_product) in vec_products.into_iter().enumerate() {
            if index == 0 {
                full_product.product = db_full_product.0;
            }
            if let Some(_prices) = db_full_product.1 {
                full_product.prices.insert(_prices.1.unwrap().name, _prices.0.price);
            }
        }
        Ok(full_product)
    }

    pub fn create(full_new_product: FullNewProduct) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();

        let product: Result<Product, diesel::result::Error> = diesel::insert_into(products::table)
            .values(&full_new_product.product)
            .get_result(&connection);

        if let Ok(db_product) = &product {
            ProductPrice::batch_create(full_new_product.prices, db_product.id)?;
        }

        product
    }

    pub fn update(param_id: i32, full_product: FullProduct) -> Result<Product, diesel::result::Error> {
        use schema::products::dsl::name;
        let connection = establish_connection();

        let product = diesel::update(products.find(param_id))
            .set((name.eq(full_product.product.name),
                  description.eq(full_product.product.description)))
            .get_result::<Product>(&connection);

        if let Ok(db_product) = &product {
            ProductPrice::batch_update(full_product.prices, db_product.id)?;
        }

        product
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        let connection = establish_connection();

        diesel::delete(products.find(param_id))
            .execute(&connection)
    }

    fn blank_product() -> Product {
        Product {
            id: 0,
            name: "".to_string(),
            description: None,
            stock: 0.0
        }
    }
}

from_data!(Product);
from_data!(NewProduct);
from_data!(FullNewProduct);
from_data!(FullProduct);