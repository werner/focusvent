use std::io::Read;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use schema::sale_products;
use models::db_connection::*;
use models::item_calc_methods::ItemCalcMethod;

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Clone,
         Queryable, Debug, FromForm)]
pub struct SaleProduct {
    pub id: i32,
    pub sale_id: i32,
    pub product_id: i32,
    pub tax: Option<i32>,
    pub amount: Option<f64>,
    pub price: Option<i32>,
    pub discount: Option<i32>,
    pub subtotal: Option<f64>
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, FromForm)]
#[table_name="sale_products"]
pub struct NewSaleProduct {
    pub sale_id: i32,
    pub product_id: i32,
    pub tax: Option<i32>,
    pub amount: Option<f64>,
    pub price: Option<i32>,
    pub discount: Option<i32>,
    pub subtotal: Option<f64>
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchSaleProduct {
    pub id: Option<i32>,
    pub sale_id: Option<i32>,
    pub product_id: Option<i32>,
    pub tax: Option<i32>,
    pub amount: Option<f64>,
    pub price: Option<i32>,
    pub discount: Option<i32>,
    pub subtotal: Option<f64>
}

impl SaleProduct {
    pub fn batch_action(vec_sale_products: Vec<NewSaleProduct>, sale_id: i32) -> Result<bool, diesel::result::Error> {
        use schema::sale_products::dsl;
        let connection = establish_connection();

        for mut new_sale_product in vec_sale_products {
            new_sale_product.sale_id = sale_id;

            let result_sale_product = 
                dsl::sale_products
                    .filter(dsl::product_id.eq(new_sale_product.product_id).and(dsl::sale_id.eq(sale_id)))
                    .first::<SaleProduct>(&connection);

            if let Ok(edit_sale_product) = result_sale_product {
                diesel::update(dsl::sale_products.find(edit_sale_product.id))
                    .set((dsl::tax.eq(new_sale_product.tax),
                          dsl::amount.eq(new_sale_product.amount),
                          dsl::price.eq(new_sale_product.price),
                          dsl::discount.eq(new_sale_product.discount),
                          dsl::subtotal.eq(new_sale_product.calculate_total())))
                    .get_result::<SaleProduct>(&connection)?;
            } else {
                diesel::insert_into(sale_products::table)
                    .values(&new_sale_product)
                    .get_result::<SaleProduct>(&connection)?;
            }
        }

        Ok(true)
    }
}

impl NewSaleProduct {
    pub fn to_item_calc_method(&self) -> ItemCalcMethod {
        ItemCalcMethod::new(self.tax, self.discount, self.price, self.amount)
    }

    pub fn calculate_total(&self) -> Option<f64> {
        let item_calc = self.to_item_calc_method();
        Some(item_calc.calculate_total())
    }
}

from_data!(SaleProduct);
from_data!(NewSaleProduct);
