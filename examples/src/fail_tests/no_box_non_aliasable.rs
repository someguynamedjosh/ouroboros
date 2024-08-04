use ouroboros::self_referencing;

#[self_referencing]
struct Test {
    #[no_box]
    data: (),
    #[borrows(data)]
    field: (),
}

#[self_referencing]
struct Test {
    #[no_box]
    data: Box<()>,
    #[borrows(data)]
    field: (),
}
