//!# Response
//! [`ApiResponse`] is returned by enpoints to achieve a Json Response


///Is returned by enpoints to achieve a Json Response
///# Requirements
///T has to implement [`serde::Serialize`]
///# Example
///- Code
///```
///#[derive(serde::Serialize)]
///pub struct TestResponse {
///   data: String
///}
///
///#[post("/test")]
///pub fn test() -> rocketjson::ApiResponse<RegisterResponse> {
///     rocketjson::ApiResponse::new(
///         rocket::http::Status::Ok, RegisterResponse {
///         data: String::from("test")
///     })
///}
///```
///- Response (200 OK)
///```
///{
///     data: "test"
///}
///```
///[`serde::Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
#[derive(Debug)]
pub struct ApiResponse<T> {
    /// This is the Json-data sent to the client
    pub json: rocket::serde::json::Json<T>,
    /// This is the Statuscode sent to the client, it is not included in the Json
    pub status: rocket::http::Status,
}

impl<T> ApiResponse<T> {
    pub fn new(status: rocket::http::Status, json_data: T) -> Self {
        Self {
            status,
            json: rocket::serde::json::Json::from(json_data)
        }
    }
}

impl<'r, T> rocket::response::Responder<'r, 'static> for ApiResponse<T> where T: serde::Serialize {
    fn respond_to(self, req: &'r rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        rocket::response::Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

