//!# ApiResponseErr
//! [`ApiResponseErr`] is returned by enpoints to achieve a Json response success or failure

use crate::error;

///Is returned by enpoints to achieve a Json response success or failure
///# Example
///```
///#[derive(serde::Deserialize, validator::Validate, rocketjson::JsonBody)]
///pub struct LoginRequest {
///    #[validate(length(min = 1))]
///    username: String,
///    #[validate(length(min = 1))]
///    password: String
///}
///
///#[derive(serde::Serialize)]
///pub struct LoginResponse {
///    message: String
///}
///
///#[post("/login", data="<data>")]
///pub fn login(data: LoginRequest) -> rocketjson::ApiResponseErr<LoginResponse> {
///    if data.username == "admin" && data.password == "admin" {
///        return rocketjson::ApiResponseErr::ok(
///            rocket::http::Status::Ok,
///            LoginResponse{ message: "logged in" }
///        );
///    }
///
///    return rocketjson::ApiResponseErr::err(rocket::http::Status::InternalServerError, String::from("login failed"))
///}
///```
///- Input
///```
///{
///    "username": "admin",
///    "password": "admin"
///}
///```
///- Output (200 OK)
///```
///{
///    "message": "logged in"
///}
///```
///- Input
///```
///{
///    "username": "test",
///    "password": "test"
///}
///```
///- Output (500 Internal Server Error)
///```
///{
///    "error": "login failed"
///}
///```
#[derive(Debug)]
pub struct ApiResponseErr<T> {
    /// This is the Json-data sent to the client
    pub json: Result<rocket::serde::json::Json<T>, error::ApiError>,
    /// This is the Statuscode sent to the client, it is not included in the Json
    pub status: rocket::http::Status,
}

impl<T> ApiResponseErr<T> {
    pub fn ok(status: rocket::http::Status, json_data: T) -> Self {
        Self {
            status,
            json: Ok(rocket::serde::json::Json::from(json_data))
        }
    }

    pub fn err(status: rocket::http::Status, error: String) -> Self {
        Self {
            status,
            json: Err(error::ApiError::new(status, error))
        }
    }
}

impl<'r, T> rocket::response::Responder<'r, 'static> for ApiResponseErr<T> where T: serde::Serialize {
    fn respond_to(self, req: &'r rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        rocket::response::Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}
