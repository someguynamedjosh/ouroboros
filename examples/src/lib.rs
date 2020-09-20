use ouroboros::self_referencing;

#[self_referencing]
/// A simple struct which contains a `Box<i32>` and a `&'this i32`.
pub struct BoxAndRef {
    data: Box<i32>,
    #[borrows(data)]
    data_ref: &'this i32,
}
