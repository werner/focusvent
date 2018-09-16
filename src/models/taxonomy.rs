#[macro_export]
macro_rules! taxonomy {
    ($table_model:ident, $type_model:ty, $new_type_model:ty) => {

        pub trait Taxonomy {
            fn list(limit: i64, offset: i64) -> Result<Vec<$type_model>, diesel::result::Error> {
                let connection = ::models::db_connection::establish_connection();

                $table_model
                    .limit(limit)
                    .offset(offset)
                    .load::<$type_model>(&connection)
            }

            fn create(new_type_model: $new_type_model) -> Result<$type_model, diesel::result::Error> {
                let connection = ::models::db_connection::establish_connection();

                diesel::insert_into($table_model::table)
                    .values(&new_type_model)
                    .get_result(&connection)
            }

            fn show(request_id: i32) -> Result<$type_model, diesel::result::Error> {
                let connection = ::models::db_connection::establish_connection();

                $table_model
                    .find(request_id)
                    .get_result::<$type_model>(&connection)
            }

            fn update(param_id: i32, type_model: $type_model) -> Result<$type_model, diesel::result::Error> {
                let connection = ::models::db_connection::establish_connection();

                diesel::update($table_model.find(param_id))
                    .set(&type_model)
                    .get_result::<$type_model>(&connection)
            }

            fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
                let connection = ::models::db_connection::establish_connection();

                diesel::delete($table_model.find(param_id))
                    .execute(&connection)
            }
        }

        impl Taxonomy for $type_model{ }
    };
}
