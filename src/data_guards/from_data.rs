#[macro_export]
macro_rules! from_data {
    ($type:ty) => {
        impl ::rocket::data::FromData for $type {
            type Error = String;

            fn from_data(
                _: &::rocket::Request,
                data: ::rocket::Data,
            ) -> ::rocket::data::Outcome<Self, String> {

                let mut string_data = String::new();
                if let Err(e) = data.open().read_to_string(&mut string_data) {
                    return ::rocket::Outcome::Failure((
                        ::rocket::http::Status::InternalServerError,
                        format!("{:?}", e),
                    ));
                }

                let object: $type = match ::serde_json::from_str(&string_data) {
                    Ok(value) => value,
                    Err(err) => {
                        println!("Error deserializing {:?}", &string_data);
                        return ::rocket::Outcome::Failure((
                            ::rocket::http::Status::BadRequest,
                            format!("Error deserializing {:?}", err),
                        ));
                    }
                };

                ::rocket::Outcome::Success(object)
            }
        }
    };
}
