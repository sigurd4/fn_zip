use core::{marker::{Tuple, PhantomData, Destruct}, mem::ManuallyDrop};

use tupleops::{TupleConcat, ConcatTuples};

/// The result of zipping two functions together using [FnZip::fn_zip](FnZip::fn_zip).
/// 
/// Can be called as if a function, using the arguments of both zipped functions in sequence.
/// 
/// May implement Fn and FnMut if both zipped functions qualify, but will always implement FnOnce.
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
pub struct ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: FnOnce<LX>,
    RF: FnOnce<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    pub(crate) left: LF,
    pub(crate) right: RF,
    pub(crate) phantom: PhantomData<(LX, RX)>
}

impl<LX, RX, LF, RF> const FnOnce<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: ~const FnOnce<LX> + ~const Destruct,
    RF: ~const FnOnce<RX> + ~const Destruct,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    type Output = (LF::Output, RF::Output);

    extern "rust-call" fn call_once(self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        let (args_left, args_right) = private::tuple_split_const_hold(args);
        (self.left.call_once(ManuallyDrop::into_inner(args_left)), self.right.call_once(ManuallyDrop::into_inner(args_right)))
    }
}

impl<LX, RX, LF, RF> const FnMut<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: ~const FnMut<LX> + ~const Destruct,
    RF: ~const FnMut<RX> + ~const Destruct,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    extern "rust-call" fn call_mut(&mut self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        let (args_left, args_right) = private::tuple_split_const_hold(args);
        (self.left.call_mut(ManuallyDrop::into_inner(args_left)), self.right.call_mut(ManuallyDrop::into_inner(args_right)))
    }
}

impl<LX, RX, LF, RF> const Fn<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: ~const Fn<LX> + ~const Destruct,
    RF: ~const Fn<RX> + ~const Destruct,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    extern "rust-call" fn call(&self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        let (args_left, args_right) = private::tuple_split_const_hold(args);
        (self.left.call(ManuallyDrop::into_inner(args_left)), self.right.call(ManuallyDrop::into_inner(args_right)))
    }
}

mod private
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

    /*pub const fn tuple_split_const<L, R>(tuple: ConcatTuples<L, R>) -> (L, R)
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        let (left, right) = tuple_split_const_hold(tuple);
        (ManuallyDrop::into_inner(left), ManuallyDrop::into_inner(right))
    }*/
}