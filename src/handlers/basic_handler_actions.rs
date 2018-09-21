#[macro_export]
macro_rules! basic_handler_actions {
    ($resource:expr, $model:ident, $new_model:ident) => {

        impl ::std::str::FromStr for $model {
            type Err = ::serde_json::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                ::serde_json::from_str(s)
            }
        }

        pub static STATIC_ROCKET_ROUTE_INFO_FOR_INDEX: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                method: ::rocket::http::Method::Get,
                path: concat!("/", $resource, "?<params>"),
                handler: rocket_route_fn_index,
                format: None,
                rank: None,
            };

        fn rocket_route_fn_index<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case)]
            let rocket_param_params: GetTransactionParams<$model> = {
                let mut items = ::rocket::request::FormItems::from(match __req.uri().query() {
                    Some(query) => query,
                    None => return ::rocket::Outcome::Forward(__data),
                });
                let form = ::rocket::request::FromForm::from_form(items.by_ref(), true);
                #[allow(unreachable_patterns)]
                let obj = match form {
                    Ok(v) => v,
                    Err(_) => return ::rocket::Outcome::Forward(__data),
                };
                if !items.exhaust() {
                    {
                        ::std::io::_print(::std::fmt::Arguments::new_v1_formatted(
                            &["    => The query string ", " is malformed.\n"],
                            &match (&match __req.uri().query() {
                                Some(query) => query,
                                None => return ::rocket::Outcome::Forward(__data),
                            },)
                            {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)]
                                }
                            },
                            &[::std::fmt::rt::v1::Argument {
                                position: ::std::fmt::rt::v1::Position::At(0usize),
                                format: ::std::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::std::fmt::rt::v1::Count::Implied,
                                    width: ::std::fmt::rt::v1::Count::Implied,
                                },
                            }],
                        ));
                    };
                    return ::rocket::Outcome::Failure(::rocket::http::Status::BadRequest);
                }
                obj
            };
            let responder = index(rocket_param_params);
            ::rocket::handler::Outcome::from(__req, responder)
        }

        pub fn index(
            params: GetTransactionParams<$model>,
        ) -> Result<Json<Vec<$model>>, status::Custom<String>> {
            match $model::list(params.limit.unwrap_or(10), params.offset.unwrap_or(0)) {
                Ok(records) => Ok(Json(records)),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        }

        pub fn index_route() -> ::rocket::Route {
            ::rocket::Route::from(&STATIC_ROCKET_ROUTE_INFO_FOR_INDEX)
        }

        pub static STATIC_ROCKET_ROUTE_INFO_FOR_SHOW: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                method: ::rocket::http::Method::Get,
                path: concat!("/", $resource, "/<id>"),
                handler: rocket_route_fn_show,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::Source::None,
                    top: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed("json")),
                    params: ::rocket::http::MediaParams::Static(&[]),
                }),
                rank: None,
            };

        #[allow(unreachable_code)]
        fn rocket_route_fn_show<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns)]
            let rocket_param_id: i32 = match match __req.get_param_str(0usize) {
                Some(s) => <i32 as ::rocket::request::FromParam>::from_param(s),
                None => return ::rocket::Outcome::Forward(__data),
            } {
                Ok(v) => v,
                Err(e) => {
                    {
                        ::std::io::_print(::std::fmt::Arguments::new_v1_formatted(
                            &["    => Failed to parse \'", "\': ", "\n"],
                            &match (&"id", &e) {
                                (arg0, arg1) => [
                                    ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                    ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(0usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(1usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                        ));
                    };
                    return ::rocket::Outcome::Forward(__data);
                }
            };
            let responder = show(rocket_param_id);
            ::rocket::handler::Outcome::from(__req, responder)
        }

        pub fn show(id: i32) -> Result<Json<$model>, status::Custom<String>> {
            match $model::show(id) {
                Ok(record) => Ok(Json(record)),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        }

        pub fn show_route() -> ::rocket::Route {
            ::rocket::Route::from(&STATIC_ROCKET_ROUTE_INFO_FOR_SHOW)
        }

        pub static STATIC_ROCKET_ROUTE_INFO_FOR_CREATE: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                method: ::rocket::http::Method::Post,
                path: concat!("/", $resource),
                handler: rocket_route_fn_create,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::Source::None,
                    top: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed("json")),
                    params: ::rocket::http::MediaParams::Static(&[]),
                }),
                rank: None,
            };

        #[allow(unreachable_code)]
        fn rocket_route_fn_create<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns)]
            let rocket_param_record: $new_model =
                match ::rocket::data::FromData::from_data(__req, __data) {
                    ::rocket::Outcome::Success(d) => d,
                    ::rocket::Outcome::Forward(d) => return ::rocket::Outcome::Forward(d),
                    ::rocket::Outcome::Failure((code, _)) => {
                        return ::rocket::Outcome::Failure(code);
                    }
                };
            let responder = create(rocket_param_record);
            ::rocket::handler::Outcome::from(__req, responder)
        }

        pub fn create(record: $new_model) -> Result<Json<$model>, status::Custom<String>> {
            match $model::create(record) {
                Ok(new_record) => Ok(Json(new_record)),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        }

        pub fn create_route() -> ::rocket::Route {
            ::rocket::Route::from(&STATIC_ROCKET_ROUTE_INFO_FOR_CREATE)
        }

        pub static STATIC_ROCKET_ROUTE_INFO_FOR_UPDATE: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                method: ::rocket::http::Method::Put,
                path: concat!("/", $resource, "/<id>"),
                handler: rocket_route_fn_update,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::Source::None,
                    top: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed("json")),
                    params: ::rocket::http::MediaParams::Static(&[]),
                }),
                rank: None,
            };

        #[allow(unreachable_code)]
        fn rocket_route_fn_update<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns)]
            let rocket_param_id: i32 = match match __req.get_param_str(0usize) {
                Some(s) => <i32 as ::rocket::request::FromParam>::from_param(s),
                None => return ::rocket::Outcome::Forward(__data),
            } {
                Ok(v) => v,
                Err(e) => {
                    {
                        ::std::io::_print(::std::fmt::Arguments::new_v1_formatted(
                            &["    => Failed to parse \'", "\': ", "\n"],
                            &match (&"id", &e) {
                                (arg0, arg1) => [
                                    ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                    ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(0usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(1usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                        ));
                    };
                    return ::rocket::Outcome::Forward(__data);
                }
            };
            #[allow(non_snake_case, unreachable_patterns)]
            let rocket_param_record: $model =
                match ::rocket::data::FromData::from_data(__req, __data) {
                    ::rocket::Outcome::Success(d) => d,
                    ::rocket::Outcome::Forward(d) => return ::rocket::Outcome::Forward(d),
                    ::rocket::Outcome::Failure((code, _)) => {
                        return ::rocket::Outcome::Failure(code);
                    }
                };
            let responder = update(rocket_param_id, rocket_param_record);
            ::rocket::handler::Outcome::from(__req, responder)
        }

        pub fn update(id: i32, record: $model) -> Result<Json<$model>, status::Custom<String>> {
            match $model::update(id, record) {
                Ok(updated_record) => Ok(Json(updated_record)),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        }

        pub fn update_route() -> ::rocket::Route {
            ::rocket::Route::from(&STATIC_ROCKET_ROUTE_INFO_FOR_UPDATE)
        }

        pub static STATIC_ROCKET_ROUTE_INFO_FOR_DELETE: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                method: ::rocket::http::Method::Delete,
                path: concat!("/", $resource, "/<id>"),
                handler: rocket_route_fn_delete,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::Source::None,
                    top: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::IndexedStr::Concrete(::std::borrow::Cow::Borrowed("json")),
                    params: ::rocket::http::MediaParams::Static(&[]),
                }),
                rank: None,
            };

        #[allow(unreachable_code)]
        fn rocket_route_fn_delete<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns)]
            let rocket_param_id: i32 = match match __req.get_param_str(0usize) {
                Some(s) => <i32 as ::rocket::request::FromParam>::from_param(s),
                None => return ::rocket::Outcome::Forward(__data),
            } {
                Ok(v) => v,
                Err(e) => {
                    {
                        ::std::io::_print(::std::fmt::Arguments::new_v1_formatted(
                            &["    => Failed to parse \'", "\': ", "\n"],
                            &match (&"id", &e) {
                                (arg0, arg1) => [
                                    ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                    ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(0usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(1usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                        ));
                    };
                    return ::rocket::Outcome::Forward(__data);
                }
            };
            let responder = delete(rocket_param_id);
            ::rocket::handler::Outcome::from(__req, responder)
        }

        pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
            match $model::delete(id) {
                Ok(qid) => Ok(Json(qid)),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        }

        pub fn delete_route() -> ::rocket::Route {
            ::rocket::Route::from(&STATIC_ROCKET_ROUTE_INFO_FOR_DELETE)
        }
    };
}
