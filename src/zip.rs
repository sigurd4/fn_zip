use core::marker::Tuple;

use tupleops::TupleConcat;

use super::*;

/// Combines two functions into one where the arguments are conjoined and the return-values are given in a tuple pair.
/// 
/// # Example
/// 
/// ```rust
/// use fn_zip::FnZip;
/// 
/// fn a(x: f32) -> f64
/// {
///     (x as f64).sqrt()
/// }
/// fn b(x: u8) -> u8
/// {
///     x + 1
/// }
/// let ab = a.fn_zip(b); // (f32, u8) -> (f64, u8)
/// 
/// let (x_a, x_b) = (4.0, 23);
/// 
/// let (y_a, y_b) = ab(x_a, x_b);
/// 
/// assert_eq!(y_a, a(x_a));
/// assert_eq!(y_b, b(x_b));
/// ```
#[const_trait]
pub trait FnZip<RX, LX, Rhs>
{
    type Output;

    fn fn_zip_once(self, rhs: Rhs) -> Self::Output;
    fn fn_zip_mut<'a>(&'a mut self, rhs: Rhs) -> <&'a mut Self as FnZip<RX, LX, Rhs>>::Output
    where
        &'a mut Self: ~const FnZip<RX, LX, Rhs>
    {
        self.fn_zip_once(rhs)
    }
    fn fn_zip<'a>(&'a self, rhs: Rhs) -> <&'a Self as FnZip<RX, LX, Rhs>>::Output
    where
        &'a Self: ~const FnZip<RX, LX, Rhs>
    {
        self.fn_zip_once(rhs)
    }
}

impl<RX, LX, LF, RF> const FnZip<RX, LX, RF> for LF
where
    LX: Tuple,
    RX: Tuple,
    LF: FnOnce<LX>,
    RF: FnOnce<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    type Output = ZippedFn<LX, RX, LF, RF>;

    fn fn_zip_once(self, rhs: RF) -> Self::Output
    {
        ZippedFn::new(self, rhs)
    }
}