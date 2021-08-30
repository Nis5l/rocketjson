extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(JsonBody)]
pub fn writable_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        #[rocket::async_trait]
        impl<'r> rocket::data::FromData<'r> for #name<'r> {
            type Error = JsonBodyError;

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

                let errors = obj.validate();
                if errors.is_err() {
                    return rocket::outcome::Outcome::Failure((rocket::http::Status::BadRequest, Self::Error::ValidationError))
                }

                rocket::outcome::Outcome::Success(obj)
            }
        }
    };

    TokenStream::from(expanded)
}
