extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Lit, Meta, NestedMeta};

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
