[![Build Status (nightly)](https://github.com/sigurd4/fn_zip/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/fn_zip/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/fn_zip/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/fn_zip/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/fn_zip/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/fn_zip/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/fn_zip/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/fn_zip/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/fn_zip/workflows/Test/badge.svg)](https://github.com/sigurd4/fn_zip/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/fn_zip/workflows/Lint/badge.svg)](https://github.com/sigurd4/fn_zip/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/fn_zip.svg)](https://crates.io/crates/fn_zip)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/fn_zip)](https://docs.rs/fn_zip)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/fn_zip)](https://app.codecov.io/github/sigurd4/fn_zip)

# fn_zip

Provides a zip trait for functions, allowing two functions to be combined at compile-time before being called. This is equivalent to `core::future::join!()`, but lazy, and works for non-async functions.

The resulting function takes the arguments of both functions and return a tuple.

## Example

```rust
use fn_zip::*;

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
```

## Async

The zipped functions can also implement `AsyncFnOnce`, `AsyncFnMut` and `AsyncFn` if both functions qualify.

This is an experimental feature, since it just recently (as of writing) got added to the rust core library on rust-nightly, and may be subject to change at any point. Enable it with feature `async` or `experimental`.

```rust
#![feature(fn_traits)]
#![feature(async_fn_traits)]

use fn_zip::*;

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
```

## Tuple sizes

By default, this crate operates with function pairs of up to 16 arguments combined, and splits them up in the form of tuples. If you want to use differently sized tuples, use the features `8`, `16`, `32`, `64`, `96`, `128`, `160`, `192`, `224` or `256` to set the maximum supported tuple size.

The `dont_hurt_yourself_by_using_all_features` is there to prevent usage of tuples bigger than 8 if `cargo` is ran with the flag `--all-features`. Using a tuple size above 16 is highly discouraged as it will make compilation time unbearably long. Compilation time will increase exponentially. You have been warned.