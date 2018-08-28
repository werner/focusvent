use std::io::Read;
use diesel;
use diesel::prelude::*;
use models::db_connection::*;
use models::product_price::ProductPrice;
use models::price::Price;
use models::price::NewPrice;
use schema::product_prices;
use schema::product_prices::dsl::*;
use schema::prices;
use schema::prices::dsl::*;
use schema::products;
use schema::products::dsl::*;

#[derive(Serialize, Deserialize, Queryable)]
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
    price: Option<Price>
}

impl Product {
    pub fn list(limit: i64, offset: i64) -> Result<Vec<(Product, Option<(ProductPrice, Price)>)>, diesel::result::Error> {
        use schema::products::dsl::*;

        let connection = establish_connection();
        products
            .left_join(product_prices.inner_join(prices))
            .limit(limit)
            .offset(offset)
            .load::<(Product, Option<(ProductPrice, Price)>)>(&connection)
    }

    pub fn create(full_new_product: FullNewProduct) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();

        let product: Result<Product, diesel::result::Error> = diesel::insert_into(products::table)
            .values(&full_new_product.product)
            .get_result(&connection);

        if let Ok(db_product) = &product {
            if let Some(raw_price) = full_new_product.price {
                match prices.find(raw_price.id).first(&connection) {
                    Ok(_price) => {
                        let db_price: Price = _price;
                        let product_price: Result<ProductPrice, diesel::result::Error> =
                            product_prices.filter(product_id.eq(db_product.id).and(price_id.eq(db_price.id))).first(&connection);
                        if let Err(_err) = product_price {
                            ProductPrice::create();
                        }
                    },
                    Err(_) => {
                        Price::create(NewPrice { name: raw_price.name });
                    }
                }
            }
        }

        product
    }

    pub fn update(param_id: i32, product: Product) -> Result<Product, diesel::result::Error> {
        use schema::products::dsl::name;
        let connection = establish_connection();

        diesel::update(products.find(param_id))
            .set((name.eq(product.name),
                  description.eq(product.description)))
            .get_result::<Product>(&connection)
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        let connection = establish_connection();

        diesel::delete(products.find(param_id))
            .execute(&connection)
    }
}

from_data!(Product);
from_data!(NewProduct);
from_data!(FullNewProduct);