#[derive(Debug)]
pub enum ApiErrors {
    ApiError(ApiError),
    DieselError(diesel::result::Error),
    SqlxError(sqlx::Error),
}

#[derive(Debug)]
pub struct ApiError {
    pub status: rocket::http::Status,
    pub error: String
}

impl ApiError {
    pub fn new(status: rocket::http::Status, error: String) -> Self {
        Self {
            status,
            error
        }
    }
}

///Setting the local_cache to a [`JsonBodyError`] when returning a failed Outcome will output the error as JSON.
///
///```
///#[rocket::async_trait]
///impl<'r> FromRequest<'r> for ... {
///    type Error = ();
///    async fn from_request(req: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
///        req.local_cache(|| rocketjson::error::JsonBodyError::CustomError(String::from("Custom error occured")));
///        rocket::request::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
///     }
/// }
///```
#[derive(Debug)]
pub enum JsonBodyError {
    NoError,
    CustomError(String),
    JsonValidationError,
    ValidationError(validator::ValidationErrors),
}
