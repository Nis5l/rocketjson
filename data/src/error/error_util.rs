use super::{ApiErrors, ApiError};

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
