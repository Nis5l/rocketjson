# rocketjson
[![Current Crates.io Version](https://img.shields.io/crates/v/rocketjson.svg)](https://crates.io/crates/rocketjson)

Crate for working with Json and [Rocket](https://github.com/SergioBenitez/Rocket). \
Ultimately the goal is to have [validated](https://github.com/Keats/validator) Structs enter and leave the endpoint as Json
while having everything happen in the background.

# Documentation
Documentation is on [docs.rs](https://docs.rs/rocketjson)

# Example
```
#[macro_use] extern crate rocket;

#[derive(serde::Deserialize, validator::Validate, rocketjson::JsonBody)]
pub struct RegisterRequest {
   #[validate(length(min = 1))]
   username: String 
}

#[derive(serde::Serialize)]
pub struct RegisterResponse {
   message: String
}

#[post("/register", data="<data>")]
pub fn register(data: RegisterRequest) -> rocketjson::ApiResponse<RegisterResponse> {
   rocketjson::ApiResponse::new(rocket::http::Status::Ok, RegisterResponse { message: format!("Welcome {}", data.username) })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![register]).
        register("/", vec![rocketjson::error::get_catcher()])
}
```
- Input 
```
{
    "username": "testuser"
}
```
- Output 200 OK
```
{
    "message": "Welcome testuser"
}
```
- Input
```
{
    "username": ""
}
```
- Output 400 Bad Request
```
{
    "username": [
        {
            "code": "length",
            "message": null,
            "params": {
                "value": "",
                "min": 1
            }
        }
    ]
}
```
# License
The license can be chosen to be either of the following:
- [MIT](https://opensource.org/licenses/MIT)
- [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) 
