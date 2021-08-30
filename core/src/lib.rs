pub use rocket_json_macro::*;

#[derive(Debug)]
pub enum JsonBodyError {
    TooLarge,
    //TODO: forward error
    JsonValidationError,
    ValidationError,
    Io(std::io::Error),
}
