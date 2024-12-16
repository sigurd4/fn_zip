use core::{future::Future, pin::Pin, task::{Context, Poll}};

/// A pair of joined futures.
/// 
/// This is really only for use with the `ZippedFn` struct.
/// If you need to join threads normally, use the `core::future::join!` macro.
/// 
/// # Example
/// 
/// ```rust
/// #![feature(fn_traits)]
/// #![feature(async_fn_traits)]
/// 
/// use fn_zip::*;
/// use core::ops::AsyncFn;
/// 
/// async fn a(x: f32) -> f64
/// {
///     (x as f64).sqrt()
/// }
/// async fn b(x: u8) -> u8
/// {
///     x + 1
/// }
/// 
/// let ab = a.fn_zip(b);
/// let (x_a, x_b) = (4.0, 23);
/// 
/// # tokio_test::block_on(async {
/// // I don't know of any prettier way to call an async function...
/// 
/// let (y_a, y_b) = ab.async_call((x_a, x_b)).await;
/// 
/// assert_eq!(y_a, a(x_a).await);
/// assert_eq!(y_b, b(x_b).await);
/// # })
/// ```
pub struct JoinedPair<L, R>
where
    L: Future,
    R: Future
{
    left: MaybeDone<L>,
    right: MaybeDone<R>
}

impl<L, R> JoinedPair<L, R>
where
    L: Future,
    R: Future
{
    pub fn new(left: L, right: R) -> Self
    {
        Self {
            left: MaybeDone::Future(left),
            right: MaybeDone::Future(right)
        }
    }
}

impl<L, R> Future for JoinedPair<L, R>
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
    
enum MaybeDone<F: Future>
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