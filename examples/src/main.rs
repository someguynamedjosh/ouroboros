use ouroboros::self_referencing;

#[derive(Clone, Debug)]
struct RefHolder<'a> {
    held: &'a (),
}

#[self_referencing]
pub struct Test<'a> {
    data: Box<i32>,
    #[tail]
    external: &'a i32,
    ptr2: &'this i32,
}

struct Manual {
    data: Box<()>,
    reff: RefHolder<'static>,
    real_ref: &'static (),
}

fn make_manual(
    data: Box<()>,
    reff_maker: impl for<'a> FnOnce(&'a ()) -> RefHolder<'a>,
    reff_maker_2: impl for<'a> FnOnce(&'a ()) -> &'a (),
) -> Manual {
    let data_ref: &'static () = unsafe { &*((&*data) as *const _) };
    let reff = reff_maker(data_ref);
    let real_ref = reff_maker_2(data_ref);
    Manual { data, reff, real_ref }
}

fn get_reff<'man, 'a>(manual: &'a Manual) -> &'a RefHolder<'man> {
    &manual.reff
}

fn get_ref2<'man, 'a>(manual: &'a Manual) -> &'a &'man () {
    &manual.real_ref
}

fn use_reff<'a, T>(manual: &'a Manual, function: impl for<'this> FnOnce(&'a RefHolder<'this>) -> T) -> T {
    function(&manual.reff)
}

fn use_ref2<'a, T>(manual: &'a Manual, function: impl for<'this> FnOnce(&'a &'this ()) -> T) -> T {
    function(&manual.real_ref)
}

fn main() {
    let external_int = 123;
    let test: Test = Test::new(Box::new(321), |_data| &external_int, |data| data);
    let reffed_data = test.use_ptr2(|data| *data);
    println!("{:?}", reffed_data);
    drop(test);

    // let manual = make_manual(Box::new(()), |data| RefHolder { held: data }, |data| data);
    // let externally_stored: &RefHolder<'_> = use_reff(&manual, |reff| reff);
    // let externally_stored = externally_stored.clone();
    // println!("{:?}", externally_stored);
    // drop(manual);
}
