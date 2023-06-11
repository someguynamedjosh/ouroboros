use crate::{
    info_structures::StructInfo,
    utils::{self, replace_this_with_lifetime},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

pub fn create_drop_impl(info: &StructInfo) -> Result<TokenStream, Error> {
    let ident = &info.ident;
    let internal_ident = &info.internal_ident;
    let generics = &info.generics;
    let generic_args = info.generic_arguments();

    let mut where_clause = quote! {};
    if let Some(clause) = &generics.where_clause {
        where_clause = quote! { #clause };
    }
    Ok(quote! {
        impl #generics ::core::ops::Drop for #ident<#(#generic_args,)*> #where_clause {
            fn drop(&mut self) {
                unsafe {
                    ::core::ptr::drop_in_place(::core::mem::transmute::<_, *mut #internal_ident <#(#generic_args,)*>>(self));
                }
            }
        }
    })
}
