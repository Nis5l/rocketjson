#[derive(Debug)]
pub struct ApiResponse<T> {
    pub json: rocket::serde::json::Json<T>,
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

