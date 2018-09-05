use std::io::Read;
use std::collections::BTreeMap;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::pg::PgConnection;
use schema::product_costs;
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
    pub supplier_id: i32,
    pub cost: i32
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="product_costs"]
pub struct EditableProductCost {
    pub product_id: i32,
    pub cost_id: i32,
    pub supplier_id: i32,
    pub cost: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditableProductSupplierCost {
    pub cost_id: i32,
    pub supplier_id: i32,
    pub cost: i32
}

impl EditableProductSupplierCost {
    pub fn mapped_to_editable_product_cost(&self, cost_id: i32, product_id: i32) -> EditableProductCost {
        EditableProductCost {
            product_id: product_id,
            cost_id: cost_id,
            supplier_id: self.supplier_id,
            cost: self.cost
        }
    }
}

impl ProductCost {
    pub fn batch_action(vec_costs: Vec<EditableProductSupplierCost>, product_id: i32) -> Result<bool, diesel::result::Error> {
        use schema::product_costs::dsl;
        let connection = establish_connection();

        for mut product_cost in vec_costs {
            let db_cost: Cost = costs.find(&product_cost.cost_id).get_result(&connection)?;
            let editable_product_cost: EditableProductCost = product_cost.mapped_to_editable_product_cost(db_cost.id, product_id);

            let result_edit_cost = 
                dsl::product_costs
                    .filter(dsl::cost_id.eq(db_cost.id).and(dsl::product_id.eq(product_id)))
                    .first::<ProductCost>(&connection);

            if let Ok(edit_cost) = result_edit_cost {
                diesel::update(dsl::product_costs.find(edit_cost.id))
                    .set((dsl::cost.eq(product_cost.cost),
                          dsl::supplier_id.eq(product_cost.supplier_id)))
                    .get_result::<ProductCost>(&connection)?;
            } else {
                ProductCost::create_product_cost(&connection, &editable_product_cost)?;
            }
        }

        Ok(true)
    }

    pub fn mapped_to_editable_suppler_product_cost(&self) -> EditableProductSupplierCost {
        EditableProductSupplierCost {
            cost_id: self.cost_id,
            supplier_id: self.supplier_id,
            cost: self.cost
        }
    }

    fn create_product_cost(connection: &PgConnection, product_cost: &EditableProductCost) -> Result<ProductCost, diesel::result::Error> {
        diesel::insert_into(product_costs::table)
            .values(product_cost)
            .get_result::<ProductCost>(connection)
    }
}

from_data!(ProductCost);