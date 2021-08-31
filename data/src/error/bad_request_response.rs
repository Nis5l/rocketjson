#[derive(serde::Serialize)]
pub struct DefaultError {
    message: String
}

impl DefaultError {
    fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

pub enum BadRequestError {
    DefaultError(rocket::serde::json::Json<DefaultError>),
    ValidationErrors(rocket::serde::json::Json<validator::ValidationErrors>)
}

impl<'r> rocket::response::Responder<'r, 'static> for BadRequestError {
    fn respond_to(self, req: &'r rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            BadRequestError::DefaultError(err) => {
                return rocket::response::Response::build_from(err.respond_to(req).unwrap())
                    .status(rocket::http::Status::BadRequest)
                    .header(rocket::http::ContentType::JSON)
                    .ok();
            },
            BadRequestError::ValidationErrors(err) => {
                return rocket::response::Response::build_from(err.respond_to(req).unwrap())
                    .status(rocket::http::Status::BadRequest)
                    .header(rocket::http::ContentType::JSON)
                    .ok();
            }
        }
    }
}

pub fn get_bad_request_catcher() -> rocket::Catcher {
    rocket::Catcher::new(400, bad_request_catcher)
}

fn bad_request_catcher<'r>(_: rocket::http::Status, req: &'r rocket::Request<'_>) -> rocket::catcher::BoxFuture<'r> {
    use rocket::response::Responder;

    let errors = std::sync::Arc::new(validator::ValidationErrors::new());
    
    let local_cache = req.local_cache(move || errors.clone());

    if local_cache.is_empty() {
        let err = rocket::serde::json::Json::from(DefaultError::new(String::from("Bad Request")));
        return Box::pin(async move { err.respond_to(req) })
    }

    let err = rocket::serde::json::Json::from(local_cache.as_ref().clone());
    Box::pin(async move { err.respond_to(req) })
}
