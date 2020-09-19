use ouroboros::self_referencing;

#[derive(Clone, Debug)]
struct RefHolder<'a> {
    held: &'a (),
}

#[self_referencing]
pub struct Test<'a, D: 'static + Clone> {
    data1: Box<D>,
    data2: Box<D>,
    external: &'a D,
    #[borrows(data1)]
    ptr1: &'this D,
    #[borrows(data2)]
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
    let mut test = TestBuilder {
        data1: Box::new(321),
        data2: Box::new(555),
        external: &external_int,
        ptr1_builder: |data| data,
        ptr2_builder: |data| data,
    }.build();
    let reffed_data = test.use_ptr2(|data| *data);
    println!("{:?}", reffed_data);
    test.use_all_fields_mut(|all_fields| {
        *all_fields.ptr2 = *all_fields.ptr1;
    });
    drop(test);
}
