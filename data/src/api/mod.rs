//!# Api
//!Should be returned by the endpoints in order to achieve a Json response
//!# Example
//!- Code
//!```
//!#[derive(serde::Serialize)]
//!pub struct TestResponse {
//!   data: String
//!}
//!
//!#[post("/test")]
//!pub fn test() -> rocketjson::ApiResponse<RegisterResponse> {
//!     rocketjson::ApiResponse::new(
//!         rocket::http::Status::Ok, RegisterResponse {
//!         data: String::from("test")
//!     })
//!}
//!```
//!- Response (200 OK)
//!```
//!{
//!     data: "test"
//!}
//!```
pub mod response;
pub mod response_err;

pub use response::ApiResponse;
pub use response_err::ApiResponseErr;
