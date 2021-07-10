use ouroboros::self_referencing;
use std::fmt::Debug;

// All tests here should compile and run correctly and pass Miri's safety checks.

#[self_referencing]
struct TraitObject {
    data: Box<dyn Debug>,
    #[borrows(data)]
    #[covariant]
    dref: &'this dyn Debug,
}

#[self_referencing]
struct BoxAndRef {
    data: i32,
    #[borrows(data)]
    dref: &'this i32,
}

#[self_referencing]
struct BoxAndMutRef {
    data: i32,
    #[borrows(mut data)]
    dref: &'this mut i32,
}

#[self_referencing(no_doc)]
struct ChainedAndUndocumented {
    data: i32,
    #[borrows(data)]
    ref1: &'this i32,
    #[borrows(data)]
    ref2: &'this &'this i32,
}

#[self_referencing]
struct BoxCheckWithLifetimeParameter<'t> {
    external_data: &'t (),
    #[borrows(external_data)]
    #[covariant]
    self_reference: &'this &'t (),
}

#[self_referencing]
struct AutoDetectCovarianceOnFieldsWithoutThis {
    data: (),
    unrelated_data: Box<i32>,
    #[borrows(data)]
    self_reference: &'this (),
}

/// This test just makes sure that the macro copes with a ton of template parameters being thrown at
/// it, specifically checking that the templates work fine even when a generated struct doesn't need
/// all of them. (E.G. heads will only contain 'd, A, and B.)
#[self_referencing]
struct TemplateMess<'d, A, B, C>
where
    A: ?Sized,
    B: 'static,
    C: 'static,
{
    external: &'d A,
    data1: B,
    #[borrows(data1)]
    data2: &'this C,
    data3: B,
    #[borrows(mut data3)]
    data4: &'this mut C,
}

#[test]
fn box_and_ref() {
    let bar = BoxAndRefBuilder {
        data: 12,
        dref_builder: |data| data,
    }
    .build();
    assert!(bar.with_dref(|dref| **dref) == 12);
    drop(bar);
}

// Miri crashes with Pin<Box<Future>> types due to
// https://github.com/rust-lang/miri/issues/1038
#[cfg(not(feature = "miri"))]
#[tokio::test]
async fn async_new() {
    let bar = BoxAndRefAsyncBuilder {
        data: 12,
        dref_builder: |data| Box::pin(async move { data }),
    }
    .build()
    .await;
    assert!(bar.with_dref(|dref| **dref) == 12);
    drop(bar);
}

// Miri crashes with Pin<Box<Future>> types due to
// https://github.com/rust-lang/miri/issues/1038
#[cfg(not(feature = "miri"))]
#[tokio::test]
async fn async_try_new() {
    let bar = BoxAndRefAsyncTryBuilder {
        data: 12,
        dref_builder: |data| Box::pin(async move { Result::<_, ()>::Ok(data) }),
    }
    .try_build()
    .await
    .unwrap();
    assert!(bar.with_dref(|dref| **dref) == 12);
    drop(bar);
}

// Miri crashes with Pin<Box<Future>> types due to
// https://github.com/rust-lang/miri/issues/1038
#[cfg(not(feature = "miri"))]
#[tokio::test]
async fn async_try_new_err() {
    let result = BoxAndRefAsyncTryBuilder {
        data: 12,
        dref_builder: |_data| Box::pin(async move { Err(56u64) }),
    }
    .try_build()
    .await;
    if let Err(56) = result {
        // okay
    } else {
        panic!("Test failed.");
    }
}

#[test]
fn try_new() {
    let bar = BoxAndRefTryBuilder {
        data: 12,
        dref_builder: |data| Result::<_, ()>::Ok(data),
    }
    .try_build()
    .unwrap();
    assert!(bar.with_dref(|dref| **dref) == 12);
    drop(bar);
}

#[test]
fn try_new_err() {
    let result = BoxAndRefTryBuilder {
        data: 12,
        dref_builder: |_data| Err(56),
    }
    .try_build();
    if let Err(56) = result {
        // okay
    } else {
        panic!("Test failed.");
    }
}

#[test]
fn try_new_recover_heads() {
    let result = BoxAndRefTryBuilder {
        data: 12,
        dref_builder: |_data| Err(56),
    }
    .try_build_or_recover();
    if let Err((56, heads)) = result {
        assert!(heads.data == 12);
    } else {
        panic!("Test failed.");
    }
}

#[test]
fn into_heads() {
    let bar = BoxAndRefBuilder {
        data: 12,
        dref_builder: |data| data,
    }
    .build();
    assert!(bar.into_heads().data == 12);
}

#[test]
fn box_and_mut_ref() {
    let mut bar = BoxAndMutRefBuilder {
        data: 12,
        dref_builder: |data| data,
    }
    .build();
    assert!(bar.with_dref(|dref| **dref) == 12);
    bar.with_dref_mut(|dref| **dref = 34);
    assert!(bar.with_dref(|dref| **dref) == 34);
}

#[test]
fn template_mess() {
    let ext_str = "Hello World!".to_owned();
    let mut instance = TemplateMessBuilder {
        external: &ext_str[..],
        data1: "asdf".to_owned(),
        data2_builder: |data1_contents| data1_contents,
        data3: "asdf".to_owned(),
        data4_builder: |data3_contents| data3_contents,
    }
    .build();
    instance.with_external(|ext| println!("{}", ext));
    instance.with_data1(|data| println!("{}", *data));
    instance.with_data4_mut(|con| **con = "Modified".to_owned());
    instance.with(|fields| {
        assert!(**fields.data1 == **fields.data2);
        assert!(*fields.data4 == "Modified");
    });
}

const STATIC_INT: i32 = 456;
#[test]
fn self_reference_with() {
    let mut bar = BoxAndRef::new(123, |b| b);
    bar.with_dref(|dref| {
        assert_eq!(**dref, 123);
    });
    bar.with_dref_mut(|dref| {
        *dref = &STATIC_INT;
    });
    assert_eq!(**bar.borrow_dref(), STATIC_INT);
    bar.with_mut(|fields| {
        *fields.dref = fields.data;
    });
    assert_eq!(**bar.borrow_dref(), 123);
}

#[test]
fn single_lifetime() {
    #[self_referencing]
    struct Struct<'a> {
        external: &'a str,
        #[borrows(external)]
        internal: &'this &'a str,
    }
}

#[test]
fn double_lifetime() {
    #[self_referencing]
    struct Struct<'a, 'b: 'a> {
        external: &'a str,
        external2: &'b str,
        #[borrows(external, external2)]
        internal: &'this &'b str,
    }
}

#[cfg(not(feature = "miri"))]
mod compile_tests {
    /// Tests that all files in fail_tests fail to compile.
    #[test]
    fn fails_ok() {
        let t = trybuild::TestCases::new();
        t.compile_fail("src/fail_tests/*.rs");
    }
}

#[allow(dead_code)]
mod test_hygiene {
    mod std {}
    mod core {}

    struct Copy;
    struct Send;
    struct Sync;
    struct Sized;

    struct Drop;
    struct Fn;
    struct FnMut;
    struct FnOnce;

    struct Result;
    struct Ok;
    struct Err;
    struct Option;
    struct Some;
    struct None;

    fn drop() {}

    #[ouroboros::self_referencing]
    struct BoxAndRef {
        data: i32,
        #[borrows(data)]
        dref: &'this i32,
    }
}
