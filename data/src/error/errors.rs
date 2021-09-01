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

#[derive(Debug)]
pub enum JsonBodyError {
    JsonValidationError,
    ValidationError(validator::ValidationErrors)
}
