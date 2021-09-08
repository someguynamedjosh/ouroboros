use quote::ToTokens;
use syn::{GenericArgument, PathArguments, Type};

use crate::utils::uses_this_lifetime;

const STD_CONTAINER_TYPES: &[&str] = &["Box", "Arc", "Rc"];

/// Returns Some((type_name, element_type)) if the provided type appears to be Box, Arc, or Rc from
/// the standard library. Returns None if not.
pub fn apparent_std_container_type(raw_type: &Type) -> Option<(&'static str, &Type)> {
    let tpath = if let Type::Path(x) = raw_type {
        x
    } else {
        return None;
    };
    let segment = if let Some(segment) = tpath.path.segments.last() {
        segment
    } else {
        return None;
    };
    let args = if let PathArguments::AngleBracketed(args) = &segment.arguments {
        args
    } else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    let arg = args.args.first().unwrap();
    let eltype = if let GenericArgument::Type(x) = arg {
        x
    } else {
        return None;
    };
    for type_name in STD_CONTAINER_TYPES {
        if segment.ident == type_name {
            return Some((type_name, eltype));
        }
    }
    None
}

/// Returns true if the specified type can be assumed to be covariant.
pub fn type_is_covariant(ty: &syn::Type, in_template: bool) -> bool {
    use syn::Type::*;
    // If the type never uses the 'this lifetime, we don't have to
    // worry about it not being covariant.
    if !uses_this_lifetime(ty.to_token_stream()) {
        return true;
    }
    match ty {
        Array(arr) => type_is_covariant(&*arr.elem, in_template),
        BareFn(f) => {
            for arg in f.inputs.iter() {
                if !type_is_covariant(&arg.ty, true) {
                    return false;
                }
            }
            if let syn::ReturnType::Type(_, ty) = &f.output {
                type_is_covariant(ty, true)
            } else {
                true
            }
        }
        Group(ty) => type_is_covariant(&ty.elem, in_template),
        ImplTrait(..) => false, // Unusable in struct definition.
        Infer(..) => false,     // Unusable in struct definition.
        Macro(..) => false,     // Assume false since we don't know.
        Never(..) => false,
        Paren(ty) => type_is_covariant(&ty.elem, in_template),
        Path(path) => {
            if let Some(qself) = &path.qself {
                if !type_is_covariant(&qself.ty, in_template) {
                    return false;
                }
            }
            let mut is_covariant = false;
            // If the type is Box, Arc, or Rc, we can assume it to be covariant.
            if apparent_std_container_type(ty).is_some() {
                is_covariant = true;
            }
            for segment in path.path.segments.iter() {
                let args = &segment.arguments;
                if let syn::PathArguments::AngleBracketed(args) = &args {
                    for arg in args.args.iter() {
                        if let syn::GenericArgument::Type(ty) = arg {
                            if !type_is_covariant(ty, !is_covariant) {
                                return false;
                            }
                        } else if let syn::GenericArgument::Lifetime(lt) = arg {
                            if lt.ident.to_string() == "this" && !is_covariant {
                                return false;
                            }
                        }
                    }
                } else if let syn::PathArguments::Parenthesized(args) = &args {
                    for arg in args.inputs.iter() {
                        if !type_is_covariant(arg, true) {
                            return false;
                        }
                    }
                    if let syn::ReturnType::Type(_, ty) = &args.output {
                        if !type_is_covariant(ty, true) {
                            return false;
                        }
                    }
                }
            }
            true
        }
        Ptr(ptr) => type_is_covariant(&ptr.elem, in_template),
        // Ignore the actual lifetime of the reference because Rust can automatically convert those.
        Reference(rf) => !in_template && type_is_covariant(&rf.elem, in_template),
        Slice(sl) => type_is_covariant(&sl.elem, in_template),
        TraitObject(..) => false,
        Tuple(tup) => {
            for ty in tup.elems.iter() {
                if !type_is_covariant(ty, in_template) {
                    return false;
                }
            }
            false
        }
        // As of writing this, syn parses all the types we could need.
        Verbatim(..) => unimplemented!(),
        _ => unimplemented!(),
    }
}
