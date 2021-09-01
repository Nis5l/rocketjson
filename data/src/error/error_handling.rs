//!# Handeling errors
//!Use [`get_catcher`] to transform errors into Json.

///Transforms errors into Json
///# Example
///```
///#[launch]
///fn rocket() -> _ {
///    rocket::build()
///        .mount("/", routes![]).
///        register("/", vec![rocketjson::error::get_catcher()])
///}
///```
pub fn get_catcher() -> rocket::Catcher {
    rocket::Catcher::new(None, request_catcher)
}

#[derive(serde::Serialize)]
struct DefaultError {
    error: String
}

impl DefaultError {
    fn new(error: String) -> Self {
        Self {
            error
        }
    }
}

fn request_catcher<'r>(status: rocket::http::Status, req: &'r rocket::Request<'_>) -> rocket::catcher::BoxFuture<'r> {
    use rocket::response::Responder;

    loop { match status.code {
        400 => {
            let local_cache = req.local_cache(move || std::sync::Arc::new(validator::ValidationErrors::new()));

            if local_cache.is_empty() {
                break;
            }

            let err = rocket::serde::json::Json::from(local_cache.as_ref().clone());
            return Box::pin(async move {
                rocket::response::Response::build_from(err.respond_to(req).unwrap())
                .status(rocket::http::Status::BadRequest)
                .header(rocket::http::ContentType::JSON)
                .ok()
            })
        },
        _ => ()
    } break; }

    let message = if status.reason().is_some() {
        String::from(status.reason().unwrap())
    } else {
        format!("Unknown error: {}", status.code)
    };

    let err = rocket::serde::json::Json::from(DefaultError::new(message));

    return Box::pin(async move {
        rocket::response::Response::build_from(err.respond_to(req).unwrap())
        .status(status)
        .header(rocket::http::ContentType::JSON)
        .ok()
    })
}
