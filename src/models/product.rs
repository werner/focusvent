use std::io::Read;
use std::collections::BTreeMap;
use diesel;
use diesel::prelude::*;
use models::db_connection::*;
use models::product_price::ProductPrice;
use models::price::Price;
use schema::product_prices::dsl::*;
use schema::prices::dsl::*;
use models::product_cost::ProductCost;
use models::product_cost::EditableProductCost;
use models::product_cost::EditableProductSupplierCost;
use models::cost::Cost;
use schema::product_costs::dsl::*;
use schema::costs::dsl::*;
use schema::products;
use schema::products::dsl::*;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub stock: Option<f64>
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullNewProduct {
    product: NewProduct,
    prices: BTreeMap<String, i32>,
    costs: Vec<EditableProductSupplierCost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullProduct {
    product: Product,
    prices: BTreeMap<String, i32>,
    costs: Vec<EditableProductSupplierCost>
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
                prices: BTreeMap::new(),
                costs: vec![]
            };
        let vec_products = products
            .find(request_id)
            .left_join(product_prices.left_join(prices))
            .left_join(product_costs.left_join(costs))
            .load::<(Product, Option<(ProductPrice, Option<Price>)>, Option<(ProductCost, Option<Cost>)>)>(&connection)?;

        for (index, db_full_product) in vec_products.into_iter().enumerate() {
            if index == 0 {
                full_product.product = db_full_product.0;
            }
            if let Some(_prices) = db_full_product.1 {
                full_product.prices.insert(_prices.1.unwrap().name, _prices.0.price);
            }
            if let Some(_costs) = db_full_product.2 {
                full_product.costs.push(_costs.0.mapped_to_editable_suppler_product_cost());
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
            ProductCost::batch_action(full_new_product.costs, db_product.id)?;
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
            ProductCost::batch_action(full_product.costs, db_product.id)?;
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
            stock: None
        }
    }
}

from_data!(Product);
from_data!(NewProduct);
from_data!(FullNewProduct);
from_data!(FullProduct);