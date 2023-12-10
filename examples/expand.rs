#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate alloc;
use alloc::boxed::Box;
use ouroboros::self_referencing;
///Encapsulates implementation details for a self-referencing struct. This module is only visible when using --document-private-items.
mod ouroboros_impl_with_const_param {
    use super::*;
    ///The self-referencing struct.
    #[repr(transparent)]
    pub(super) struct WithConstParam<const REV: bool> {
        actual_data: ::core::mem::MaybeUninit<WithConstParamInternal<REV>>,
    }
    struct WithConstParamInternal<const REV: bool> {
        #[doc(hidden)]
        dref: Option<&'static Box<Vec<u64>>>,
        #[doc(hidden)]
        data: ::ouroboros::macro_help::AliasableBox<Box<Vec<u64>>>,
    }
    impl<const REV: bool> ::core::ops::Drop for WithConstParam<REV> {
        fn drop(&mut self) {
            unsafe { self.actual_data.assume_init_drop() };
        }
    }
    fn check_if_okay_according_to_checkers<const REV: bool>(
        data: Box<Vec<u64>>,
        dref_builder: impl for<'this> ::core::ops::FnOnce(
            &'this Box<Vec<u64>>,
        ) -> Option<&'this Box<Vec<u64>>>,
    ) {
        let data = data;
        let dref = dref_builder(&data);
        let dref = dref;
        BorrowedFields::<'_, '_, REV> {
            data: &data,
            dref: &dref,
            _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
        };
    }
    /**A more verbose but stable way to construct self-referencing structs. It is comparable to using `StructName { field1: value1, field2: value2 }` rather than `StructName::new(value1, value2)`. This has the dual benefit of making your code both easier to refactor and more readable. Call [`build()`](Self::build) to construct the actual struct. The fields of this struct should be used as follows:

| Field | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> dref: _` |
*/
    pub(super) struct WithConstParamBuilder<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> Option<&'this Box<Vec<u64>>>,
    > {
        pub(super) data: Box<Vec<u64>>,
        pub(super) dref_builder: DrefBuilder_,
    }
    impl<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> Option<&'this Box<Vec<u64>>>,
    > WithConstParamBuilder<REV, DrefBuilder_> {
        ///Calls [`WithConstParam::new()`](WithConstParam::new) using the provided values. This is preferable over calling `new()` directly for the reasons listed above.
        pub(super) fn build(self) -> WithConstParam<REV> {
            WithConstParam::new(self.data, self.dref_builder)
        }
    }
    /**A more verbose but stable way to construct self-referencing structs. It is comparable to using `StructName { field1: value1, field2: value2 }` rather than `StructName::new(value1, value2)`. This has the dual benefit of making your code both easier to refactor and more readable. Call [`build()`](Self::build) to construct the actual struct. The fields of this struct should be used as follows:

| Field | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> dref: _` |
*/
    pub(super) struct WithConstParamAsyncBuilder<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = Option<&'this Box<Vec<u64>>>,
                        > + 'this,
                    >,
                >,
    > {
        pub(super) data: Box<Vec<u64>>,
        pub(super) dref_builder: DrefBuilder_,
    }
    impl<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = Option<&'this Box<Vec<u64>>>,
                        > + 'this,
                    >,
                >,
    > WithConstParamAsyncBuilder<REV, DrefBuilder_> {
        ///Calls [`WithConstParam::new()`](WithConstParam::new) using the provided values. This is preferable over calling `new()` directly for the reasons listed above.
        pub(super) async fn build(self) -> WithConstParam<REV> {
            WithConstParam::new_async(self.data, self.dref_builder).await
        }
    }
    /**A more verbose but stable way to construct self-referencing structs. It is comparable to using `StructName { field1: value1, field2: value2 }` rather than `StructName::new(value1, value2)`. This has the dual benefit of making your code both easier to refactor and more readable. Call [`build()`](Self::build) to construct the actual struct. The fields of this struct should be used as follows:

| Field | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> dref: _` |
*/
    pub(super) struct WithConstParamAsyncSendBuilder<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = Option<&'this Box<Vec<u64>>>,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
    > {
        pub(super) data: Box<Vec<u64>>,
        pub(super) dref_builder: DrefBuilder_,
    }
    impl<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = Option<&'this Box<Vec<u64>>>,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
    > WithConstParamAsyncSendBuilder<REV, DrefBuilder_> {
        ///Calls [`WithConstParam::new()`](WithConstParam::new) using the provided values. This is preferable over calling `new()` directly for the reasons listed above.
        pub(super) async fn build(self) -> WithConstParam<REV> {
            WithConstParam::new_async_send(self.data, self.dref_builder).await
        }
    }
    /**A more verbose but stable way to construct self-referencing structs. It is comparable to using `StructName { field1: value1, field2: value2 }` rather than `StructName::new(value1, value2)`. This has the dual benefit of making your code both easier to refactor and more readable. Call [`try_build()`](Self::try_build) or [`try_build_or_recover()`](Self::try_build_or_recover) to construct the actual struct. The fields of this struct should be used as follows:

| Field | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
    pub(super) struct WithConstParamTryBuilder<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::result::Result<Option<&'this Box<Vec<u64>>>, Error_>,
        Error_,
    > {
        pub(super) data: Box<Vec<u64>>,
        pub(super) dref_builder: DrefBuilder_,
    }
    impl<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::result::Result<Option<&'this Box<Vec<u64>>>, Error_>,
        Error_,
    > WithConstParamTryBuilder<REV, DrefBuilder_, Error_> {
        ///Calls [`WithConstParam::try_new()`](WithConstParam::try_new) using the provided values. This is preferable over calling `try_new()` directly for the reasons listed above.
        pub(super) fn try_build(
            self,
        ) -> ::core::result::Result<WithConstParam<REV>, Error_> {
            WithConstParam::try_new(self.data, self.dref_builder)
        }
        ///Calls [`WithConstParam::try_new_or_recover()`](WithConstParam::try_new_or_recover) using the provided values. This is preferable over calling `try_new_or_recover()` directly for the reasons listed above.
        pub(super) fn try_build_or_recover(
            self,
        ) -> ::core::result::Result<WithConstParam<REV>, (Error_, Heads<REV>)> {
            WithConstParam::try_new_or_recover(self.data, self.dref_builder)
        }
    }
    /**A more verbose but stable way to construct self-referencing structs. It is comparable to using `StructName { field1: value1, field2: value2 }` rather than `StructName::new(value1, value2)`. This has the dual benefit of making your code both easier to refactor and more readable. Call [`try_build()`](Self::try_build) or [`try_build_or_recover()`](Self::try_build_or_recover) to construct the actual struct. The fields of this struct should be used as follows:

| Field | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
    pub(super) struct WithConstParamAsyncTryBuilder<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + 'this,
                    >,
                >,
        Error_,
    > {
        pub(super) data: Box<Vec<u64>>,
        pub(super) dref_builder: DrefBuilder_,
    }
    impl<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + 'this,
                    >,
                >,
        Error_,
    > WithConstParamAsyncTryBuilder<REV, DrefBuilder_, Error_> {
        ///Calls [`WithConstParam::try_new()`](WithConstParam::try_new) using the provided values. This is preferable over calling `try_new()` directly for the reasons listed above.
        pub(super) async fn try_build(
            self,
        ) -> ::core::result::Result<WithConstParam<REV>, Error_> {
            WithConstParam::try_new_async(self.data, self.dref_builder).await
        }
        ///Calls [`WithConstParam::try_new_or_recover()`](WithConstParam::try_new_or_recover) using the provided values. This is preferable over calling `try_new_or_recover()` directly for the reasons listed above.
        pub(super) async fn try_build_or_recover(
            self,
        ) -> ::core::result::Result<WithConstParam<REV>, (Error_, Heads<REV>)> {
            WithConstParam::try_new_or_recover_async(self.data, self.dref_builder).await
        }
    }
    /**A more verbose but stable way to construct self-referencing structs. It is comparable to using `StructName { field1: value1, field2: value2 }` rather than `StructName::new(value1, value2)`. This has the dual benefit of making your code both easier to refactor and more readable. Call [`try_build()`](Self::try_build) or [`try_build_or_recover()`](Self::try_build_or_recover) to construct the actual struct. The fields of this struct should be used as follows:

| Field | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
    pub(super) struct WithConstParamAsyncSendTryBuilder<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
        Error_,
    > {
        pub(super) data: Box<Vec<u64>>,
        pub(super) dref_builder: DrefBuilder_,
    }
    impl<
        const REV: bool,
        DrefBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
        Error_,
    > WithConstParamAsyncSendTryBuilder<REV, DrefBuilder_, Error_> {
        ///Calls [`WithConstParam::try_new()`](WithConstParam::try_new) using the provided values. This is preferable over calling `try_new()` directly for the reasons listed above.
        pub(super) async fn try_build(
            self,
        ) -> ::core::result::Result<WithConstParam<REV>, Error_> {
            WithConstParam::try_new_async_send(self.data, self.dref_builder).await
        }
        ///Calls [`WithConstParam::try_new_or_recover()`](WithConstParam::try_new_or_recover) using the provided values. This is preferable over calling `try_new_or_recover()` directly for the reasons listed above.
        pub(super) async fn try_build_or_recover(
            self,
        ) -> ::core::result::Result<WithConstParam<REV>, (Error_, Heads<REV>)> {
            WithConstParam::try_new_or_recover_async_send(self.data, self.dref_builder)
                .await
        }
    }
    ///A struct for holding immutable references to all [tail and immutably borrowed fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) in an instance of [`WithConstParam`](WithConstParam).
    pub(super) struct BorrowedFields<'outer_borrow, 'this, const REV: bool>
    where
        'static: 'this,
        'this: 'outer_borrow,
    {
        pub(super) dref: &'outer_borrow Option<&'this Box<Vec<u64>>>,
        pub(super) data: &'this Box<Vec<u64>>,
        _comsume_template_const_parameter_rev: ::core::marker::PhantomData<()>,
    }
    ///A struct for holding mutable references to all [tail fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) in an instance of [`WithConstParam`](WithConstParam).
    pub(super) struct BorrowedMutFields<'outer_borrow, 'this1, 'this0, const REV: bool>
    where
        'static: 'this0,
        'static: 'this1,
        'this1: 'this0,
    {
        pub(super) dref: &'outer_borrow mut Option<&'this0 Box<Vec<u64>>>,
        pub(super) data: &'this1 Box<Vec<u64>>,
        _comsume_template_const_parameter_rev: ::core::marker::PhantomData<()>,
    }
    ///A struct which contains only the [head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) of [`WithConstParam`](WithConstParam).
    pub(super) struct Heads<const REV: bool> {
        pub(super) data: Box<Vec<u64>>,
        _comsume_template_const_parameter_rev: ::core::marker::PhantomData<()>,
    }
    impl<const REV: bool> WithConstParam<REV> {
        /**Constructs a new instance of this self-referential struct. (See also [`WithConstParamBuilder::build()`](WithConstParamBuilder::build)). Each argument is a field of the new struct. Fields that refer to other fields inside the struct are initialized using functions instead of directly passing their value. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> dref: _` |
*/
        pub(super) fn new(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> Option<&'this Box<Vec<u64>>>,
        ) -> WithConstParam<REV> {
            let data = ::ouroboros::macro_help::aliasable_boxed(data);
            let data_illegal_static_reference = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*data)
            };
            let dref = dref_builder(data_illegal_static_reference);
            unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(WithConstParamInternal {
                        data,
                        dref,
                    }),
                }
            }
        }
        /**Constructs a new instance of this self-referential struct. (See also [`WithConstParamAsyncBuilder::build()`](WithConstParamAsyncBuilder::build)). Each argument is a field of the new struct. Fields that refer to other fields inside the struct are initialized using functions instead of directly passing their value. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> dref: _` |
*/
        pub(super) async fn new_async(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = Option<&'this Box<Vec<u64>>>,
                        > + 'this,
                    >,
                >,
        ) -> WithConstParam<REV> {
            let data = ::ouroboros::macro_help::aliasable_boxed(data);
            let data_illegal_static_reference = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*data)
            };
            let dref = dref_builder(data_illegal_static_reference).await;
            unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(WithConstParamInternal {
                        data,
                        dref,
                    }),
                }
            }
        }
        /**Constructs a new instance of this self-referential struct. (See also [`WithConstParamAsyncSendBuilder::build()`](WithConstParamAsyncSendBuilder::build)). Each argument is a field of the new struct. Fields that refer to other fields inside the struct are initialized using functions instead of directly passing their value. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> dref: _` |
*/
        pub(super) async fn new_async_send(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = Option<&'this Box<Vec<u64>>>,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
        ) -> WithConstParam<REV> {
            let data = ::ouroboros::macro_help::aliasable_boxed(data);
            let data_illegal_static_reference = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*data)
            };
            let dref = dref_builder(data_illegal_static_reference).await;
            unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(WithConstParamInternal {
                        data,
                        dref,
                    }),
                }
            }
        }
        /**(See also [`WithConstParamTryBuilder::try_build()`](WithConstParamTryBuilder::try_build).) Like [`new`](Self::new), but builders for [self-referencing fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) can return results. If any of them fail, `Err` is returned. If all of them succeed, `Ok` is returned. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
        pub(super) fn try_new<Error_>(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::result::Result<Option<&'this Box<Vec<u64>>>, Error_>,
        ) -> ::core::result::Result<WithConstParam<REV>, Error_> {
            WithConstParam::try_new_or_recover(data, dref_builder)
                .map_err(|(error, _heads)| error)
        }
        /**(See also [`WithConstParamTryBuilder::try_build_or_recover()`](WithConstParamTryBuilder::try_build_or_recover).) Like [`try_new`](Self::try_new), but all [head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) are returned in the case of an error. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
        pub(super) fn try_new_or_recover<Error_>(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::result::Result<Option<&'this Box<Vec<u64>>>, Error_>,
        ) -> ::core::result::Result<WithConstParam<REV>, (Error_, Heads<REV>)> {
            let data = ::ouroboros::macro_help::aliasable_boxed(data);
            let data_illegal_static_reference = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*data)
            };
            let dref = match dref_builder(data_illegal_static_reference) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(err) => {
                    return ::core::result::Result::Err((
                        err,
                        Heads {
                            data: ::ouroboros::macro_help::unbox(data),
                            _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
                        },
                    ));
                }
            };
            ::core::result::Result::Ok(unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(WithConstParamInternal {
                        data,
                        dref,
                    }),
                }
            })
        }
        /**(See also [`WithConstParamAsyncTryBuilder::try_build()`](WithConstParamAsyncTryBuilder::try_build).) Like [`new`](Self::new), but builders for [self-referencing fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) can return results. If any of them fail, `Err` is returned. If all of them succeed, `Ok` is returned. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
        pub(super) async fn try_new_async<Error_>(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + 'this,
                    >,
                >,
        ) -> ::core::result::Result<WithConstParam<REV>, Error_> {
            WithConstParam::try_new_or_recover_async(data, dref_builder)
                .await
                .map_err(|(error, _heads)| error)
        }
        /**(See also [`WithConstParamAsyncTryBuilder::try_build_or_recover()`](WithConstParamAsyncTryBuilder::try_build_or_recover).) Like [`try_new`](Self::try_new), but all [head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) are returned in the case of an error. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
        pub(super) async fn try_new_or_recover_async<Error_>(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + 'this,
                    >,
                >,
        ) -> ::core::result::Result<WithConstParam<REV>, (Error_, Heads<REV>)> {
            let data = ::ouroboros::macro_help::aliasable_boxed(data);
            let data_illegal_static_reference = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*data)
            };
            let dref = match dref_builder(data_illegal_static_reference).await {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(err) => {
                    return ::core::result::Result::Err((
                        err,
                        Heads {
                            data: ::ouroboros::macro_help::unbox(data),
                            _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
                        },
                    ));
                }
            };
            ::core::result::Result::Ok(unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(WithConstParamInternal {
                        data,
                        dref,
                    }),
                }
            })
        }
        /**(See also [`WithConstParamAsyncSendTryBuilder::try_build()`](WithConstParamAsyncSendTryBuilder::try_build).) Like [`new`](Self::new), but builders for [self-referencing fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) can return results. If any of them fail, `Err` is returned. If all of them succeed, `Ok` is returned. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
        pub(super) async fn try_new_async_send<Error_>(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
        ) -> ::core::result::Result<WithConstParam<REV>, Error_> {
            WithConstParam::try_new_or_recover_async_send(data, dref_builder)
                .await
                .map_err(|(error, _heads)| error)
        }
        /**(See also [`WithConstParamAsyncSendTryBuilder::try_build_or_recover()`](WithConstParamAsyncSendTryBuilder::try_build_or_recover).) Like [`try_new`](Self::try_new), but all [head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) are returned in the case of an error. The arguments are as follows:

| Argument | Suggested Use |
| --- | --- |
| `data` | Directly pass in the value this field should contain |
| `dref_builder` | Use a function or closure: `(data: &_) -> Result<dref: _, Error_>` |
*/
        pub(super) async fn try_new_or_recover_async_send<Error_>(
            data: Box<Vec<u64>>,
            dref_builder: impl for<'this> ::core::ops::FnOnce(
                &'this Box<Vec<u64>>,
            ) -> ::core::pin::Pin<
                    ::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<
                            Output = ::core::result::Result<
                                Option<&'this Box<Vec<u64>>>,
                                Error_,
                            >,
                        > + ::core::marker::Send + 'this,
                    >,
                >,
        ) -> ::core::result::Result<WithConstParam<REV>, (Error_, Heads<REV>)> {
            let data = ::ouroboros::macro_help::aliasable_boxed(data);
            let data_illegal_static_reference = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*data)
            };
            let dref = match dref_builder(data_illegal_static_reference).await {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(err) => {
                    return ::core::result::Result::Err((
                        err,
                        Heads {
                            data: ::ouroboros::macro_help::unbox(data),
                            _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
                        },
                    ));
                }
            };
            ::core::result::Result::Ok(unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(WithConstParamInternal {
                        data,
                        dref,
                    }),
                }
            })
        }
        ///Provides limited immutable access to `data`. This method was generated because the contents of `data` are immutably borrowed by other fields.
        #[inline(always)]
        pub(super) fn with_data<'outer_borrow, ReturnType>(
            &'outer_borrow self,
            user: impl for<'this> ::core::ops::FnOnce(
                &'outer_borrow Box<Vec<u64>>,
            ) -> ReturnType,
        ) -> ReturnType {
            let field = &unsafe { self.actual_data.assume_init_ref() }.data;
            user(field)
        }
        ///Provides limited immutable access to `data`. This method was generated because the contents of `data` are immutably borrowed by other fields.
        #[inline(always)]
        pub(super) fn borrow_data<'this>(&'this self) -> &'this Box<Vec<u64>> {
            &unsafe { self.actual_data.assume_init_ref() }.data
        }
        ///Provides an immutable reference to `dref`. This method was generated because `dref` is a [tail field](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions).
        #[inline(always)]
        pub(super) fn with_dref<'outer_borrow, ReturnType>(
            &'outer_borrow self,
            user: impl for<'this> ::core::ops::FnOnce(
                &'outer_borrow Option<&'this Box<Vec<u64>>>,
            ) -> ReturnType,
        ) -> ReturnType {
            let field = &unsafe { self.actual_data.assume_init_ref() }.dref;
            user(field)
        }
        ///Provides a mutable reference to `dref`. This method was generated because `dref` is a [tail field](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions). No `borrow_dref_mut` function was generated because Rust's borrow checker is currently unable to guarantee that such a method would be used safely.
        #[inline(always)]
        pub(super) fn with_dref_mut<'outer_borrow, ReturnType>(
            &'outer_borrow mut self,
            user: impl for<'this> ::core::ops::FnOnce(
                &'outer_borrow mut Option<&'this Box<Vec<u64>>>,
            ) -> ReturnType,
        ) -> ReturnType {
            let field = &mut unsafe { self.actual_data.assume_init_mut() }.dref;
            user(field)
        }
        ///This method provides immutable references to all [tail and immutably borrowed fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions).
        #[inline(always)]
        pub(super) fn with<'outer_borrow, ReturnType>(
            &'outer_borrow self,
            user: impl for<'this> ::core::ops::FnOnce(
                BorrowedFields<'outer_borrow, 'this, REV>,
            ) -> ReturnType,
        ) -> ReturnType {
            let this = unsafe { self.actual_data.assume_init_ref() };
            user(BorrowedFields {
                dref: &this.dref,
                data: unsafe { ::ouroboros::macro_help::change_lifetime(&*this.data) },
                _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
            })
        }
        ///This method provides mutable references to all [tail fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions).
        #[inline(always)]
        pub(super) fn with_mut<'outer_borrow, ReturnType>(
            &'outer_borrow mut self,
            user: impl for<'this0, 'this1> ::core::ops::FnOnce(
                BorrowedMutFields<'outer_borrow, 'this1, 'this0, REV>,
            ) -> ReturnType,
        ) -> ReturnType {
            let this = unsafe { self.actual_data.assume_init_mut() };
            user(BorrowedMutFields {
                dref: &mut this.dref,
                data: unsafe { ::ouroboros::macro_help::change_lifetime(&*this.data) },
                _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
            })
        }
        ///This function drops all internally referencing fields and returns only the [head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) of this struct.
        #[allow(clippy::drop_ref)]
        #[allow(clippy::drop_copy)]
        #[allow(clippy::drop_non_drop)]
        pub(super) fn into_heads(self) -> Heads<REV> {
            let this_ptr = &self as *const _;
            let this: WithConstParamInternal<REV> = unsafe {
                ::core::mem::transmute_copy(&*this_ptr)
            };
            ::core::mem::forget(self);
            ::core::mem::drop(this.dref);
            let data = this.data;
            Heads {
                data: ::ouroboros::macro_help::unbox(data),
                _comsume_template_const_parameter_rev: ::core::marker::PhantomData,
            }
        }
    }
    fn type_asserts<const REV: bool>() {
        ::ouroboros::macro_help::CheckIfTypeIsStd::<Box<Vec<u64>>>::is_std_box_type();
    }
}
use ouroboros_impl_with_const_param::WithConstParam;
use ouroboros_impl_with_const_param::WithConstParamBuilder;
use ouroboros_impl_with_const_param::WithConstParamAsyncBuilder;
use ouroboros_impl_with_const_param::WithConstParamAsyncSendBuilder;
use ouroboros_impl_with_const_param::WithConstParamTryBuilder;
use ouroboros_impl_with_const_param::WithConstParamAsyncTryBuilder;
use ouroboros_impl_with_const_param::WithConstParamAsyncSendTryBuilder;
