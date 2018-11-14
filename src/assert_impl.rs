/// Asserts that the type implements the given traits.
///
/// This can be used to ensure types implement auto traits such as [`Send`] and
/// [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// # Examples
///
/// On stable Rust, using the macro requires a unique “label” when used in a
/// module scope:
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {}
/// assert_impl!(str; String, Send, Sync, From<&'static str>);
/// assert_impl!(vec; &'static [u8], Into<Vec<u8>>);
/// ```
///
/// The [labeling limitation](index.html#limitations) is not necessary if
/// compiling on nightly Rust with the `nightly` feature enabled:
///
#[cfg_attr(feature = "nightly", doc = "```")]
#[cfg_attr(not(feature = "nightly"), doc = "```ignore")]
/// #![feature(underscore_const_names)]
/// # #[macro_use] extern crate static_assertions;
///
/// assert_impl!(u32, Copy, Send);
///
/// fn main() {
///     assert_impl!(&str, Into<String>);
/// }
/// ```
///
/// Raw pointers cannot be sent between threads safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_impl!(*const u8, Send);
/// # }
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [blanket]: https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export]
macro_rules! assert_impl {
    ($($xs:tt)+) => { _assert_impl!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export]
macro_rules! _assert_impl {
    ($x:ty, $($t:path),+ $(,)*) => {
        const _: fn() -> () = || {
            fn assert_impl<T>() where T: ?Sized $(+ $t)+ {}
            assert_impl::<$x>();
        };
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export]
macro_rules! _assert_impl {
    ($x:ty, $($t:path),+ $(,)*) => {
        {
            fn assert_impl<T>() where T: ?Sized $(+ $t)+ {}
            assert_impl::<$x>();
        }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_impl!($($xs)+); }
    };
}
