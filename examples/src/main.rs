use ouroboros::self_referencing;

#[derive(Clone, Debug)]
struct RefHolder<'a> {
    held: &'a (),
}

#[self_referencing]
pub struct Test<'a, D: 'static + Clone> {
    data: Box<D>,
    external: &'a D,
    #[borrows(data)]
    ptr2: &'this D,
}

#[self_referencing]
pub struct Test2 {
    data: Box<i32>,
    #[borrows(data)]
    ptr: &'this i32,
}

fn main() {
    let external_int = 123;
    let test: Test<i32> = Test::new(Box::new(321), &external_int, |data| data);
    let reffed_data = test.use_ptr2(|data| *data);
    println!("{:?}", reffed_data);
    drop(test);
}
