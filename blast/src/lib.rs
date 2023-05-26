extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Lit, Meta, NestedMeta};

/// Derive the `Into<rocket::http::Status>` trait for an enum.
#[proc_macro_derive(ToStatus, attributes(give))]
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
        impl Into<rocket::http::Status> for #name {
            fn into(self) -> rocket::http::Status {
                match self {
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
