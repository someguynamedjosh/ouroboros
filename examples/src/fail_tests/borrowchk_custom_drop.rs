use std::cell::RefCell;

use ouroboros::self_referencing;

struct Bar<'a>(RefCell<(Option<&'a Bar<'a>>, String)>);

#[self_referencing]
struct Foo {
    owner: (),
    #[borrows(owner)]
    #[not_covariant]
    bar: Bar<'this>,
    #[borrows(bar)]
    #[not_covariant]
    baz: &'this Bar<'this>,
}

impl Drop for Bar<'_> {
    fn drop(&mut self) {
        let r1 = self.0.get_mut();
        let string_ref_1 = &mut r1.1;
        let mut r2 = r1.0.unwrap().0.borrow_mut();
        let string_ref_2 = &mut r2.1;

        let s = &string_ref_1[..];
        string_ref_2.clear();
        string_ref_2.shrink_to_fit();
        println!("{}", s); // prints garbage :-), use-after free
    }
}

fn main() {
    Foo::new(
        (),
        |_| Bar(RefCell::new((None, "Hello World!".to_owned()))),
        |bar| {
            bar.0.borrow_mut().0 = Some(bar);
            bar
        },
    );
}
