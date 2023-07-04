
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
#![feature(array_zip)]
#![feature(const_default_impls)]

moddef::moddef!(
    flat(pub) mod {
        zip,
        zipped_fn,
        zip_par for cfg(feature = "par"),
        zipped_fn_par for cfg(feature = "par")
    }
);

pub mod private
{
    use core::{marker::Tuple, mem::ManuallyDrop};

    use tupleops::{ConcatTuples, TupleConcat};

    union TupleConcatTransmutation<L, R>
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        split: ManuallyDrop<(ManuallyDrop<L>, ManuallyDrop<R>)>,
        concat: ManuallyDrop<ConcatTuples<L, R>>
    }

    pub const fn tuple_split_const_hold<L, R>(tuple: ConcatTuples<L, R>) -> (ManuallyDrop<L>, ManuallyDrop<R>)
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        unsafe {
            ManuallyDrop::into_inner(TupleConcatTransmutation
            {
                concat: ManuallyDrop::new(tuple)
            }.split)
        }
    }

    pub const fn tuple_split_const<L, R>(tuple: ConcatTuples<L, R>) -> (L, R)
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        let (left, right) = tuple_split_const_hold(tuple);
        (ManuallyDrop::into_inner(left), ManuallyDrop::into_inner(right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
