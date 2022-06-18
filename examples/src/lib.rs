#![no_std]
extern crate alloc;
use alloc::boxed::Box;

use ouroboros::self_referencing;

#[cfg(test)]
mod ok_tests;

#[self_referencing]
/// A simple struct which contains an `i32` and a `&'this i32`.
pub struct DataAndRef {
    data: i32,
    #[borrows(data)]
    data_ref: &'this i32,
}

#[self_referencing()]
#[allow(clippy::redundant_allocation)]
/// A chain of references, where c references b which references a.
pub struct Chain {
    a: i32,
    #[borrows(a)]
    b: &'this i32,
    #[borrows(b)]
    c: &'this i32,
}

#[self_referencing]
/// The example provided in the documentation.
pub struct DocumentationExample {
    int_data: i32,
    float_data: f32,
    #[borrows(int_data)]
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}

#[self_referencing(no_doc)]
/// This struct is created using `#[self_referencing(no_doc)]` so the generated methods and
/// builders are hidden from documentation.
pub struct Undocumented {
    data: Box<i32>,
    #[borrows(data)]
    data_ref: &'this i32,
}

/// This struct demonstrates how visibility can be controlled. The struct
/// is defined with the following code:
/// ```rust
/// # use ouroboros::self_referencing;
/// #[self_referencing(pub_extras)]
/// pub struct Visibility {
///     private_field: Box<i32>,
///     #[borrows(private_field)]
///     pub public_field: &'this i32,
///     #[borrows(private_field)]
///     pub(crate) pub_crate_field: &'this i32,
/// }
/// ```
/// By using `pub_extras`, the visibility of items not related to any particular
/// field like `with_mut` or `VisibilityBuilder` is made public to match the
/// visibility of the original struct definition. Without adding this option,
/// these items would only be visible in the module where the struct is
/// declared.
#[self_referencing(pub_extras)]
pub struct Visibility {
    private_field: Box<i32>,
    #[borrows(private_field)]
    pub public_field: &'this i32,
    #[borrows(private_field)]
    pub(crate) pub_crate_field: &'this i32,
}
