use ouroboros::self_referencing;

#[self_referencing]
pub struct S<'a> {
    o: String,

    #[borrows(o)]
    c: &'a &'this (),

    e: &'a (),
}

fn main() {
    #[allow(clippy::needless_lifetimes)]
    fn bar<'a>(x: &'a ()) -> &'a str {
        let s = SBuilder {
            o: "Hello World!".to_owned(),
            c_builder: |_| &&(),
            e: x,
        }
        .build();
        let r = s.with(f);
        return r;

        fn f<'outer_borrow, 'this, 'a>(
            b: ouroboros_impl_s::BorrowedFields<'outer_borrow, 'this, 'a>,
        ) -> &'a str {
            b.o
        }
    }

    let s = bar(&());
    println!("{}", s); // use-after-free :-)
}
