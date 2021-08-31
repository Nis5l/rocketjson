#![recursion_limit="256"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(JsonBody)]
pub fn writable_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    //TODO:
    //reimplement Error handling once
    //https://github.com/SergioBenitez/Rocket/issues/749
    //is out.

    let expanded = quote! {
        #[rocket::async_trait]
        impl<'r> rocket::data::FromData<'r> for #name<'r> where Self: validator::Validate {
            type Error = rocketjson::error::JsonBodyError;

            async fn from_data(req: &'r rocket::request::Request<'_>, data: rocket::data::Data<'r>) -> rocket::data::Outcome<'r, Self> {
                if req.content_type() != Some(&rocket::http::ContentType::new("application", "json")) {
                    return rocket::outcome::Outcome::Forward(data);
                }

                let json_opt = rocket::serde::json::Json::<Self>::from_data(req, data).await;

                match json_opt {
                    rocket::outcome::Outcome::Failure(_) => {
                        return rocket::outcome::Outcome::Failure((rocket::http::Status::BadRequest, Self::Error::JsonValidationError));
                    },
                    rocket::outcome::Outcome::Forward(forward) => {
                        return rocket::outcome::Outcome::Forward(forward);
                    },
                    rocket::outcome::Outcome::Success(_) => ()
                }

                let obj = json_opt.unwrap().0;

                let errors_ok = obj.validate();
                if let Err(errors) = errors_ok {
                    req.local_cache(|| std::sync::Arc::new(errors.clone()) );
                    return rocket::outcome::Outcome::Failure((rocket::http::Status::BadRequest, Self::Error::ValidationError(errors)))
                }

                rocket::outcome::Outcome::Success(obj)
            }
        }
    };

    TokenStream::from(expanded)
}
