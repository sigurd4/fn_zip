//! Provides a zip trait for functions, allowing two functions to be combined at compile-time before being called.
//! This is equivalent to `core::future::join!()`, but lazy, and works for non-async functions.
//!
//! The resulting function takes the arguments of both functions and return a tuple.
//!
//! # Example
//!
//! ```rust
//! use fn_zip::FnZip;
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

#![no_std]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![feature(const_destruct)]
#![feature(async_fn_traits)]
#![feature(array_methods)]

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
    #[tokio::test]
    async fn test_async()
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
    }
}
