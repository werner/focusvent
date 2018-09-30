use std::io::Read;
use schema::sale_products;

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Clone,
         Queryable, Debug, FromForm)]
pub struct SaleProduct {
    pub id: i32,
    pub sale_id: i32,
    pub tax_id: i32,
    pub product_id: i32,
    pub amount: Option<f64>,
    pub price: Option<i32>,
    pub discount: Option<i32>,
    pub subtotal: Option<i32>
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, FromForm)]
#[table_name="sale_products"]
pub struct NewSaleProduct {
    pub sale_id: i32,
    pub tax_id: i32,
    pub product_id: i32,
    pub amount: Option<f64>,
    pub price: Option<i32>,
    pub discount: Option<i32>,
    pub subtotal: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchSaleProduct {
    pub id: Option<i32>,
    pub sale_id: Option<i32>,
    pub tax_id: Option<i32>,
    pub product_id: Option<i32>,
    pub amount: Option<f64>,
    pub price: Option<i32>,
    pub discount: Option<i32>,
    pub subtotal: Option<i32>
}

from_data!(SaleProduct);
from_data!(NewSaleProduct);
