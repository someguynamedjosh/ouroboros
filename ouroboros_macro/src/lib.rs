use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Expr, Ident, Token};

#[proc_macro_attribute]
pub fn self_referencing(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        fn test() {
            println!("Hello from macro!");
        }
    })
}
