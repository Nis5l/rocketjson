use super::{ApiErrors, ApiError};

///Easier way to create enum from supported errors
///- Example
///```
///use rocketjson::{ApiResponseErr, error::{ApiErrors, ApiErrorsCreate}};
///
///ApiResponseErr::err(ApiErrors::to_rocketjson_error(error))
///```
pub trait ApiErrorsCreate<TIN> {
    fn to_rocketjson_error(error: TIN) -> ApiErrors;
}

impl ApiErrorsCreate<ApiError> for ApiErrors {
    fn to_rocketjson_error(error: ApiError) -> ApiErrors {
        ApiErrors::ApiError(error)
    }
}

impl<T> ApiErrorsCreate<T> for ApiErrors
    where T: std::error::Error + Send + Sync + 'static {
    fn to_rocketjson_error(error: T) -> ApiErrors {
        ApiErrors::AnyError(anyhow::Error::new(error))
    }
}

///To forward Errors as [`ApiResponseErr`] [`rjtry`] can be used.
///# Requirements
///the trait [`ApiErrorsCreate`] has to be in scope.
///# Example
///```
///use rocketjson::{ApiResponseErr, rjtry, error::ApiErrorsCreate};
///
///pub async fn db_get_users() -> Result<String, diesel::result::Error> {
///    ...
///}
///
///pub async fn is_admin() -> ApiResponseErr<bool> {
///    let user = rjtry!(db_get_users().await);
///    ApiResponseErr::ok(rocket::http::Status::Ok, user == "admin")
///}
///```
#[macro_export]
macro_rules! rjtry {
    ($i:expr) => (
        match $i {
            Result::Ok(val) => val,
            Result::Err(err) => {
                return rocketjson::ApiResponseErr::<_>::err(rocketjson::error::ApiErrors::to_rocketjson_error(err));
            }
        }
    )
}
