#![recursion_limit="256"]

extern crate proc_macro;

use quote::quote;

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

#[proc_macro_derive(JsonBody)]
pub fn writable_template_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);

    input.generics.make_where_clause();

    let name = &input.ident;
    let mut where_clause = input.generics.where_clause.clone().unwrap();

    let generics = input.generics.clone();

    let mut lgenerics = generics.clone();
    if lgenerics.params.is_empty() {
        lgenerics.params.push(syn::parse_quote!('r));
    } else {
        let mut lgenericsnew = syn::Generics::default();
        lgenericsnew.params.push(syn::parse_quote!('r));
        let mut iter = lgenerics.params.iter();
        iter.next();
        for g in iter {
            lgenericsnew.params.push(g.clone());
        }
        lgenerics = lgenericsnew;
    }

    let mut from_data_generics = lgenerics.clone(); 
    while from_data_generics.params.len() > 1 {
        from_data_generics.params.pop();
    }

    let mut validate_args_generics = from_data_generics.clone(); 

    //TODO:
    //reimplement Error handling once
    //https://github.com/SergioBenitez/Rocket/issues/749
    //is out.

    let mut streams = Vec::new();

    let mut args_generics = Vec::new();
    let mut rocket_states: Vec<syn::Stmt> = Vec::new();
    let mut rocket_states_variables: Vec<String> = Vec::new();

    for i in 0..10 {
        if i != 0 {
            let generic_str = format!("TRJ{}", i);
            let mut generic = syn::Generics::default();
            generic.params.push(syn::parse_str(&generic_str).unwrap());

            rocket_states.push(syn::parse_str(&format!("let p{} = req.rocket().state::<{}>().expect(\"type not found in state\");", i, generic_str)[..]).unwrap());
            rocket_states_variables.push(format!("p{}", i));

            args_generics.push(generic_str.clone());

            lgenerics.params.push(syn::parse_str(&format!("{}: 'static", generic_str)[..]).unwrap());

        }

        validate_args_generics.params.push(
            syn::parse_str(&format!("Args=({})", args_generics.iter().map(|s| { format!("&'r {}", s) }).collect::<Vec<String>>().join(","))).unwrap()
        );

        where_clause.predicates = syn::punctuated::Punctuated::new();
        where_clause.predicates.push(syn::parse_quote!(Self: validator::ValidateArgs#validate_args_generics));
        for g in args_generics.iter() {
            where_clause.predicates.push(syn::parse_str(&format!("{}: Sync + std::marker::Send", g)[..]).unwrap());
        }

        let rocket_states_folded = rocket_states.iter().fold(quote! {}, |acc, new| quote! {#acc #new});
        let rocket_states_variables: syn::Type = syn::parse_str(&format!("({})", rocket_states_variables.join(","))[..]).unwrap();

        streams.push(quote! {
            #[rocket::async_trait]
            //impl #from_data_generics rocket::data::FromData #from_data_generics for #name #generics where Self: validator::Validate {
            impl #lgenerics rocket::data::FromData #from_data_generics for #name #generics
                #where_clause
                {

                type Error = ();

                async fn from_data(req: &'r rocket::request::Request<'_>, data: rocket::data::Data<'r>) -> rocket::data::Outcome<'r, Self> {
                    use validator::ValidateArgs;

                    if req.content_type() != Some(&rocket::http::ContentType::new("application", "json")) {
                        return rocket::outcome::Outcome::Forward(data);
                    }

                    let json_opt = rocket::serde::json::Json::<Self>::from_data(req, data).await;

                    //TODO:
                    //let opt =
                    match json_opt {
                        rocket::outcome::Outcome::Failure(_) => {
                            req.local_cache(|| rocketjson::error::JsonBodyError::JsonValidationError);
                            return rocket::outcome::Outcome::Failure((rocket::http::Status::BadRequest, ()));
                        },
                        rocket::outcome::Outcome::Forward(forward) => {
                            return rocket::outcome::Outcome::Forward(forward);
                        },
                        rocket::outcome::Outcome::Success(_) => ()
                    }

                    let obj = json_opt.unwrap().0;

                    #rocket_states_folded
                    let errors_ok = obj.validate_args(#rocket_states_variables) ;

                    if let Err(errors) = errors_ok {
                        req.local_cache(move || rocketjson::error::JsonBodyError::ValidationError(errors));
                        return rocket::outcome::Outcome::Failure((rocket::http::Status::BadRequest, ()))
                    }

                    rocket::outcome::Outcome::Success(obj)
                }
            }
        });

        validate_args_generics.params.pop();
    }

    let folded = streams.iter().fold(quote! {}, |acc, new| quote! {#acc #new});
    
    let stream = quote! {
        #folded 
    };
    
    proc_macro::TokenStream::from(stream)
}
