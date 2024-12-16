use core::marker::{PhantomData, Tuple};


use tuple_split::TupleSplitInto;
use tupleops::{TupleConcat, ConcatTuples};

#[cfg(feature = "async")]
use crate::JoinedPair;
#[cfg(feature = "async")]
use core::ops::{AsyncFnOnce, AsyncFnMut, AsyncFn};

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
    RX: Tuple
{
    pub left: LF,
    pub right: RF,
    marker: PhantomData<(LX, RX)>
}

impl<LX, RX, LF, RF> ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: FnOnce<LX>,
    RF: FnOnce<RX>
{
    pub const fn new(left: LF, right: RF) -> Self
    {
        Self {
            left,
            right,
            marker: PhantomData
        }
    }
}

impl<LX, RX, LF, RF> /*const*/ FnOnce<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ FnOnce<LX>,
    RF: /*~const*/ FnOnce<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    type Output = (LF::Output, RF::Output);

    extern "rust-call" fn call_once(self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        (self.left.call_once(args_left), self.right.call_once(args_right))
    }
}

impl<LX, RX, LF, RF> /*const*/ FnMut<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ FnMut<LX>,
    RF: /*~const*/ FnMut<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    extern "rust-call" fn call_mut(&mut self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        (self.left.call_mut(args_left), self.right.call_mut(args_right))
    }
}

impl<LX, RX, LF, RF> /*const*/ Fn<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ Fn<LX>,
    RF: /*~const*/ Fn<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    extern "rust-call" fn call(&self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        (self.left.call(args_left), self.right.call(args_right))
    }
}

#[cfg(feature = "async")]
impl<LX, RX, LF, RF> /*const*/ AsyncFnOnce<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ AsyncFnOnce<LX>,
    RF: /*~const*/ AsyncFnOnce<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    type Output = (LF::Output, RF::Output);
    type CallOnceFuture = JoinedPair<<LF as AsyncFnOnce<LX>>::CallOnceFuture, <RF as AsyncFnOnce<RX>>::CallOnceFuture>;

    extern "rust-call" fn async_call_once(self, args: ConcatTuples<LX, RX>) -> Self::CallOnceFuture
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        JoinedPair::new(
            self.left.async_call_once(args_left),
            self.right.async_call_once(args_right)
        )
    }
}

#[cfg(feature = "async")]
impl<LX, RX, LF, RF> /*const*/ AsyncFnMut<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ AsyncFnMut<LX>,
    RF: /*~const*/ AsyncFnMut<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    type CallRefFuture<'a> = JoinedPair<<LF as AsyncFnMut<LX>>::CallRefFuture<'a>, <RF as AsyncFnMut<RX>>::CallRefFuture<'a>>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(&mut self, args: ConcatTuples<LX, RX>) -> Self::CallRefFuture<'_>
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        JoinedPair::new(
            self.left.async_call_mut(args_left),
            self.right.async_call_mut(args_right)
        )
    }
}

#[cfg(feature = "async")]
impl<LX, RX, LF, RF> /*const*/ AsyncFn<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ AsyncFn<LX>,
    RF: /*~const*/ AsyncFn<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    extern "rust-call" fn async_call(&self, args: ConcatTuples<LX, RX>) -> Self::CallRefFuture<'_>
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        JoinedPair::new(
            self.left.async_call(args_left),
            self.right.async_call(args_right)
        )
    }
}