use ouroboros::self_referencing;

#[self_referencing]
/// A simple struct which contains a `Box<i32>` and a `&'this i32`.
pub struct BoxAndRef {
    data: Box<i32>,
    #[borrows(data)]
    data_ref: &'this i32,
}

/// A chain of references, where c references b which references a.
#[self_referencing]
pub struct ChainRef {
    a: Box<i32>,
    #[borrows(a)]
    b: Box<&'this i32>,
    #[borrows(b)]
    c: Box<&'this i32>,
}

#[self_referencing]
/// The example provided in the documentation.
pub struct DocumentationExample {
    int_data: Box<i32>,
    float_data: Box<f32>,
    #[borrows(int_data)]
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}