use core::{marker::{Tuple}, any::Any};

use tupleops::{TupleConcat, ConcatTuples};

use super::*;

/// The result of zipping two functions together using [FnZipPar::fn_zip_par](FnZipPar::fn_zip_par).
/// 
/// Can be called as if a function, using the arguments of both zipped functions in sequence.
/// 
/// May implement Fn and FnMut if both zipped functions qualify, but will always implement FnOnce.
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
pub struct ZippedFnPar<LX, RX, LF, RF>
where
    LX: Tuple + Send,
    RX: Tuple + Send,
    LF: FnOnce<LX, Output: Send> + Send,
    RF: FnOnce<RX, Output: Send> + Send,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    zipped: ZippedFn<LX, RX, LF, RF>,
    pub thread_names: [Option<String>; 2],
    pub thread_stack_sizes: [Option<usize>; 2]
}

impl<LX, RX, LF, RF> ZippedFnPar<LX, RX, LF, RF>
where
    LX: Tuple + Send,
    RX: Tuple + Send,
    LF: FnOnce<LX, Output: Send> + Send,
    RF: FnOnce<RX, Output: Send> + Send,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    pub const fn from(zipped: ZippedFn<LX, RX, LF, RF>) -> Self
    {
        Self
        {
            zipped,
            thread_names: Default::default(),
            thread_stack_sizes: Default::default()
        }
    }
}
impl<LX, RX, LF, RF> const From<ZippedFn<LX, RX, LF, RF>> for ZippedFnPar<LX, RX, LF, RF>
where
    LX: Tuple + Send,
    RX: Tuple + Send,
    LF: FnOnce<LX, Output: Send> + Send,
    RF: FnOnce<RX, Output: Send> + Send,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    fn from(zipped: ZippedFn<LX, RX, LF, RF>) -> Self
    {
        Self::from(zipped)
    }
}

impl<LX, RX, LF, RF> FnOnce<ConcatTuples<LX, RX>> for ZippedFnPar<LX, RX, LF, RF>
where
    LX: Tuple + Send,
    RX: Tuple + Send,
    LF: FnOnce<LX, Output: Send> + Send,
    RF: FnOnce<RX, Output: Send> + Send,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    type Output = Result<(LF::Output, RF::Output), (bool, ParError)>;

    extern "rust-call" fn call_once(mut self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        use std::thread::Builder;
        
        let (args_left, args_right) = private::tuple_split_const(args);

        let mut builders = [Builder::new(), Builder::new()]
        .zip(self.thread_names.each_mut()
            .zip(self.thread_stack_sizes.each_mut())
        ).map(|(mut builder, (name, stack_size))| {
            if let Some(name) = name.take()
            {
                builder = builder.name(name);
            }
            if let Some(stack_size) = stack_size.take()
            {
                builder = builder.stack_size(stack_size);
            }
            builder
        }).into_iter();

        std::thread::scope(|scope| {
            let (handle_left, handle_right) = (
                builders.next().unwrap().spawn_scoped(scope, || self.zipped.left.call_once(args_left))
                    .map_err(|err| (false, ParError::SpawnThreadError(err)))?,
                builders.next().unwrap().spawn_scoped(scope, || self.zipped.right.call_once(args_right))
                    .map_err(|err| (true, ParError::SpawnThreadError(err)))?
            );
    
            Ok((
                handle_left.join().map_err(|err| (false, ParError::JoinThreadError(err)))?,
                handle_right.join().map_err(|err| (true, ParError::JoinThreadError(err)))?
            ))
        })
    }
}

impl<LX, RX, LF, RF> FnMut<ConcatTuples<LX, RX>> for ZippedFnPar<LX, RX, LF, RF>
where
    LX: Tuple + Send,
    RX: Tuple + Send,
    LF: FnMut<LX, Output: Send> + Send,
    RF: FnMut<RX, Output: Send> + Send,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    extern "rust-call" fn call_mut(&mut self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        use std::thread::Builder;
        
        let (args_left, args_right) = private::tuple_split_const(args);

        let mut builders = [Builder::new(), Builder::new()]
        .zip(self.thread_names.each_ref()
            .zip(self.thread_stack_sizes.each_ref())
        ).map(|(mut builder, (name, stack_size))| {
            if let Some(name) = name
            {
                builder = builder.name(name.clone());
            }
            if let Some(stack_size) = stack_size
            {
                builder = builder.stack_size(*stack_size);
            }
            builder
        }).into_iter();

        std::thread::scope(|scope| {
            let (handle_left, handle_right) = (
                builders.next().unwrap().spawn_scoped(scope, || self.zipped.left.call_mut(args_left))
                    .map_err(|err| (false, ParError::SpawnThreadError(err)))?,
                builders.next().unwrap().spawn_scoped(scope, || self.zipped.right.call_mut(args_right))
                    .map_err(|err| (true, ParError::SpawnThreadError(err)))?
            );
    
            Ok((
                handle_left.join().map_err(|err| (false, ParError::JoinThreadError(err)))?,
                handle_right.join().map_err(|err| (true, ParError::JoinThreadError(err)))?
            ))
        })
    }
}

impl<LX, RX, LF, RF> Fn<ConcatTuples<LX, RX>> for ZippedFnPar<LX, RX, LF, RF>
where
    LX: Tuple + Send + Sync,
    RX: Tuple + Send + Sync,
    LF: Fn<LX, Output: Send> + Send + Sync,
    RF: Fn<RX, Output: Send> + Send + Sync,
    (LX, RX): TupleConcat<LX, RX, Type: Tuple>
{
    extern "rust-call" fn call(&self, args: ConcatTuples<LX, RX>) -> Self::Output
    {
        use std::thread::Builder;
        
        let (args_left, args_right) = private::tuple_split_const(args);

        let mut builders = [Builder::new(), Builder::new()]
        .zip(self.thread_names.each_ref()
            .zip(self.thread_stack_sizes.each_ref())
        ).map(|(mut builder, (name, stack_size))| {
            if let Some(name) = name
            {
                builder = builder.name(name.clone());
            }
            if let Some(stack_size) = stack_size
            {
                builder = builder.stack_size(*stack_size);
            }
            builder
        }).into_iter();

        std::thread::scope(|scope| {
            let (handle_left, handle_right) = (
                builders.next().unwrap().spawn_scoped(scope, || self.zipped.left.call(args_left))
                    .map_err(|err| (false, ParError::SpawnThreadError(err)))?,
                builders.next().unwrap().spawn_scoped(scope, || self.zipped.right.call(args_right))
                    .map_err(|err| (true, ParError::SpawnThreadError(err)))?
            );
    
            Ok((
                handle_left.join().map_err(|err| (false, ParError::JoinThreadError(err)))?,
                handle_right.join().map_err(|err| (true, ParError::JoinThreadError(err)))?
            ))
        })
    }
}

pub enum ParError
{
    SpawnThreadError(std::io::Error),
    JoinThreadError(Box<dyn Any + Send>)
}