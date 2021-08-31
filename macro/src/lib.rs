#![recursion_limit="256"]

extern crate proc_macro;

use quote::quote;

///# Validated Json Input
///Structs that derive [`JsonBody`] can be used as Endpoint Input.
///The data is read from the body as Json and validated via [`Validator`].
///# Requirements
///The struct has to implement [`serde::Deserialize`] and [`validator::Validate`]
///# Example
///```
///#[derive(serde::Deserialize, validator::Validate, rocketjson::JsonBody)]
///pub struct TestRequest {
///   #[validate(length(min = 1))]
///   username: String 
///}
///
///#[post("/register", data="<data>")]
///pub fn register(data: RegisterRequest) {
/// //data is validated from json body
///}
///```
///[`Validator`]: https://github.com/Keats/validator
///[`validator::Validate`]: https://docs.rs/validator/0.14.0/validator/trait.Validate.html
///[`serde::Deserialize`]: https://docs.serde.rs/serde/trait.Deserialize.html

#[proc_macro_derive(JsonBody)]
pub fn writable_template_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    
    let mut from_data_generics = generics.clone();
    if from_data_generics.params.is_empty() {
        from_data_generics.params.push(syn::parse_quote!('r))
    }

    //TODO:
    //reimplement Error handling once
    //https://github.com/SergioBenitez/Rocket/issues/749
    //is out.

    let expanded = quote! {
        #[rocket::async_trait]
        impl #from_data_generics rocket::data::FromData #from_data_generics for #name #generics where Self: validator::Validate {
            type Error = rocketjson::error::JsonBodyError;

            async fn from_data(req: &'r rocket::request::Request<'_>, data: rocket::data::Data<'r>) -> rocket::data::Outcome<'r, Self> {
                use validator::Validate;

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
                    req.local_cache(|| std::sync::Arc::new(errors.clone()));
                    return rocket::outcome::Outcome::Failure((rocket::http::Status::BadRequest, Self::Error::ValidationError(errors)))
                }

                rocket::outcome::Outcome::Success(obj)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
