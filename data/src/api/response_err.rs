//!# ApiResponseErr
//! [`ApiResponseErr`] is returned by enpoints to achieve a Json response success or failure

use crate::error;

///Is returned by enpoints to achieve a Json response success or failure
///Returned can be errors in [`ApiErrors`]. with `ApiResponseErr.err(...)`.
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
///    return rocketjson::ApiResponseErr::api_err(rocket::http::Status::InternalServerError, String::from("login failed"))
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
    pub json: Result<rocket::serde::json::Json<T>, error::ApiErrors>,
    /// This is the Statuscode sent to the client, it is not included in the Json
    pub status: Option<rocket::http::Status>,
}

impl<T> ApiResponseErr<T> {
    pub fn ok(status: rocket::http::Status, json_data: T) -> Self {
        Self {
            status: Some(status),
            json: Ok(rocket::serde::json::Json::from(json_data))
        }
    }

    pub fn api_err(status: rocket::http::Status, error: String) -> Self {
        Self {
            status: Some(status),
            json: Err(error::ApiErrors::ApiError(error::ApiError::new(status, error)))
        }
    }

    pub fn err(error: error::ApiErrors) -> Self {
        Self {
            status: None,
            json: Err(error)
        }
    }

    pub fn get_status(&self) -> rocket::http::Status {
        if self.status.is_none() {
            return rocket::http::Status::InternalServerError
        }

        return self.status.unwrap();
    }
}

impl<'r, T> rocket::response::Responder<'r, 'static> for ApiResponseErr<T> where T: serde::Serialize {
    fn respond_to(self, req: &'r rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        let status = self.get_status();
        rocket::response::Response::build_from(self.json.respond_to(req).unwrap())
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}
