#![no_std]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![cfg_attr(feature = "async", feature(async_fn_traits))]

//! Provides a zip trait for functions, allowing two functions to be combined at compile-time before being called.
//! This is equivalent to `core::future::join!()`, but lazy, and works for non-async functions.
//!
//! The resulting function takes the arguments of both functions and return a tuple.
//!
//! # Example
//!
//! ```rust
//! use fn_zip::*;
//!
//! fn a(x: f32) -> f64
//! {
//!     (x as f64).sqrt()
//! }
//! fn b(x: u8) -> u8
//! {
//!     x + 1
//! }
//! let ab = a.fn_zip(b); // (f32, u8) -> (f64, u8)
//!
//! let (x_a, x_b) = (4.0, 23);
//!
//! let (y_a, y_b) = ab(x_a, x_b);
//!
//! assert_eq!(y_a, a(x_a));
//! assert_eq!(y_b, b(x_b));
//! ```
//! 
//! # Async
//! 
//! The zipped functions can also implement [AsyncFnOnce](core::ops::AsyncFnOnce), [AsyncFnMut](core::ops::AsyncFnMut) and [AsyncFn](core::ops::AsyncFn) if both functions qualify.
//! 
//! This is an experimental feature, since it just recently (as of writing) got added to the rust core library on rust-nightly, and may be subject to change at any point. Enable it with feature `async` or `experimental`.
//! 
#![cfg_attr(feature = "async", doc = r##"
```rust
#![feature(fn_traits)]
#![feature(async_fn_traits)]

use fn_zip::*;
use core::ops::AsyncFn;
 
async fn a(x: f32) -> f64
{
    (x as f64).sqrt()
}
async fn b(x: u8) -> u8
{
    x + 1
}

let ab = a.fn_zip(b);
let (x_a, x_b) = (4.0, 23);

# tokio_test::block_on(async {
// I don't know of any prettier way to call an async function...

let (y_a, y_b) = ab.async_call((x_a, x_b)).await;

assert_eq!(y_a, a(x_a).await);
assert_eq!(y_b, b(x_b).await);
# })
```"##)]
//! 
//! Independent of this feature, it's still possible to zip two asyncronous functions normally, but their futures will not be joined.
//! 
//! # Compile time function zipping
//! 
//! Functions can also be zipped during compile-time.
//! 
//! ```rust
//! #![feature(const_trait_impl)]
//! 
//! use fn_zip::*;
//! 
//! fn a(x: f32) -> f64
//! {
//!     (x as f64).sqrt()
//! }
//! fn b(x: u8) -> u8
//! {
//!     x + 1
//! }
//! 
//! // Corce functions into function pointers
//! const A: fn(f32) -> f64 = a;
//! const B: fn(u8) -> u8 = b;
//! 
//! // Zip during compile time
//! const AB: ZippedFn<(f32,), (u8,), fn(f32) -> f64, fn(u8) -> u8> = A.fn_zip_once(B);
//! 
//! let (x_a, x_b) = (4.0, 23);
//! let (y_a, y_b) = AB(x_a, x_b);
//! 
//! assert_eq!(y_a, a(x_a));
//! assert_eq!(y_b, b(x_b));
//! ```
//! 
//! # Tuple sizes
//! 
//! By default, this crate operates with function pairs of up to 16 arguments combined, and splits them up in the form of tuples. If you want to use differently sized tuples, use the features `8`, `16`, `32`, `64`, `96`, `128`, `160`, `192`, `224` or `256` to set the maximum supported tuple size.
//! 
//! The `dont_hurt_yourself_by_using_all_features` is there to prevent usage of tuples bigger than 8 if `cargo` is ran with the flag `--all-features`. Using a tuple size above 16 is highly discouraged as it will make compilation time unbearably long. Compilation time will increase exponentially. You have been warned.

moddef::moddef!(
    flat(pub) mod {
        zip,
        zipped_fn,
        join for cfg(feature = "async")
    }
);

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        fn a(x: f32) -> f64
        {
            (x as f64).sqrt()
        }
        fn b(x: u8) -> u8
        {
            x + 1
        }

        let mut ab = a.fn_zip(b);
        let (x_a, x_b) = (4.0, 23);

        let (y_a, y_b) = ab(x_a, x_b);

        assert_eq!(y_a, a(x_a));
        assert_eq!(y_b, b(x_b));

        let ab = &mut ab;

        let (y_a, y_b) = ab(x_a, x_b);

        assert_eq!(y_a, a(x_a));
        assert_eq!(y_b, b(x_b));

        let ab = &*ab;

        let (y_a, y_b) = ab(x_a, x_b);

        assert_eq!(y_a, a(x_a));
        assert_eq!(y_b, b(x_b));
    }

    #[cfg(feature = "async")]
    #[test]
    fn test_async()
    {
        use core::ops::{AsyncFn, AsyncFnMut, AsyncFnOnce};

        async fn a(x: f32) -> f64
        {
            (x as f64).sqrt()
        }
        async fn b(x: u8) -> u8
        {
            x + 1
        }

        let mut ab = a.fn_zip(b);
        let (x_a, x_b) = (4.0, 23);

        tokio_test::block_on(async {
            // I don't know of any prettier way to call an async function...

            let (y_a, y_b) = ab.async_call((x_a, x_b)).await;

            assert_eq!(y_a, a(x_a).await);
            assert_eq!(y_b, b(x_b).await);

            let (y_a, y_b) = ab.async_call_mut((x_a, x_b)).await;

            assert_eq!(y_a, a(x_a).await);
            assert_eq!(y_b, b(x_b).await);

            let (y_a, y_b) = ab.async_call_once((x_a, x_b)).await;

            assert_eq!(y_a, a(x_a).await);
            assert_eq!(y_b, b(x_b).await);
        });
    }

    #[test]
    fn test_const()
    {
        fn a(x: f32) -> f64
        {
            (x as f64).sqrt()
        }
        fn b(x: u8) -> u8
        {
            x + 1
        }

        // Corce functions into function pointers
        const A: fn(f32) -> f64 = a;
        const B: fn(u8) -> u8 = b;

        // Zip during compile time
        const AB: ZippedFn<(f32,), (u8,), fn(f32) -> f64, fn(u8) -> u8> = A.fn_zip_once(B);
        let (x_a, x_b) = (4.0, 23);

        let (y_a, y_b) = AB(x_a, x_b);

        assert_eq!(y_a, a(x_a));
        assert_eq!(y_b, b(x_b));
    }
}
