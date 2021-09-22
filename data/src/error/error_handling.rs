//!# Handeling errors
//!Use [`get_catcher`] to transform errors into Json.

use crate::error::{self, JsonBodyError};

///Transforms errors into Json
///# Example
///```
///#[launch]
///fn rocket() -> _ {
///    rocket::build()
///        .mount("/", routes![]).
///        register("/", vec![rocketjson::error::get_catcher()])
///}
///```
pub fn get_catcher() -> rocket::Catcher {
    rocket::Catcher::new(None, request_catcher)
}

#[derive(serde::Serialize)]
struct DefaultError {
    error: String
}

impl DefaultError {
    fn new(error: String) -> Self {
        Self {
            error
        }
    }
}

fn request_catcher<'r>(status: rocket::http::Status, req: &'r rocket::Request<'_>) -> rocket::catcher::BoxFuture<'r> {
    use rocket::response::Responder;

    loop {
        let local_cache = req.local_cache(move || JsonBodyError::NoError);

        match local_cache {
            JsonBodyError::NoError => break,
            JsonBodyError::ValidationError(validation_errors) => {
                let err = rocket::serde::json::Json::from(validation_errors.clone());
                return Box::pin(async move {
                    rocket::response::Response::build_from(err.respond_to(req).unwrap())
                    .status(rocket::http::Status::BadRequest)
                    .header(rocket::http::ContentType::JSON)
                    .ok()
                });
            },
            JsonBodyError::JsonValidationError => {
                let err = rocket::serde::json::Json::from(DefaultError::new(String::from("Parsing JSON failed")));

                return Box::pin(async move {
                    rocket::response::Response::build_from(err.respond_to(req).unwrap())
                    .status(status)
                    .header(rocket::http::ContentType::JSON)
                    .ok()
                })
            },
            JsonBodyError::CustomError(message) => {
                let err = rocket::serde::json::Json::from(DefaultError::new(message.clone()));

                return Box::pin(async move {
                    rocket::response::Response::build_from(err.respond_to(req).unwrap())
                    .status(status)
                    .header(rocket::http::ContentType::JSON)
                    .ok()
                })
            }
        }
    }

    let message = if status.reason().is_some() {
        String::from(status.reason().unwrap())
    } else {
        format!("Unknown error: {}", status.code)
    };

    let err = rocket::serde::json::Json::from(DefaultError::new(message));

    return Box::pin(async move {
        rocket::response::Response::build_from(err.respond_to(req).unwrap())
        .status(status)
        .header(rocket::http::ContentType::JSON)
        .ok()
    })
}

impl<'r> rocket::response::Responder<'r, 'static> for error::ApiErrors {
    fn respond_to(self, req: &'r rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            error::ApiErrors::ApiError(err) => {
                let json = rocket::serde::json::Json::from(DefaultError::new(err.error));

                rocket::response::Response::build_from(json.respond_to(req).unwrap())
                    .status(err.status)
                    .header(rocket::http::ContentType::JSON)
                    .ok()
            },
            error::ApiErrors::DieselError(_err) => {
                let json = rocket::serde::json::Json::from(DefaultError::new(String::from("Database error")));

                rocket::response::Response::build_from(json.respond_to(req).unwrap())

                    .status(rocket::http::Status::InternalServerError)
                    .header(rocket::http::ContentType::JSON)
                    .ok()
            }
        }
    }
}
