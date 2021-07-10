use ouroboros::self_referencing;

struct NotGuaranteedCovariant<'a> {
    data: &'a (),
}

#[self_referencing]
struct Test {
    data: (),
    #[borrows(data)]
    field: NotGuaranteedCovariant<'this>
}