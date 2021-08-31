//!# Errors and Error-Handling
//!
//!# Catcher
//![`get_catcher`] should be registered in order to transform errors into Json.
//!```
//!#[launch]
//!fn rocket() -> _ {
//!    rocket::build()
//!        .mount("/", routes![]).
//!        register("/", vec![rocketjson::error::get_catcher()])
//!}
//!```
//!# Output
//!- Errors
//!```
//!{
//!     "error": "Bad Request"
//!}
//!```
//!- ValidationError
//!Validation error structure is handled by [`Validator`]
//!```
//!{
//!    "username": [
//!        {
//!            "code": "length",
//!            "message": null,
//!            "params": {
//!                "value": "",
//!                "min": 1
//!            }
//!        }
//!    ]
//!}
//!```
//![`Validator`]: https://github.com/Keats/validator

pub mod error_handling;
pub mod json_body_error;

pub use error_handling::get_catcher;
pub use json_body_error::JsonBodyError;
