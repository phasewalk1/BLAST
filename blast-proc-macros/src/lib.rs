extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, LitInt};

#[proc_macro]
pub fn make_stateful(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let args = input.data;

    let (guts_ident, pure_ident): (Ident, Ident) = match args {
        syn::Data::Struct(s) => {
            let guts_ident = s
                .fields
                .iter()
                .find(|f| f.ident.as_ref().unwrap() == "GUTS")
                .unwrap()
                .ty
                .clone();
            let pure_ident = s
                .fields
                .iter()
                .find(|f| f.ident.as_ref().unwrap() == "PURE")
                .unwrap()
                .ty
                .clone();
            // turn tys into idents
            let guts_ident = match guts_ident {
                syn::Type::Path(p) => p.path.segments[0].ident.clone(),
                _ => panic!("Invalid input"),
            };
            let pure_ident = match pure_ident {
                syn::Type::Path(p) => p.path.segments[0].ident.clone(),
                _ => panic!("Invalid input"),
            };
            (guts_ident, pure_ident)
        }
        _ => panic!("Invalid input"),
    };

    let output: proc_macro2::TokenStream = quote! {
        impl BlastState for #pure_ident {
            type Inner = Arc<#guts_ident>;
        }
        impl State for #pure_ident {}
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn snake_case_catcher(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    let ident_str = ident.to_string();
    let snake_case_ident = ident_str.to_snake_case();
    let snake_case_catcher = format!("{}_catcher", snake_case_ident);
    let new_ident = syn::Ident::new(&snake_case_catcher, ident.span());
    quote!(#new_ident).into()
}

/// Derive the `Responder` trait for an enum of application errors.
#[proc_macro_derive(MakeResponder)]
pub fn derive_to_responder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    match &input.data {
        syn::Data::Enum(_) => {}
        _ => panic!("This macro only works with Enums"),
    };

    let expanded = quote! {
        impl<'r> rocket::response::Responder<'r, 'static> for #name {
            fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                let status: rocket::http::Status = self.into();
                return status.respond_to(req);
            }
        }

    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Limiter, attributes(rate))]
pub fn limiter_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name and rate value
    let struct_name = &input.ident;
    let rate_attr = match &input.data {
        Data::Struct(_) => {
            let fields = match &input.data {
                Data::Struct(data) => &data.fields,
                _ => panic!("Limiter macro can only be used on structs"),
            };
            let rate_lit = match fields {
                syn::Fields::Named(fields) => {
                    let rate_field = fields.named.iter().find(|field| field.ident.is_some());
                    match rate_field {
                        Some(field) => {
                            let rate_attr =
                                field.attrs.iter().find(|attr| attr.path.is_ident("rate"));
                            match rate_attr {
                                Some(attr) => {
                                    let rate_lit =
                                        attr.parse_args::<LitInt>().expect("Invalid rate value");
                                    rate_lit
                                }
                                None => panic!("Missing rate attribute"),
                            }
                        }
                        None => panic!("Missing rate attribute"),
                    }
                }
                _ => panic!("Missing rate attribute"),
            };
            rate_lit
        }
        _ => panic!("Limiter macro can only be used on structs"),
    };

    // Generate the rate limiter implementation
    let gen = quote! {
        impl<'r> RocketGovernable<'r> for #struct_name {
            fn quota(_method: Method, _route_name: &str) -> Quota {
                Quota::per_second(Self::nonzero(#rate_attr))
            }
        }
    };

    // Return the generated implementation as a TokenStream
    TokenStream::from(gen)
}
