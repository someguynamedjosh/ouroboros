use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Group, Span, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parenthesized, Attribute, Expr, Field, Fields, FieldsUnnamed, GenericParam, Generics, Ident,
    ItemStruct, Lifetime, LifetimeDef, Token, Type, TypeParam, TypeParamBound, Visibility,
};

#[derive(Clone, Copy, PartialEq)]
enum FieldType {
    /// Not borrowed by other parts of the struct.
    Tail,
    /// Immutably borrowed by at least one other member.
    Borrowed,
    /// Mutably borrowed by one other member.
    BorrowedMut,
}

impl FieldType {
    fn is_tail(self) -> bool {
        self == Self::Tail
    }
}

struct BorrowRequest {
    index: usize,
    mutable: bool,
}

struct StructFieldInfo {
    name: Ident,
    typ: Type,
    field_type: FieldType,
    borrows: Vec<BorrowRequest>,
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

fn handle_borrows_attr(
    field_info: &mut [StructFieldInfo],
    attr: &Attribute,
    borrows: &mut Vec<BorrowRequest>,
) {
    let mut borrow_mut = false;
    let mut waiting_for_comma = false;
    let tokens = attr.tokens.clone();
    let tokens = if let Some(TokenTree::Group(group)) = tokens.into_iter().next() {
        group.stream()
    } else {
        panic!("Invalid syntax for borrows() macro.");
    };
    for token in tokens {
        if let TokenTree::Ident(ident) = token {
            if waiting_for_comma {
                panic!("Unexpected '{}', expected comma.", ident);
            }
            let istr = ident.to_string();
            if istr == "mut" {
                if borrow_mut {
                    panic!("Unexpected double 'mut' in borrows() macro.");
                }
                borrow_mut = true;
            } else {
                let index = field_info.iter().position(|item| item.name == istr);
                let index = if let Some(v) = index {
                    v
                } else {
                    panic!(
                        concat!(
                            "Unknown identifier '{}', make sure that it is spelled ",
                            "correctly and defined above the location it is borrowed."
                        ),
                        istr
                    );
                };
                if borrow_mut {
                    if field_info[index].field_type == FieldType::Borrowed {
                        panic!(
                            "Cannot borrow '{}' as mut as it was previously borrowed immutably.",
                            istr,
                        );
                    }
                    if field_info[index].field_type == FieldType::BorrowedMut {
                        panic!("Cannot borrow '{}' mutably more than once.", istr,)
                    }
                    field_info[index].field_type = FieldType::BorrowedMut;
                } else {
                    if field_info[index].field_type == FieldType::BorrowedMut {
                        panic!(
                            "Cannot borrow '{}' again as it was previously borrowed mutably.",
                            istr,
                        );
                    }
                    field_info[index].field_type = FieldType::Borrowed;
                }
                borrows.push(BorrowRequest {
                    index,
                    mutable: borrow_mut,
                });
                waiting_for_comma = true;
                borrow_mut = false;
            }
        } else if let TokenTree::Punct(punct) = token {
            if punct.as_char() == ',' {
                if waiting_for_comma {
                    waiting_for_comma = false;
                } else {
                    panic!("Unexpected extra comma in borrows() macro.");
                }
            } else {
                panic!(
                    "Unexpected punctuation {}, expected comma or identifier.",
                    punct
                );
            }
        } else {
            panic!("Unexpected token {}, expected comma or identifier.", token);
        }
    }
}

#[proc_macro_attribute]
pub fn self_referencing(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_struct_def: ItemStruct = syn::parse_macro_input!(item);
    let struct_name = &original_struct_def.ident;
    let mod_name = format_ident!("ouroboros_impl_{}", struct_name.to_string().to_snake_case());
    let visibility = &original_struct_def.vis;

    // Actual data struct generation & metadata gathering.
    let mut actual_struct_def = original_struct_def.clone();
    actual_struct_def.vis = syn::parse_quote! { pub };
    let mut field_info = Vec::new();
    match &mut actual_struct_def.fields {
        Fields::Named(fields) => {
            for field in &mut fields.named {
                let mut borrows = Vec::new();
                for (index, attr) in field.attrs.iter().enumerate() {
                    let path = &attr.path;
                    if path.leading_colon.is_some() {
                        continue;
                    }
                    if path.segments.len() != 1 {
                        continue;
                    }
                    if path.segments.first().unwrap().ident.to_string() == "borrows" {
                        handle_borrows_attr(&mut field_info[..], attr, &mut borrows);
                        field.attrs.remove(index);
                        break;
                    }
                }
                field_info.push(StructFieldInfo {
                    name: field.ident.clone().expect("Named field has no name."),
                    typ: field.ty.clone(),
                    field_type: FieldType::Tail,
                    borrows,
                });
            }
        }
        Fields::Unnamed(_fields) => unimplemented!("Tuple structs are not supported yet."),
        Fields::Unit => panic!("Unit structs cannot be self-referential."),
    }
    if field_info.len() < 2 {
        panic!("Self-referencing structs must have at least 2 members.");
    }
    let mut has_non_tail = false;
    for field in &field_info {
        if !field.field_type.is_tail() {
            has_non_tail = true;
            break;
        }
    }
    if !has_non_tail {
        panic!(
            concat!(
                "Self-referencing struct cannot be made entirely of tail fields, try adding ",
                "#[borrows({0})] to a member defined after {0}."
            ),
            field_info[0].name
        );
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
                }
                GenericParam::Const(_) => unimplemented!(),
            }
        }
        arguments
    };

    // Constructor generation.
    let constructor_def: TokenStream2 = {
        let mut params: Vec<TokenStream2> = Vec::new();
        let mut code: Vec<TokenStream2> = Vec::new();
        for field in &field_info {
            let field_name = &field.name;
            let field_type = &field.typ;

            if field.borrows.len() > 0 {
                let mut field_builder_params = Vec::new();
                let mut field_builder_args = Vec::new();
                for borrow in &field.borrows {
                    if borrow.mutable {
                        unimplemented!();
                    } else {
                        let field = &field_info[borrow.index];
                        let field_type = &field.typ;
                        field_builder_params.push(quote! {
                            &'this <#field_type as ::std::ops::Deref>::Target
                        });
                        let ref_name = format_ident!("{}_illegal_static_reference", field.name);
                        field_builder_args.push(ref_name);
                    }
                }
                let builder_name = format_ident!("{}_builder", field_name);
                params.push(quote! {
                    #builder_name : impl for<'this> FnOnce(#(#field_builder_params),*) -> #field_type
                });
                code.push(quote! { let #field_name = #builder_name (#(#field_builder_args),*); })
            } else {
                // If it doesn't need to borrow anything, we can just copy it straight in to the
                // struct without any fancy builder nonsense.
                params.push(quote!( #field_name : #field_type ));
            }

            if field.field_type == FieldType::Borrowed {
                let ref_name = format_ident!("{}_illegal_static_reference", field_name);
                code.push(quote! {
                    let #ref_name = unsafe {
                        ::ouroboros::macro_help::stable_deref_and_strip_lifetime(&#field_name)
                    };
                });
            } else if field.field_type == FieldType::BorrowedMut {
                unimplemented!();
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
        if field.field_type == FieldType::Tail {
            users.push(quote! {
                pub fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> FnOnce(&'outer_borrow #field_type) -> ReturnType,
                ) -> ReturnType {
                    user(&self. #field_name)
                }
            });
        } else if field.field_type == FieldType::Borrowed {
            users.push(quote! {
                pub fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> FnOnce(&'outer_borrow <#field_type as ::std::ops::Deref>::Target) -> ReturnType,
                ) -> ReturnType {
                    user(&*self. #field_name)
                }
            });
        } else if field.field_type == FieldType::BorrowedMut {
            unimplemented!()
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
            if field.field_type == FieldType::Tail {
                fields.push(quote! { pub #field_name: &'outer_borrow #field_type });
                field_assignments.push(quote! { #field_name: &self.#field_name });
            } else if field.field_type == FieldType::Borrowed {
                fields.push(quote! { pub #field_name: &'outer_borrow <#field_type as ::std::ops::Deref>::Target });
                field_assignments.push(quote! { #field_name: &*self.#field_name });
            } else if field.field_type == FieldType::BorrowedMut {
                unimplemented!()
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
        mod #mod_name {
            #actual_struct_def
            pub #all_fields_struct
            impl #generic_producers #struct_name <#(#generic_consumers),*> {
                #constructor_def
                #(#users)*
                #use_all_fields_fn
            }
        }
        #visibility use #mod_name :: #struct_name;
    });
    // eprintln!("{:#?}", final_data);
    final_data
}
