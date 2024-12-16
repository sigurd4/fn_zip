//! Combines two functions into one where the arguments are conjoined and the return-values are given in a tuple pair.
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

#![cfg_attr(all(not(test), not(feature = "par")), no_std)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![feature(associated_type_bounds)]
#![feature(const_mut_refs)]
#![feature(transmutability)]
#![feature(trait_alias)]
#![feature(cfg_accessible)]
#![feature(array_methods)]
#![feature(const_destruct)]
#![feature(async_fn_traits)]
#![feature(future_join)]
#![feature(impl_trait_in_assoc_type)]

moddef::moddef!(
    flat(pub) mod {
        zip,
        zipped_fn
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
        let ab = a.fn_zip(b); // (f32, u8) -> (f64, u8)

        let (x_a, x_b) = (4.0, 23);

        let (y_a, y_b) = ab(x_a, x_b);

        assert_eq!(y_a, a(x_a));
        assert_eq!(y_b, b(x_b));
    }

    #[tokio::test]
    async fn test_async()
    {
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

        // i don't know of any prettier way to call an async function...

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
