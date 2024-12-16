use core::{cell::RefCell, future::{poll_fn, Future}, marker::{Destruct, PhantomData, PhantomPinned, Tuple}, mem::ManuallyDrop, pin::Pin, task::{Context, Poll}};

use core::ops::{AsyncFnOnce, AsyncFnMut, AsyncFn};

use tuple_split::TupleSplitInto;
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
    RX: Tuple
{
    pub left: LF,
    pub right: RF,
    marker: PhantomData<(LX, RX)>
}

impl<LX, RX, LF, RF> ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple
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
    type CallOnceFuture = Join<<LF as AsyncFnOnce<LX>>::CallOnceFuture, <RF as AsyncFnOnce<RX>>::CallOnceFuture>;

    extern "rust-call" fn async_call_once(self, args: ConcatTuples<LX, RX>) -> Self::CallOnceFuture
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        Join {
            left: private::MaybeDone::Future(self.left.async_call_once(args_left)),
            right: private::MaybeDone::Future(self.right.async_call_once(args_right))
        }
    }
}

impl<LX, RX, LF, RF> /*const*/ AsyncFnMut<ConcatTuples<LX, RX>> for ZippedFn<LX, RX, LF, RF>
where
    LX: Tuple,
    RX: Tuple,
    LF: /*~const*/ AsyncFnMut<LX>,
    RF: /*~const*/ AsyncFnMut<RX>,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>,
    ConcatTuples<LX, RX>: TupleSplitInto<LX, RX>
{
    type CallRefFuture<'a> = Join<<LF as AsyncFnMut<LX>>::CallRefFuture<'a>, <RF as AsyncFnMut<RX>>::CallRefFuture<'a>>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(&mut self, args: ConcatTuples<LX, RX>) -> Self::CallRefFuture<'_>
    {
        let (args_left, args_right) = tuple_split::split_tuple_into(args);
        Join {
            left: private::MaybeDone::Future(self.left.async_call_mut(args_left)),
            right: private::MaybeDone::Future(self.right.async_call_mut(args_right))
        }
    }
}

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
        Join {
            left: private::MaybeDone::Future(self.left.async_call(args_left)),
            right: private::MaybeDone::Future(self.right.async_call(args_right))
        }
    }
}

/// A pair of joined futures
pub struct Join<L, R>
where
    L: Future,
    R: Future
{
    left: private::MaybeDone<L>,
    right: private::MaybeDone<R>
}

impl<L, R> Future for Join<L, R>
where
    L: Future,
    R: Future
{
    type Output = (<L as Future>::Output, <R as Future>::Output);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        // This is pretty much the code for the `core::future::join!` macro made  limited to only two futures.
        if unsafe {
            !self.as_mut()
                .map_unchecked_mut(|join| &mut join.left)
                .poll(cx)
                .is_ready()
                || !self.as_mut()
                .map_unchecked_mut(|join| &mut join.right)
                .poll(cx)
                .is_ready()
            }
        {
            return Poll::Pending
        }

        let join = unsafe {
            self.as_mut()
                .get_unchecked_mut()
        };

        Poll::Ready((
            join.left.take_output().unwrap(),
            join.right.take_output().unwrap()
        ))
    }
}

mod private
{
    use core::{future::Future, pin::Pin, task::{Context, Poll}};
    
    pub enum MaybeDone<F: Future>
    {
        Future(F),
        Done(F::Output),
        Taken,
    }
    
    impl<F: Future> MaybeDone<F>
    {
        pub fn take_output(&mut self) -> Option<F::Output>
        {
            match *self
            {
                MaybeDone::Done(_) => match core::mem::replace(self, Self::Taken)
                {
                    MaybeDone::Done(val) => Some(val),
                    _ => unreachable!(),
                },
                _ => None,
            }
        }
    }
    
    impl<F: Future> Future for MaybeDone<F>
    {
        type Output = ();
    
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
        {
            // SAFETY: pinning in structural for `f`
            unsafe {
                // Do not mix match ergonomics with unsafe.
                match *self.as_mut().get_unchecked_mut()
                {
                    MaybeDone::Future(ref mut f) => {
                        let val = core::task::ready!(Pin::new_unchecked(f).poll(cx));
                        self.set(Self::Done(val));
                    }
                    MaybeDone::Done(_) => {}
                    MaybeDone::Taken => unreachable!(),
                }
            }
    
            Poll::Ready(())
        }
    }    
}