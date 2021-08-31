#[derive(Debug)]
pub enum JsonBodyError {
    JsonValidationError,
    ValidationError(validator::ValidationErrors)
}
