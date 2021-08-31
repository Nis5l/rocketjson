#[derive(Debug)]
pub enum JsonBodyError {
    JsonValidationError,
    ValidationError(validator::ValidationErrors),
    Io(std::io::Error),
}
