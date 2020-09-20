//! A crate for creating safe self-referencing structs.
//!
//! See the documentation for [`#[self_referencing]`](self_referencing) to get started.
//! See the documentation of [`ouroboros_examples`](https://docs.rs/ouroboros_examples) for
//! sample documentation of structs which have had the macro applied to them.

/// This macro is used to turn a regular struct into a self-referencing one. An example:
/// ```rust
/// use ouroboros::self_referencing;
///
/// #[self_referencing]
/// struct MyStruct {
///     int_data: Box<i32>,
///     float_data: Box<f32>,
///     #[borrows(int_data)]
///     int_reference: &'this i32,
///     #[borrows(mut float_data)]
///     float_reference: &'this mut f32,
/// }
///
/// fn main() {
///     let mut my_value = MyStructBuilder {
///         int_data: Box::new(42),
///         float_data: Box::new(3.14),
///         int_reference_builder: |int_data: &i32| int_data,
///         float_reference_builder: |float_data: &mut f32| float_data,
///     }.build();
///
///     // Prints 42
///     println!("{:?}", my_value.use_int_data_contents(|int_data| *int_data));
///     // Prints 3.14
///     println!("{:?}", my_value.use_float_reference(|float_reference| **float_reference));
///     // Sets the value of float_data to 84.0
///     my_value.use_all_fields_mut(|fields| {
///         **fields.float_reference = (**fields.int_reference as f32) * 2.0;
///     });
///
///     // We can hold on to this reference...
///     let int_ref = my_value.use_int_reference(|int_ref| *int_ref);
///     println!("{:?}", *int_ref);
///     // As long as the struct is still alive.
///     drop(my_value);
///     // This will cause an error!
///     // println!("{:?}", *int_ref);
/// }
/// ```
/// To explain the features and limitations of this crate, some definitions are necessary:
/// # Definitions:
/// - **immutably borrowed field**: a field which is immutably borrowed by at least one other field.
/// - **mutably borrowed field**: a field which is mutably borrowed by exactly one other field.
/// - **self-referencing field**: a field which borrows at least one other field.
/// - **head field**: a field which does not borrow any other fields, I.E. not self-referencing.
/// - **tail field**: a field which is not borrowed by any other fields.
///
/// To make a self-referencing struct, you must write a struct definition and place
/// `#[self_referencing]` on top. For every field that borrows other fields, you must place
/// `#[borrows()]` on top and place inside the parenthesis a list of fields that it borrows. Mut can
/// be prefixed to indicate that a mutable borrow is required. For example,
/// `#[borrows(a, b, mut c)]` indicates that the first two fields need to be borrowed immutably and
/// the third needs to be borrowed mutably.
/// # You must comply with these limitations:
/// - Fields must be declared before the first time they are borrowed.
/// - Normal borrowing rules apply, E.G. a field cannot be borrowed mutably twice.
/// - Fields that are borrowed must be of a data type that implement
///   [`StableDeref`](https://docs.rs/stable_deref_trait/1.2.0/stable_deref_trait/trait.StableDeref.html).
///   Normally this just means `Box<T>`.
///
/// Violating them will result in a compile error.
/// # What does the macro generate?
/// The `#[self_referencing]` struct will replace your definition with an unsafe self-referencing
/// struct with a safe public interface. Many functions will be generated depending on your original
/// struct definition. Documentation is generated for all items, so building documentation for
/// your project allows accessing detailed information about available functions. The following
/// is an overview of what is generated:
/// ### `MyStruct::new(fields...) -> MyStruct`
/// A basic constructor. It accepts values for each field in the order you declared them in. For
/// **head fields**, you only need to pass in what value it should have and it will be moved in
/// to the output. For **self-referencing fields**, you must provide a function or closure which creates
/// the value based on the values it borrows. A field using the earlier example of
/// `#[borrow(a, b, mut c)]` would require a function typed as
/// `FnOnce(a: &_, b: &_, c: &mut _) -> _`.
/// ### `MyStructBuilder`
/// This is the preferred way to create a new instance of your struct. It is similar to using the
/// `MyStruct { a, b, c, d }` syntax instead of `MyStruct::new(a, b, c, d)`. It contains one field
/// for every argument in the actual constructor. **Head fields** have the same name that you
/// originally defined them with. **self-referencing fields** are suffixed with `_builder` since you need
/// to provide a function instead of a value. Calling `.build()` on an instance of `MyStructBuilder`
/// will convert it to an instance of `MyStruct`.
/// ### `MyStruct::try_new<E>(fields...) -> Result<MyStruct, E>`
/// Similar to the regular `new()` function, except the functions wich create values for all
/// **self-referencing fields** can return `Result<>`s. If any of those are `Err`s, that error will be
/// returned instead of an instance of `MyStruct`. The preferred way to use this function is through
/// `MyStructTryBuilder` and its `try_build()` function.
/// ### `MyStruct::try_new_or_recover<E>(fields...) -> Result<MyStruct, (E, Heads)>`
/// Similar to the `try_new()` function, except that all the **head fields** are returned along side
/// the original error in case of an error. The preferred way to use this function is through
/// `MyStructTryBuilder` and its `try_build_or_recover()` function.
/// ### `MyStruct::use_FIELD<R>(&self, user: FnOnce(field: &FieldType) -> R) -> R`
/// This function is generated for every **tail field** in your struct. It allows safely accessing
/// a reference to that value. The function generates the reference and passes it to `user`. You
/// can do anything you want with the reference, it is constructed to not outlive the struct.
/// ### `MyStruct::use_FIELD_mut<R>(&mut self, user: FnOnce(field: &mut FieldType) -> R) -> R`
/// This function is generated for every **tail field** in your struct. It is the mutable version
/// of `use_FIELD`.
/// ### `MyStruct::use_FIELD_contents<R>(&self, user: FnOnce(data: &<FieldType as Deref>::Target) -> R) -> R`
/// This function is generated for every **immutably borrowed field** In your struct. It allows
/// accessing the contents of that field. It is similar to `use_FIELD` except that it provides
/// a reference to the field's content, not the field itself. E.G. a field of type `Box<i32>` would
/// cause this function to provide a reference of type `&i32`. There is no mutable version of this
/// function because if a field is already borrowed, it cannot be mutably borrowed safely.
/// ### `MyStruct::use_all_fields<R>(&self, user: FnOnce(fields: AllFields) -> R) -> R`
/// Allows borrowing all **tail and immutably-borrowed fields** at once. Functions similarly to
/// `use_FIELD`.
/// ### `MyStruct::use_all_fields_mut<R>(&self, user: FnOnce(fields: AllFields) -> R) -> R`
/// Allows mutably borrowing all **tail fields** at once. Functions similarly to `use_FIELD_mut`.
/// ### `MyStruct::into_heads(self) -> Heads`
/// Drops all self-referencing fields and returns a struct containing all **head fields**.
pub use ouroboros_macro::self_referencing;

#[doc(hidden)]
pub mod macro_help {
    use stable_deref_trait::StableDeref;
    use std::ops::DerefMut;

    /// Converts a reference to an object implementing Deref to a static reference to the data it
    /// Derefs to. This is obviously unsafe because the compiler can no longer guarantee that the
    /// data outlives the reference. This function is templated to only work for containers that
    /// implement StableDeref, E.G. Box and Rc. The intent is that the data that is being pointed
    /// to will never move as long as the container itself is not dropped. It is up to the consumer
    /// to get rid of the reference before the container is dropped. The + 'static ensures that
    /// whatever we are referring to will remain valid indefinitely, that there are no limitations
    /// on how long the pointer itself can live.
    pub unsafe fn stable_deref_and_strip_lifetime<T: StableDeref + 'static>(
        data: &T,
    ) -> &'static T::Target {
        &*((&**data) as *const _)
    }

    /// Like stable_deref_and_strip_lifetime, but for mutable references.
    pub unsafe fn stable_deref_and_strip_lifetime_mut<T: StableDeref + DerefMut + 'static>(
        data: &mut T,
    ) -> &'static mut T::Target {
        &mut *((&mut **data) as *mut _)
    }
}
