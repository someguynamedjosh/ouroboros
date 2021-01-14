use ouroboros::self_referencing;

struct NotGuaranteedCovariant<'a> {
    data: &'a (),
}

#[self_referencing]
struct Test {
    data: Box<()>,
    #[borrows(data)]
    field: NotGuaranteedCovariant<'this>
}