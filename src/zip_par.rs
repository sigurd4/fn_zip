use core::marker::Tuple;

use tupleops::TupleConcat;

use super::*;

/// Combines two functions into one where the arguments are conjoined and the return-values are given in a tuple pair.
/// 
/// The zipped result of this function will be executed in paralell.
/// 
/// # Example
/// 
/// ```rust
/// use fn_zip::FnZipPar;
/// 
/// fn a(x: f32) -> f64
/// {
///     (x as f64).sqrt()
/// }
/// fn b(x: u8) -> u8
/// {
///     x + 1
/// }
/// let ab = a.fn_zip_par(b); // (f32, u8) -> Result<(f64, u8), (bool, ParError)>
/// 
/// let (x_a, x_b) = (4.0, 23);
/// 
/// let (y_a, y_b) = ab(x_a, x_b).unwrap();
/// 
/// assert_eq!(y_a, a(x_a));
/// assert_eq!(y_b, b(x_b));
/// ```
#[const_trait]
pub trait FnZipPar<RX, LX, Rhs>: FnZip<RX, LX, Rhs>
{
    type OutputPar;

    fn fn_zip_par(self, rhs: Rhs) -> <Self as FnZipPar<RX, LX, Rhs>>::OutputPar;
}

impl<RX, LX, LF, RF> const FnZipPar<RX, LX, RF> for LF
where
    LX: Tuple + Send,
    RX: Tuple + Send,
    LF: FnOnce<LX, Output: Send> + Send,
    RF: FnOnce<RX, Output: Send> + Send,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    type OutputPar = ZippedFnPar<LX, RX, LF, RF>;

    fn fn_zip_par(self, rhs: RF) -> <Self as FnZipPar<RX, LX, RF>>::OutputPar
    {
        ZippedFnPar::from(self.fn_zip(rhs))
    }
}