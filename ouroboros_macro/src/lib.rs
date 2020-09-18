use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Span, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Expr, Ident, Token};
use syn::{Field, Fields, FieldsUnnamed, GenericParam, ItemStruct, Type, Visibility};

struct StructFieldInfo {
    name: Ident,
    typ: Type,
    /// True if no other fields in the struct can refer to this field. This allows its type to be
    /// more flexible.
    is_tail: bool,
}

#[proc_macro_attribute]
pub fn self_referencing(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_struct_def: ItemStruct = syn::parse_macro_input!(item);

    // Internal data struct generation & metadata gathering.
    let mut internal_struct_def = original_struct_def.clone();
    internal_struct_def.vis = Visibility::Inherited;
    internal_struct_def.ident = format_ident!("{}OuroborosInternalData", internal_struct_def.ident);
    // Add the 'this lifetime (which will just be 'static.)
    internal_struct_def
        .generics
        .params
        .insert(0, GenericParam::Lifetime(syn::parse_quote! { 'this }));
    let mut field_info = Vec::new();
    match &mut internal_struct_def.fields {
        Fields::Named(fields) => {
            for field in &mut fields.named {
                let mut is_tail = false;
                for (index, attr) in field.attrs.iter().enumerate() {
                    let path = &attr.path;
                    if path.leading_colon.is_some() {
                        continue;
                    }
                    if path.segments.len() != 1 {
                        continue;
                    }
                    if path.segments.first().unwrap().ident.to_string() == "tail" {
                        is_tail = true;
                        field.attrs.remove(index);
                        break;
                    }
                }
                field_info.push(StructFieldInfo {
                    name: field.ident.clone().expect("Named field has no name."),
                    typ: field.ty.clone(),
                    is_tail,
                });
            }
        }
        Fields::Unnamed(_fields) => unimplemented!("Tuple structs are not supported yet."),
        Fields::Unit => panic!("Unit structs cannot be self-referential."),
    }
    if field_info.len() < 2 {
        panic!("Self-referencing structs must have at least 2 members.");
    }
    // Last element is always a tail, by definition. Nothing can possibly refer to it.
    let last_index = field_info.len() - 1;
    field_info[last_index].is_tail = true;
    let mut has_non_tail = false;
    for field in &field_info {
        if !field.is_tail {
            has_non_tail = true;
            break;
        }
    }
    if !has_non_tail {
        panic!("Self-referencing struct cannot be made entirely of tail fields.");
    }
    // Reverse the order of all members. We ensure that items in the struct are only dependent
    // on references to items above them. Rust drops items in a struct in forward declaration order.
    // This would cause parents being dropped before children, necessitating the reversal.
    match &mut internal_struct_def.fields {
        Fields::Named(fields) => {
            let reversed = fields.named.iter().rev().cloned().collect();
            fields.named = reversed;
        }
        Fields::Unnamed(_fields) => unimplemented!("Tuple structs are not supported yet."),
        Fields::Unit => panic!("Unit structs cannot be self-referential."),
    }

    // Wrapper struct generation.
    let wrapper_struct_def = {
        let attrs = &original_struct_def.attrs;
        let visibility = &original_struct_def.vis;
        let ident = &original_struct_def.ident;
        let generics = &original_struct_def.generics;
        let internal_ident = &internal_struct_def.ident;
        let mut internal_generics = generics.clone();
        if internal_generics.params.len() > 0 {
            internal_generics
                .params
                .insert(0, GenericParam::Lifetime(syn::parse_quote! {'static}));
        } else {
            internal_generics = syn::parse_quote! { <'static> };
        }
        quote! {
            #(#attrs)*
            #visibility struct #ident #generics { evil_secret_data_bad_bad_not_good__: #internal_ident #internal_generics }
        }
    };

    // Constructor generation.
    let generics = &original_struct_def.generics;
    let wrapper_struct_name = &original_struct_def.ident;
    let internal_struct_name = &internal_struct_def.ident;
    let constructor_def: TokenStream2 = {
        let mut params: Vec<TokenStream2> = Vec::new();
        let mut code: Vec<TokenStream2> = Vec::new();
        let mut field_builder_params: Vec<TokenStream2> = Vec::new();
        let mut field_builder_args: Vec<TokenStream2> = Vec::new();
        for field in &field_info {
            let field_name = &field.name;
            let field_type = &field.typ;
            // If we haven't gotten past fields that can be referenced yet, then we can just accept
            // the value of this field as a straight argument with no builder necessary.
            if field_builder_params.len() == 0 {
                params.push(quote!( #field_name : #field_type ));
            } else {
                let builder_name = format_ident!("{}_builder", field_name);
                params.push(quote! { #builder_name : impl for<'this> FnOnce(#(#field_builder_params),*) -> #field_type });
                code.push(quote! { let #field_name = #builder_name (#(#field_builder_args),*); })
            }
            if !field.is_tail {
                let ref_name = format_ident!("{}_illegal_static_reference", field_name);
                code.push(quote! {
                    let #ref_name = unsafe { ::ouroboros::macro_help::stable_deref_and_strip_lifetime(&#field_name) };
                });
                field_builder_params
                    .push(quote! { &'this <#field_type as ::std::ops::Deref>::Target });
                field_builder_args.push(quote! { #ref_name });
            }
        }
        let mut field_names = Vec::new();
        for field in &field_info {
            field_names.push(field.name.clone());
        }
        quote! {
            pub fn new(#(#params),*) -> Self {
                #(#code)*
                Self{ evil_secret_data_bad_bad_not_good__: #internal_struct_name { #(#field_names),* }}
            }
        }
    };

    // fn use_* generation
    let mut users = Vec::new();
    for field in &field_info {
        let field_name = &field.name;
        let field_type = &field.typ;
        let user_name = format_ident!("use_{}", &field.name);
        // If the field is not a tail, we need to serve up the same kind of reference that other
        // members in the struct may have borrowed to ensure safety.
        if field.is_tail {
            users.push(quote! {
                pub fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> FnOnce(&'outer_borrow #field_type) -> ReturnType,
                ) -> ReturnType {
                    user(&self.evil_secret_data_bad_bad_not_good__. #field_name)
                }
            });
        } else {
            users.push(quote! {
                pub fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> FnOnce(&'outer_borrow <#field_type as ::std::ops::Deref>::Target) -> ReturnType,
                ) -> ReturnType {
                    user(&*self.evil_secret_data_bad_bad_not_good__. #field_name)
                }
            });
        }
    }

    let final_data = TokenStream::from(quote! {
        #internal_struct_def
        #wrapper_struct_def
        impl #generics #wrapper_struct_name #generics {
            #constructor_def
            #(#users)*
        }
    });
    // eprintln!("{:#?}", final_data);
    final_data
}
