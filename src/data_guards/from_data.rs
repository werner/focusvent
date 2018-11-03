use std::io::Read;
use std::ops::Deref;
use serde_json::from_str;
use rocket::data::{FromData, Outcome};
use rocket::{Request, Data};
use rocket::Outcome::{Failure, Success};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
pub struct RequestResource<T>(pub T);

impl<T> Deref for RequestResource<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> FromData for RequestResource<T>
    where T: for<'de> serde::Deserialize<'de>
{
    type Error = String;

    fn from_data(_: &Request, data: Data) -> Outcome<Self, String> {

        let mut string_data = String::new();
        if let Err(e) = data.open().read_to_string(&mut string_data) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let object: RequestResource<T> = match from_str(&string_data) {
            Ok(value) => value,
            Err(err) => {
                println!("Error deserializing {:#?} {:?}", &string_data, err);
                return Failure((Status::BadRequest,
                    format!("Error deserializing {:?} {:?}", &string_data, err),
                ));
            }
        };

        Success(object)
    }
}
