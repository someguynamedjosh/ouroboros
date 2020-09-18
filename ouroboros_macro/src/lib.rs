use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Group, Span, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parenthesized, Expr, Field, Fields, FieldsUnnamed, GenericParam, Generics, Ident, ItemStruct,
    Lifetime, LifetimeDef, Token, Type, TypeParam, TypeParamBound, Visibility,
};

struct StructFieldInfo {
    name: Ident,
    typ: Type,
    /// True if no other fields in the struct can refer to this field. This allows its type to be
    /// more flexible.
    is_tail: bool,
}

fn replace_this_with_static(input: TokenStream2) -> TokenStream2 {
    input
        .into_iter()
        .map(|token| match &token {
            TokenTree::Ident(ident) => {
                if ident.to_string() == "this" {
                    TokenTree::Ident(format_ident!("static"))
                } else {
                    token
                }
            }
            TokenTree::Group(group) => TokenTree::Group(Group::new(
                group.delimiter(),
                replace_this_with_static(group.stream()),
            )),
            _ => token,
        })
        .collect()
}

#[proc_macro_attribute]
pub fn self_referencing(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_struct_def: ItemStruct = syn::parse_macro_input!(item);

    // Actual data struct generation & metadata gathering.
    let mut actual_struct_def = original_struct_def.clone();
    actual_struct_def.vis = Visibility::Inherited;
    let mut field_info = Vec::new();
    match &mut actual_struct_def.fields {
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
    match &mut actual_struct_def.fields {
        Fields::Named(fields) => {
            let reversed = fields.named.iter().rev().cloned().collect();
            fields.named = reversed;
        }
        Fields::Unnamed(_fields) => unimplemented!("Tuple structs are not supported yet."),
        Fields::Unit => panic!("Unit structs cannot be self-referential."),
    }
    // Finally, replace the fake 'this lifetime with 'static.
    let actual_struct_def = replace_this_with_static(quote! { #actual_struct_def });

    // Generic stuff
    let generic_producers = original_struct_def.generics.clone();
    let generic_consumers = {
        let mut arguments = Vec::new();
        for generic in original_struct_def.generics.params.clone() {
            match generic {
                GenericParam::Type(typ) => {
                    let ident = &typ.ident;
                    arguments.push(quote! { #ident });
                }
                GenericParam::Lifetime(lt) => {
                    let lifetime = &lt.lifetime;
                    arguments.push(quote! { #lifetime });
                },
                GenericParam::Const(_) => unimplemented!(),
            }
        }
        arguments
    };

    // Constructor generation.
    let struct_name = &original_struct_def.ident;
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
                Self{ #(#field_names),* }
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
                    user(&self. #field_name)
                }
            });
        } else {
            users.push(quote! {
                pub fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> FnOnce(&'outer_borrow <#field_type as ::std::ops::Deref>::Target) -> ReturnType,
                ) -> ReturnType {
                    user(&*self. #field_name)
                }
            });
        }
    }

    // use_all_fields generation
    let (all_fields_struct, use_all_fields_fn) = {
        let mut fields = Vec::new();
        let mut field_assignments = Vec::new();
        // I don't think the reverse is necessary but it does make the expanded code more uniform.
        for field in field_info.iter().rev() {
            let field_name = &field.name;
            let field_type = &field.typ;
            if field.is_tail {
                fields.push(quote! { pub #field_name: &'outer_borrow #field_type });
                field_assignments.push(quote! { #field_name: &self.#field_name });
            } else {
                fields.push(quote! { pub #field_name: &'outer_borrow <#field_type as ::std::ops::Deref>::Target });
                field_assignments.push(quote! { #field_name: &*self.#field_name });
            }
        }
        let all_fields_struct = if generic_producers.params.len() == 0 {
            quote! {
                struct BorrowedFields<'outer_borrow, 'this> { #(#fields),* }
            }
        } else {
            let mut new_generic_producers = generic_producers.clone();
            new_generic_producers
                .params
                .insert(0, syn::parse_quote! { 'this });
            new_generic_producers
                .params
                .insert(0, syn::parse_quote! { 'outer_borrow });
            quote! {
                struct BorrowedFields #new_generic_producers { #(#fields),* }
            }
        };
        let borrowed_fields_type = if generic_consumers.len() == 0 {
            quote! { BorrowedFields<'outer_borrow, 'this> }
        } else {
            let mut new_generic_consumers = generic_consumers.clone();
            new_generic_consumers.insert(0, quote! { 'this });
            new_generic_consumers.insert(0, quote! { 'outer_borrow });
            quote! { BorrowedFields <#(#new_generic_consumers),*> }
        };
        let use_all_fields_fn = quote! {
            pub fn use_all_fields <'outer_borrow, ReturnType>(
                &'outer_borrow self,
                user: impl for <'this> FnOnce(#borrowed_fields_type) -> ReturnType
            ) -> ReturnType {
                user(BorrowedFields {
                    #(#field_assignments),*
                })
            }
        };
        (all_fields_struct, use_all_fields_fn)
    };

    let final_data = TokenStream::from(quote! {
        #actual_struct_def
        pub #all_fields_struct
        impl #generic_producers #struct_name <#(#generic_consumers),*> {
            #constructor_def
            #(#users)*
            #use_all_fields_fn
        }
    });
    // eprintln!("{:#?}", final_data);
    final_data
}
