use super::{ApiErrors, ApiError};

///Easier way to create enum from supported errors
pub trait ApiErrorsCreate<TIN> {
    fn to_rocketjson_error(error: TIN) -> ApiErrors;
}

impl ApiErrorsCreate<diesel::result::Error> for ApiErrors {
    fn to_rocketjson_error(error: diesel::result::Error) -> ApiErrors {
        ApiErrors::DieselError(error)
    }
}

impl ApiErrorsCreate<ApiError> for ApiErrors {
    fn to_rocketjson_error(error: ApiError) -> ApiErrors {
        ApiErrors::ApiError(error)
    }
}


///To forward Errors as [`ApiResponseErr`] [`rjtry`] can be used.
///# Example
///```
///pub async fn db_get_users() -> Result<String, diesel::result::Error> {
///    ...
///}
///
///pub async fn is_admin() -> ApiResponseErr<bool> {
///    let user = rjtry!(db_get_users().await);
///    user == "admin"
///}
///```
#[macro_export]
macro_rules! rjtry {
    ($i:expr) => (
        match $i {
            Result::Ok(val) => val,
            Result::Err(err) => {
                return rocketjson::ApiResponseErr::<_>::err(rocketjson::error::ApiErrors::DieselError(err));
            }
        }
    )
}
