#[macro_export]
macro_rules! basic_handler_actions {
    ($resource:expr, $model:ident) => {
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
            let rocket_param_params: GetTransactionParams = {
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

        #[rocket_route(STATIC_ROCKET_ROUTE_INFO_FOR_INDEX)]
        pub fn index(
            params: GetTransactionParams,
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

        #[rocket_route_info]
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

        #[rocket_route(STATIC_ROCKET_ROUTE_INFO_FOR_SHOW)]
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

        // #[post("/$resource", format="application/json", data="<client>")]
        // pub fn create(client: NewClient) -> Result<Json<Client>, status::Custom<String>> {
        //     match Client::create(client) {
        //         Ok(client) => Ok(Json(client)),
        //         Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
        //     }
        // }

        // #[put("/$resource/<id>", format="application/json", data="<client>")]
        // pub fn update(id: i32, client: Client) -> Result<Json<Client>, status::Custom<String>> {
        //     match Client::update(id, client) {
        //         Ok(client) => Ok(Json(client)),
        //         Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
        //     }
        // }

        // #[delete("/$resource/<id>", format="application/json")]
        // pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
        //     match Client::delete(id) {
        //         Ok(qid) => Ok(Json(qid)),
        //         Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
        //     }
        // }
    };
}
