extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
