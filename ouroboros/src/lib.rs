pub use ouroboros_macro::*;

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
