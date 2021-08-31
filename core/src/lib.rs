//!# Crate for working with Rocket and Json.
//!
//! Ultimately the goal is to have validated Structs enter and leave the endpoint as Json
//! while having everything happen in the background.
//!
//!# Example
//!```
//!#[macro_use] extern crate rocket;
//!
//!#[derive(serde::Deserialize, validator::Validate, rocketjson::JsonBody)]
//!pub struct RegisterRequest {
//!   #[validate(length(min = 1))]
//!   username: String 
//!}
//!
//!#[derive(serde::Serialize)]
//!pub struct RegisterResponse {
//!   message: String
//!}
//!
//!#[post("/register", data="<data>")]
//!pub fn register(data: RegisterRequest) -> rocketjson::ApiResponse<RegisterResponse> {
//!   rocketjson::ApiResponse::new(rocket::http::Status::Ok, RegisterResponse { message: format!("Welcome {}", data.username) })
//!}
//!
//!#[launch]
//!fn rocket() -> _ {
//!    rocket::build()
//!        .mount("/", routes![register]).
//!        register("/", vec![rocketjson::error::get_catcher()])
//!}
//!```
//!- Input 
//!```
//!{
//!    "username": "testuser"
//!}
//!```
//!- Output 200 OK
//!```
//!{
//!    "message": "Welcome testuser"
//!}
//!```
//!- Input
//!```
//!{
//!    "username": ""
//!}
//!```
//!- Output 400 Bad Request
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

pub use rocketjson_macro::*;
pub use rocketjson_data::*;
