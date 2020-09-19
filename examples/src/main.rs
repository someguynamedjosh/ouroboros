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
    #[borrows(mut data2)]
    ptr2: &'this mut D,
}

#[self_referencing]
pub struct Test2 {
    data: Box<i32>,
    #[borrows(data)]
    ptr: &'this i32,
}

fn main() {
    let external_int = 123;
    let mut test = TestTryBuilder {
        data1: Box::new(321),
        data2: Box::new(555),
        external: &external_int,
        ptr1_builder: |data| Result::<_, ()>::Ok(data),
        ptr2_builder: |data| Ok(data),
    }.try_build().unwrap();
    println!("{:?}", test.use_data1_contents(|data| data));
    test.use_ptr2_mut(|data| **data = 444);
    let reffed_data = test.use_ptr2(|data| &**data);
    println!("{:?}", reffed_data);
    
    let heads = test.into_heads();
    println!("{:?}", heads.data2);
    assert!(heads.data2 == Box::new(444));
}
