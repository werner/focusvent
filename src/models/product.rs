use std::io::Read;
use diesel;
use diesel::prelude::*;
use models::db_connection::*;
use models::product_price::ProductPrice;
use models::product_price::EditableProductPrice;
use models::price::Price;
use schema::prices::dsl::*;
use models::product_cost::ProductCost;
use models::product_cost::EditableProductCost;
use models::cost::Cost;
use models::supplier::Supplier;
use schema::costs::dsl::*;
use schema::products;
use schema::products::dsl::*;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug)]
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
    pub prices: Vec<(ProductPrice, Price)>,
    pub costs: Vec<(ProductCost, Cost, Supplier)>
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
        use schema::product_prices;
        use schema::product_costs;
        use schema::suppliers;

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
                .inner_join(costs)
                .inner_join(suppliers::dsl::suppliers)
                .load::<(ProductCost, Cost, Supplier)>(&connection)?;

            let vec_product_prices = product_prices::dsl::product_prices
                .filter(product_prices::dsl::product_id.eq(db_product.id))
                .inner_join(prices)
                .load::<(ProductPrice, Price)>(&connection)?;

            full_product.product = db_product;

            for (product_price, price) in vec_product_prices {
                full_product.prices.push((product_price, price));
            }

            for (product_cost, cost, supplier) in vec_product_costs {
                full_product.costs.push((product_cost, cost, supplier));
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