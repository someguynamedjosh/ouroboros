use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::info_structures::{ArgType, StructInfo};

pub fn generate_borrowchk_summoner(info: &StructInfo) -> Result<TokenStream, Error> {
    let mut code: Vec<TokenStream> = Vec::new();
    let mut params: Vec<TokenStream> = Vec::new();
    let mut value_consumers: Vec<TokenStream> = Vec::new();
    let mut template_consumers: Vec<TokenStream> = Vec::new();
    for field in &info.fields {
        let field_name = &field.name;

        let arg_type = field.make_constructor_arg_type(&info, false)?;
        if let ArgType::Plain(plain_type) = arg_type {
            // No fancy builder function, we can just move the value directly into the struct.
            params.push(quote! { #field_name: #plain_type });
        } else if let ArgType::TraitBound(bound_type) = arg_type {
            // Trait bounds are much trickier. We need a special syntax to accept them in the
            // contructor, and generic parameters need to be added to the builder struct to make
            // it work.
            let builder_name = field.builder_name();
            params.push(quote! { #builder_name : impl #bound_type });
            let mut builder_args = Vec::new();
            for (_, borrow) in field.borrows.iter().enumerate() {
                let borrowed_name = &info.fields[borrow.index].name;
                if borrow.mutable {
                    builder_args.push(quote! { &mut #borrowed_name });
                } else {
                    builder_args.push(quote! { &#borrowed_name });
                }
            }
            code.push(quote! { let #field_name = #builder_name (#(#builder_args),*); });
        }
        if field.is_borrowed() {
            let boxed = quote! { ::ouroboros::macro_help::aliasable_boxed(#field_name) };
            code.push(quote! { let mut #field_name = #boxed; });
        };
        if !field.is_mutably_borrowed() {
            value_consumers.push(quote! { #field_name: &#field_name });
        }
    }
    for (_ty, ident) in info.generic_consumers() {
        template_consumers.push(quote! { #ident: ::core::marker::PhantomData });
    }
    let generic_params = info.generic_params();
    Ok(quote! {
        fn check_if_okay_according_to_borrow_checker<#generic_params>(
            #(#params,)*
        ) {
            #(#code;)*
            BorrowedFields {
                #(#value_consumers,)*
                #(#template_consumers,)*
            };
        }
    })
}
