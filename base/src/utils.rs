#[doc(hidden)]
pub(crate) mod private {
    pub trait Private {}
    impl Private for () {}
}
/// Put this macro inside a trait to seal it i.e. prevent extern implementations.
#[macro_export]
macro_rules! sealed {
    (trait) => {
        /// This method prohibits implementation of this trait out side of its defining crate.
        fn _not_implementable<Private: $crate::utils::private::Private>();
    };
    (impl) => {
        fn _not_implementable<Private: $crate::utils::private::Private>() {}
    };
}
