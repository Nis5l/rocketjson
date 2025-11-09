#![recursion_limit="256"]

extern crate proc_macro;

use quote::quote;
use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate))]
struct JsonBodyOpts {
    #[darling(default)]
    context: Option<syn::Path>,
}

///# Validated Json Input
///Structs that derive [`JsonBody`] can be used as Endpoint Input.
///The data is read from the body as Json and validated via [`Validator`].
///If arguments are passed to cusom Validators
///`#[validate(custom(function="validate_password", arg="&'v_a Config"))]`
///they are read from [state](https://docs.rs/rocket/0.5.0-rc.1/rocket/struct.Rocket.html#method.state)
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
#[proc_macro_derive(JsonBody, attributes(validate))]
pub fn derive_jsonbody(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let opts = JsonBodyOpts::from_derive_input(&ast).unwrap();

    let (ctx_line, validate_call) = if let Some(ctx) = opts.context {
        (
            // Generate fetching of context from Rocket state
            quote! {
                let ctx: & #ctx = req.rocket().state::<#ctx>().expect("context state not found");
            },
            // Call validate_with_args if context exists
            quote! { obj.validate_with_args(ctx) }
        )
    } else {
        (
            // No context line
            quote! {},
            // Call normal validate
            quote! { obj.validate() }
        )
    };

    let gen = quote! {
        #[rocket::async_trait]
        impl<'r> rocket::data::FromData<'r> for #name {
            type Error = ();

            async fn from_data(
                req: &'r rocket::Request<'_>,
                data: rocket::data::Data<'r>
            ) -> rocket::data::Outcome<'r, Self> {
                //NOTE: forward if not JSON
                if req.content_type() != Some(&rocket::http::ContentType::new("application", "json")) {
                    return rocket::data::Outcome::Forward((data, rocket::http::Status::Continue));
                }

                //NOTE: parse JSON
                let json_outcome = rocket::serde::json::Json::<Self>::from_data(req, data).await;

                let obj = match json_outcome {
                    rocket::data::Outcome::Success(json) => json.into_inner(),
                    rocket::data::Outcome::Error(_) => {
                        req.local_cache(|| rocketjson::error::JsonBodyError::JsonValidationError);
                        return rocket::data::Outcome::Error((rocket::http::Status::BadRequest, ()));
                    },
                    rocket::data::Outcome::Forward(f) => return rocket::data::Outcome::Forward(f),
                };

                //NOTE: validate
                #ctx_line

                if let Err(errors) = #validate_call {
                    req.local_cache(|| rocketjson::error::JsonBodyError::ValidationError(errors));
                    return rocket::data::Outcome::Error((rocket::http::Status::BadRequest, ()));
                }

                rocket::data::Outcome::Success(obj)
            }
        }
    };

    gen.into()
}
