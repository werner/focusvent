use std::io::Read;
use std::collections::BTreeMap;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::pg::PgConnection;
use schema::product_costs;
use schema::product_costs::dsl::*;
use schema::costs::dsl::*;
use models::cost::Taxonomy;
use models::cost::NewCost;
use models::cost::Cost;
use models::db_connection::*;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Debug)]
pub struct ProductCost {
    pub id: i32,
    pub product_id: i32,
    pub cost_id: i32,
    pub cost: i32
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="product_costs"]
pub struct NewProductCost {
    pub product_id: i32,
    pub cost_id: i32,
    pub cost: i32
}

#[derive(PartialEq)]
enum Action {
    Create,
    Update
}

impl ProductCost {
    pub fn batch_create(hash_costs: BTreeMap<String, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        ProductCost::batch_action(Action::Create, hash_costs, raw_product_id)
    }

    pub fn batch_update(hash_costs: BTreeMap<String, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        ProductCost::batch_action(Action::Update, hash_costs, raw_product_id)
    }

    fn batch_action(action: Action, hash_costs: BTreeMap<String, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        let connection = establish_connection();

        for (new_name_cost, amount) in &hash_costs {
            let mut db_cost: Option<Cost> = None;
            match costs.filter(name.eq(&new_name_cost)).first(&connection) {
                Ok(_cost) => db_cost = Some(_cost),
                Err(_) => {
                    if let Ok(_cost) = Cost::create(NewCost { name: (&new_name_cost).to_string() }) {
                        db_cost = Some(_cost);
                    }
                }
            }

            if let Some(real_db_cost) = db_cost {
                if action == Action::Update {
                    let result_edit_cost = 
                        product_costs
                            .filter(cost_id.eq(real_db_cost.id).and(product_id.eq(raw_product_id)))
                            .first::<ProductCost>(&connection);

                    if let Ok(edit_cost) = result_edit_cost {
                        diesel::update(product_costs.find(edit_cost.id))
                            .set(cost.eq(*amount))
                            .get_result::<ProductCost>(&connection)?;
                    } else {
                        ProductCost::create_product_cost(&connection, raw_product_id, real_db_cost.id, *amount)?;
                    }
                }

                ProductCost::create_product_cost(&connection, raw_product_id, real_db_cost.id, *amount)?;
            }
        }

        Ok(true)
    }

    fn create_product_cost(connection: &PgConnection, param_product_id: i32, param_cost_id: i32, param_cost: i32) -> Result<ProductCost, diesel::result::Error> {
        let new_cost = NewProductCost { product_id: param_product_id, cost_id: param_cost_id, cost: param_cost };

        diesel::insert_into(product_costs::table)
            .values(&new_cost)
            .get_result::<ProductCost>(connection)
    }
}

from_data!(ProductCost);