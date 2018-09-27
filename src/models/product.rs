use std::io::Read;
use diesel;
use diesel::prelude::*;
use handlers::base::Search;
use models::db_connection::*;
use models::product_price::ProductPrice;
use models::product_price::EditableProductPrice;
use models::product_price::FullProductPrice;
use models::price::Price;
use models::product_cost::ProductCost;
use models::product_cost::EditableProductCost;
use models::product_cost::FullProductCost;
use models::cost::Cost;
use models::supplier::Supplier;
use schema::products;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, FromForm)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub stock: Option<f64>,
    pub code: Option<String>
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullNewProduct {
    product: NewProduct,
    prices: Vec<EditableProductPrice>,
    costs: Vec<EditableProductCost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullProduct {
    pub product: Product,
    pub prices: Vec<FullProductPrice>,
    pub costs: Vec<FullProductCost>
}

impl Product {
    pub fn list(limit: i64, offset: i64, search: Option<Search<Product>>) -> Result<Vec<Product>, diesel::result::Error> {
        use schema::products::dsl::*;
        let connection = establish_connection();
        
        let results: Result<Vec<Product>, diesel::result::Error>;
        let product_to_search: Product;
        if let Some(search_product) = search {
            let Search(_product) = search_product;
            product_to_search = _product;
            results = products
                .filter(name.like(product_to_search.name))
                .limit(limit)
                .offset(offset)
                .load(&connection);
        } else {
            results = products
                .limit(limit)
                .offset(offset)
                .load(&connection);
        }

        results
    }

    pub fn show(request_id: i32) -> Result<FullProduct, diesel::result::Error> {
        use schema::products::dsl::*;
        use schema::product_prices;
        use schema::product_costs;
        use schema::suppliers;
        use schema::costs;
        use schema::prices;

        let connection = establish_connection();
        let mut full_product: FullProduct =
            FullProduct { 
                product: Product::blank_product(),
                prices: vec![],
                costs: vec![]
            };
        let vec_products = products
            .find(request_id)
            .load::<Product>(&connection)?;

        for db_product in vec_products.into_iter() {

            let vec_product_costs = product_costs::dsl::product_costs
                .filter(product_costs::dsl::product_id.eq(db_product.id))
                .inner_join(costs::dsl::costs)
                .inner_join(suppliers::dsl::suppliers)
                .order(costs::name)
                .load::<(ProductCost, Cost, Supplier)>(&connection)?;

            let vec_product_prices = product_prices::dsl::product_prices
                .filter(product_prices::dsl::product_id.eq(db_product.id))
                .inner_join(prices::dsl::prices)
                .order(prices::name)
                .load::<(ProductPrice, Price)>(&connection)?;

            full_product.product = db_product;

            for (product_price, price) in vec_product_prices {
                full_product.prices.push(
                    FullProductPrice {
                        price_id: price.id,
                        price: product_price.price,
                        name: price.name
                    }
                );
            }

            for (product_cost, cost, supplier) in vec_product_costs {
                full_product.costs.push(
                    FullProductCost {
                        cost_id: cost.id,
                        supplier_id: supplier.id,
                        cost: product_cost.cost,
                        name: cost.name
                    }
                );
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
            ProductPrice::batch_action(full_new_product.prices, db_product.id)?;
            ProductCost::batch_action(full_new_product.costs, db_product.id)?;
        }

        product
    }

    pub fn update(param_id: i32, full_product: FullNewProduct) -> Result<Product, diesel::result::Error> {
        use schema::products::dsl::name;
        use schema::products::dsl::*;
        let connection = establish_connection();

        let product = diesel::update(products.find(param_id))
            .set((name.eq(full_product.product.name),
                  code.eq(full_product.product.code),
                  description.eq(full_product.product.description)))
            .get_result::<Product>(&connection);

        if let Ok(db_product) = &product {
            ProductPrice::batch_action(full_product.prices, db_product.id)?;
            ProductCost::batch_action(full_product.costs, db_product.id)?;
        }

        product
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        use schema::products::dsl::*;
        let connection = establish_connection();

        diesel::delete(products.find(param_id))
            .execute(&connection)
    }

    fn blank_product() -> Product {
        Product {
            id: 0,
            code: None,
            name: "".to_string(),
            description: None,
            stock: None
        }
    }
}

from_data!(Product);
from_data!(NewProduct);
from_data!(FullNewProduct);
from_data!(FullProduct);

use std::str::FromStr;
use serde_json;

impl FromStr for Product {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}