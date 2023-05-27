extern crate proc_macro;
extern crate quote;
extern crate rocket;
extern crate syn;

use heck::ToSnakeCase;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, LitInt, Token};

#[proc_macro]
pub fn make_stateful(input: TokenStream) -> TokenStream {
    struct Params {
        guts_ident: proc_macro::Ident,
        pure_ident: proc_macro::Ident,
        yield_ident: proc_macro::Ident,
    }

    fn has(target: &str, input: Option<TokenTree>) -> bool {
        let _has = match input {
            Some(TokenTree::Ident(ident)) => match ident {
                ident if ident.to_string() == target => true,
                _ => false,
            },
            _ => false,
        };
        return _has;
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
        // Parse the input tokens into a syntax tree to extract all 3 parameters
        fn from_input(input: TokenStream) -> Params {
            // Iterate over the TokenStream
            let mut token_iter = input.into_iter();
            // The first token should be IDENT 'GUTS'
            let has_guts = has("GUTS", token_iter.next());
            let gut_ident = match has_guts {
                true => match token_iter.next() {
                    Some(TokenTree::Ident(ident)) => ident,
                    _ => panic!("Expected IDENT for GUTS"),
                },
                false => panic!("Expected IDENT 'GUTS'"),
            };
            // tick comma
            let _ = token_iter.next();
            let has_pure = has("PURE", token_iter.next());
            let pure_ident = match has_pure {
                true => match token_iter.next() {
                    Some(TokenTree::Ident(ident)) => ident,
                    _ => panic!("Expected IDENT for PURE"),
                },
                false => panic!("Expected IDENT 'PURE'"),
            };
            // tick comma
            let _ = token_iter.next();
            let has_yield = has("YIELD", token_iter.next());
            let yield_ident = match has_yield {
                true => match token_iter.next() {
                    Some(TokenTree::Ident(ident)) => ident,
                    _ => panic!("Expected IDENT for YIELD"),
                },
                false => panic!("Expected IDENT 'YIELD'"),
            };

            return Params {
                guts_ident: gut_ident,
                pure_ident: pure_ident,
                yield_ident: yield_ident,
            };
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

    // Return the generated output as a token stream
    output.into()
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
