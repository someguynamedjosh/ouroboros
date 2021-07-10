use ouroboros::self_referencing;

#[self_referencing]
struct BoxAndRef {
    data: i32,
    #[borrows(data)]
    data_ref: &'this i32,
}

fn main() {
    let instance = BoxAndRefBuilder {
        data: 12,
        data_ref_builder: |dref| dref,
    }.build();
    let mut stored_ref: Option<&'static i32> = None;
    instance.with_data_ref(|dref| stored_ref = Some(*dref));
}
