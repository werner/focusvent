use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::pg::PgConnection;
use crate::schema::product_costs;
use crate::models::db_connection::*;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Debug)]
pub struct ProductCost {
    pub id: i32,
    pub product_id: i32,
    pub cost_id: i32,
    pub supplier_id: i32,
    pub cost: i32
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[table_name="product_costs"]
pub struct EditableProductCost {
    pub product_id: Option<i32>,
    pub cost_id: i32,
    pub supplier_id: i32,
    pub cost: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullProductCost {
    pub cost_id: i32,
    pub supplier_id: i32,
    pub cost: i32,
    pub name: String
}

impl ProductCost {
    pub fn batch_action(vec_costs: Vec<EditableProductCost>, product_id: i32) -> Result<bool, diesel::result::Error> {
        use crate::schema::product_costs::dsl;
        let connection = establish_connection();

        for mut product_cost in vec_costs {
            product_cost.product_id = Some(product_id);

            let result_edit_cost = 
                dsl::product_costs
                    .filter(dsl::cost_id.eq(product_cost.cost_id).and(dsl::product_id.eq(product_id)))
                    .first::<ProductCost>(&connection);

            if let Ok(edit_cost) = result_edit_cost {
                diesel::update(dsl::product_costs.find(edit_cost.id))
                    .set((dsl::cost.eq(product_cost.cost),
                          dsl::supplier_id.eq(product_cost.supplier_id)))
                    .get_result::<ProductCost>(&connection)?;
            } else {
                ProductCost::create_product_cost(&connection, &product_cost)?;
            }
        }

        Ok(true)
    }

    fn create_product_cost(connection: &PgConnection, product_cost: &EditableProductCost) -> Result<ProductCost, diesel::result::Error> {
        diesel::insert_into(product_costs::table)
            .values(product_cost)
            .get_result::<ProductCost>(connection)
    }
}
