use std::io::Read;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::pg::PgConnection;
use schema::product_prices;
use models::db_connection::*;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct ProductPrice {
    pub id: i32,
    pub product_id: i32,
    pub price_id: i32,
    pub price: i32
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name="product_prices"]
pub struct EditableProductPrice {
    pub product_id: Option<i32>,
    pub price_id: i32,
    pub price: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullProductPrice {
    pub price_id: i32,
    pub price: i32,
    pub name: String
}

impl ProductPrice {
    pub fn batch_action(vec_prices: Vec<EditableProductPrice>, product_id: i32) -> Result<bool, diesel::result::Error> {
        use schema::product_prices::dsl;
        let connection = establish_connection();

        for mut editable_product_price in vec_prices {
            editable_product_price.product_id = Some(product_id);

            let result_edit_price = 
                dsl::product_prices
                    .filter(dsl::price_id.eq(editable_product_price.price_id).and(dsl::product_id.eq(product_id)))
                    .first::<ProductPrice>(&connection);

            if let Ok(edit_price) = result_edit_price {
                diesel::update(dsl::product_prices.find(edit_price.id))
                    .set(dsl::price.eq(editable_product_price.price))
                    .get_result::<ProductPrice>(&connection)?;
            } else {
                ProductPrice::create_product_price(&connection, editable_product_price)?;
            }
        }

        Ok(true)
    }

    fn create_product_price(connection: &PgConnection, editable_product_price: EditableProductPrice) -> Result<ProductPrice, diesel::result::Error> {
        diesel::insert_into(product_prices::table)
            .values(&editable_product_price)
            .get_result::<ProductPrice>(connection)
    }
}

from_data!(ProductPrice);
