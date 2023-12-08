use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Lifetime, WhereClause};

use crate::info_structures::{FieldType, Options, StructInfo};

/// Returns the Heads struct and a function to convert the original struct into a Heads instance.
pub fn make_into_heads(info: &StructInfo, options: Options) -> (TokenStream, TokenStream) {
    let visibility = if options.do_pub_extras {
        info.vis.clone()
    } else {
        syn::parse_quote! { pub(super) }
    };
    let mut code = Vec::new();
    let mut field_initializers = Vec::new();
    let mut head_fields = Vec::new();
    let internal_struct = &info.internal_ident;
    // Drop everything in the reverse order of what it was declared in. Fields that come later
    // are only dependent on fields that came before them.
    for field in info.fields.iter().rev() {
        let field_name = &field.name;
        if field.self_referencing {
            // Heads are fields that do not borrow anything.
            code.push(quote! { ::core::mem::drop(this.#field_name); });
        } else {
            code.push(quote! { let #field_name = this.#field_name; });
            if field.is_borrowed() {
                field_initializers
                    .push(quote! { #field_name: ::ouroboros::macro_help::unbox(#field_name) });
            } else {
                field_initializers.push(quote! { #field_name });
            }
            let field_type = &field.typ;
            head_fields.push(quote! { #visibility #field_name: #field_type });
        }
    }
    for (ty, ident) in info.generic_consumers() {
        head_fields.push(quote! { #ident: ::core::marker::PhantomData<#ty> });
        field_initializers.push(quote! { #ident: ::core::marker::PhantomData });
    }
    let documentation = format!(
        concat!(
            "A struct which contains only the ",
            "[head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) of [`{0}`]({0})."
        ),
        info.ident.to_string()
    );
    let generic_params = info.generic_params();
    let generic_where = &info.generics.where_clause;
    let heads_struct_def = quote! {
        #[doc=#documentation]
        #visibility struct Heads <#generic_params> #generic_where {
            #(#head_fields),*
        }
    };
    let documentation = concat!(
        "This function drops all internally referencing fields and returns only the ",
        "[head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) of this struct."
    ).to_owned();

    let documentation = if !options.do_no_doc {
        quote! {
            #[doc=#documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };

    let generic_args = info.generic_arguments();
    let into_heads_fn = quote! {
        #documentation
        #[allow(clippy::drop_ref)]
        #[allow(clippy::drop_copy)]
        #[allow(clippy::drop_non_drop)]
        #visibility fn into_heads(self) -> Heads<#(#generic_args),*> {
            let this_ptr = &self as *const _;
            let this: #internal_struct<#(#generic_args),*> = unsafe { ::core::mem::transmute_copy(&*this_ptr) };
            ::core::mem::forget(self);
            #(#code)*
            Heads {
                #(#field_initializers),*
            }
        }
    };
    (heads_struct_def, into_heads_fn)
}

pub fn destruct_into_heads(
    info: &StructInfo,
    options: Options,
) -> Result<(TokenStream, TokenStream), Error> {
    let internal_struct = &info.internal_ident;

    let visibility = if options.do_pub_extras {
        info.vis.clone()
    } else {
        syn::parse_quote! { pub(super) }
    };
    let mut code = Vec::new();
    let mut fields = Vec::new();
    let mut field_assignments = Vec::new();
    let mut field_initializers = Vec::new();

    // I don't think the reverse is necessary but it does make the expanded code more uniform.
    for field in info.fields.iter().rev() {
        let field_name = &field.name;
        let field_type = &field.typ;
        if !field.self_referencing {
            code.push(quote! { let #field_name = this.#field_name; });
            if field.is_borrowed() {
                field_initializers
                    .push(quote! { #field_name: ::ouroboros::macro_help::unbox(#field_name) });
            } else {
                field_initializers.push(quote! { #field_name });
            }
        } else if field.field_type == FieldType::Tail {
            fields.push(quote! { #visibility #field_name: #field_type });
            field_assignments.push(quote! { #field_name: this.#field_name });
        }
    }

    for (ty, ident) in info.generic_consumers() {
        fields.push(quote! { #ident: ::core::marker::PhantomData<#ty> });
        field_assignments.push(quote! { #ident: ::core::marker::PhantomData });
        field_initializers.push(quote! { #ident: ::core::marker::PhantomData });
    }

    let new_generic_params = if info.generic_params().is_empty() {
        quote! { <'this> }
    } else {
        let mut new_generic_params = info.generic_params().clone();
        new_generic_params.insert(0, syn::parse_quote! { 'this });
        quote! { <#new_generic_params> }
    };
    let generic_args = info.generic_arguments();
    let mut new_generic_args = info.generic_arguments();
    new_generic_args.insert(0, quote! { 'this });

    let struct_documentation = format!(
        concat!(
            "A struct for holding ",
            "[tail fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) in an instance of ",
            "[`{0}`]({0})."
        ),
        info.ident.to_string()
    );
    let generic_where = info.generics.where_clause.clone();
    let struct_defs = quote! {
        #[doc=#struct_documentation]
        #visibility struct OwnedTailFields #new_generic_params #generic_where { #(#fields),* }
    };
    let borrowed_fields_type = quote! { OwnedTailFields<#(#new_generic_args),*> };
    let documentation = concat!(
        "This method provides immutable references to all ",
        "[tail and immutably borrowed fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions).",
    );
    let documentation = if !options.do_no_doc {
        quote! {
            #[doc=#documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };
    let fn_defs = quote! {
        #documentation
        #[inline(always)]
        #visibility fn destruct_into_heads <ReturnType>(
            self,
            user: impl for<'this> ::core::ops::FnOnce(#borrowed_fields_type) -> ReturnType
        ) -> (ReturnType, Heads<#(#generic_args),*>) {
            let this_ptr = &self as *const _;
            let this: #internal_struct<#(#generic_args),*> = unsafe { ::core::mem::transmute_copy(&*this_ptr) };
            ::core::mem::forget(self);

            let user_return = user(OwnedTailFields {
                #(#field_assignments),*
            });

            let heads = {
                #(#code)*

                Heads {
                    #(#field_initializers),*
                }
            };

            (user_return, heads)
        }
    };
    Ok((struct_defs, fn_defs))
}
