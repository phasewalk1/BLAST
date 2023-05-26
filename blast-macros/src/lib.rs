extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Lit, Meta, NestedMeta};

/// Derive the `Responder` trait for an enum of application errors.
#[proc_macro_derive(Responder)]
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

/// Derive the `From<YourEnum> for rocket::http::Status` trait for an enum.
#[proc_macro_derive(Codes, attributes(give))]
pub fn derive_to_status(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let status_codes = match &input.data {
        syn::Data::Enum(e) => e.variants.iter().map(|v| {
            let status_code = v
                .attrs
                .iter()
                .filter_map(get_status_code)
                .next()
                .expect("Status code attribute is missing!");
            let variant = &v.ident;
            quote! {
                #name::#variant => rocket::http::Status::from_code(#status_code).unwrap(),
            }
        }),
        _ => panic!("This macro only works with Enums"),
    };

    let expanded = quote! {
        impl From<#name> for rocket::http::Status {
            fn from(error: #name) -> rocket::http::Status {
                match error {
                    #(#status_codes)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_status_code(attr: &Attribute) -> Option<u16> {
    if attr.path.is_ident("give") {
        match attr.parse_meta() {
            Ok(Meta::List(meta)) => {
                if let Some(NestedMeta::Lit(Lit::Int(lit))) = meta.nested.first() {
                    return Some(lit.base10_parse().unwrap());
                }
            }
            _ => {}
        }
    }

    None
}
