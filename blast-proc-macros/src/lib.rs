extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

use heck::ToSnakeCase;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, LitInt};

/// Programatically derive the `BlastState` trait for a struct.
#[proc_macro]
pub fn make_stateful(input: TokenStream) -> TokenStream {
    struct Params {
        guts_ident: proc_macro::Ident,
        pure_ident: proc_macro::Ident,
        yield_ident: proc_macro::Ident,
    }

    fn find_ident(tokens_iter: &mut impl Iterator<Item = TokenTree>, target: &str) -> Option<proc_macro::Ident> {
        while let Some(token) = tokens_iter.next() {
            if let TokenTree::Ident(ident) = token {
                if ident.to_string() == target {
                    if let Some(TokenTree::Ident(ident)) = tokens_iter.next() {
                        return Some(ident);
                    }
                } 
            }
        }
        return None;
    }

    impl Params {
        pub(crate) fn idents(&self) -> (syn::Ident, syn::Ident, syn::Ident) {
            // turn the proc_macro::Ident into syn::Ident
            let syn_guts: syn::Ident =
                syn::Ident::new(&self.guts_ident.to_string(), self.guts_ident.span().into());
            let syn_pure: syn::Ident =
                syn::Ident::new(&self.pure_ident.to_string(), self.pure_ident.span().into());
            let syn_yield: syn::Ident = syn::Ident::new(
                &self.yield_ident.to_string(),
                self.yield_ident.span().into(),
            );

            return (syn_pure, syn_guts, syn_yield);
        }

        fn from_input(input: TokenStream) -> Params {
            // Convert the TokenStream into a Vec<TokenTree> for easier manipulation
            let tokens: Vec<_> = input.into_iter().collect();

            // Ensure that the number of tokens is correct
            if tokens.len() < 8 {
                panic!("Expected GUTS, PURE, and YIELD");
            }

            // Create an iterator over the tokens
            let mut tokens_iter = tokens.into_iter().peekable();

            // Find the `GUTS` identifier
            let guts_ident = find_ident(&mut tokens_iter, "GUTS");

            // Find the `PURE` identifier
            let pure_ident = find_ident(&mut tokens_iter, "PURE");

            // Find the `YIELD` identifier
            let yield_ident = find_ident(&mut tokens_iter, "YIELD");

            // Ensure that all parameters are present
            if guts_ident.is_none() || pure_ident.is_none() || yield_ident.is_none() {
                panic!("Expected GUTS, PURE, and YIELD");
            }

            Params {
                guts_ident: guts_ident.unwrap(),
                pure_ident: pure_ident.unwrap(),
                yield_ident: yield_ident.unwrap(),
            }
        }
    }

    let params = Params::from_input(input);
    let (pure_ident, guts_ident, yield_ident) = params.idents();
    let output = quote! {
        impl BlastState for #pure_ident {
            type Inner = Arc<#guts_ident>;
        }
        impl #yield_ident for #pure_ident {}
    };

    return output.into()
}

/// Convert a CamelCase identifier into snake_case.
#[proc_macro]
pub fn snake_trap(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    let ident_str = ident.to_string();
    let snake_case_ident = ident_str.to_snake_case();
    let snake_case_catcher = format!("{}", snake_case_ident);
    let new_ident = syn::Ident::new(&snake_case_catcher, ident.span());
    quote!(#new_ident).into()
}

/// Derive the rocket `Responder` trait for an enum of application errors.
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

/// Implement a governable rocket limiter for a custom limiter struct.
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
