# `::lending-iterator`

Fully generic `LendingIterator`s in stable Rust.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/lending-iterator.rs)
[![Latest version](https://img.shields.io/crates/v/lending-iterator.svg)](
https://crates.io/crates/lending-iterator)
[![Documentation](https://docs.rs/lending-iterator/badge.svg)](
https://docs.rs/lending-iterator)
[![MSRV](https://img.shields.io/badge/MSRV-1.57.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/lending-iterator.svg)](
https://github.com/danielhenrymantilla/lending-iterator.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/lending-iterator.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/lending-iterator.rs/actions)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

  - this pattern used to be called `StreamingIterator`, but since [`Stream`](
    https://docs.rs/futures/0.3.21/futures/stream/trait.Stream.html)s entered
    the picture (as the `async/.await` version of `Iterator`s, that is,
    `AsyncIterator`s), it has been deemed more suitable to go for the _lending_
    naming convention.

      - (this could be even more relevant since you can have a `LendingIterator`
        lending `impl Future`s, which would effectively make it another flavor
        of `AsyncIterator`, but not quite the `Stream` variant).

  - For context, this crate is a generalization of other crates such as:
      - [`::streaming_iterator`](https://docs.rs/streaming-iterator/0.1.6/streaming_iterator)
      - [`::fallible_streaming_iterator`](https://docs.rs/fallible-streaming-iterator/0.1.9/fallible_streaming_iterator)

    which hard-code their lending `Item` type to `&_` and `Result<&_, _>`
    respectively.

    This crate does not hardcode such dependent types, and thus encompasses
    _both_ of those traits, and infinitely more!

  - Mainly, it allows lending `&mut _` `Item`s, which means it can handle the
    infamously challenging [`windows_mut()`] pattern!

## Examples

<details open><summary>Click to hide</summary>

### `windows_mut()`!

```rust
use ::lending_iterator::prelude::*;

let mut array = [0; 15];
array[1] = 1;
// Cumulative sums are trivial with a `mut` sliding window,
// so let's showcase that by generating a Fibonacci sequence.
let mut iter = array.windows_mut::<3>();
while let Some(&mut [a, b, ref mut next]) = iter.next() {
    *next = a + b;
}
assert_eq!(
    array,
    [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377],
);
```

### Rolling your own version of it using the handy `from_fn` constructor

  - (Or even the `FromFn` flavor of it to enjoy "named arguments")

```rust
use ::lending_iterator::prelude::*;

let mut array = [0; 15];
array[1] = 1;
// Let's hand-roll our iterator lending `&mut` sliding windows:
let mut iter = {
    let mut start = 0;
    lending_iterator::FromFn::<HKT!(&mut [u16; 3]), _, _> {
        state: &mut array,
        next: move |array| {
            let to_yield =
                array
                    .get_mut(start..)?
                    .get_mut(..3)?
                    .try_into() // `&mut [u8] -> &mut [u8; 3]`
                    .unwrap()
            ;
            start += 1;
            Some(to_yield)
        },
        _phantom: <_>::default(),
    }
};
while let Some(&mut [a, b, ref mut next]) = iter.next() {
    *next = a + b;
}
assert_eq!(
    array,
    [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377],
);
```

  - where that <code>[HKT!]\(\&mut \[u16; 3\]\)</code> is a [higher-kinded] type
    parameter that **has to be turbofished** to let the generic context
    properly figure out the return type of the `next` closure.

    Indeed, if we were to let type inference, alone, figure it out, it wouldn't
    be able to know which lifetimes would be fixed/tied to call-site captures,
    and which would be tied to the "lending-ness" of the iterator (higher-order
    return type).
    See [`::higher-order-closure`](https://docs.rs/higher-order-closure) for
    more info about this.

### `LendingIterator` adapters

See [`lending_iterator::adapters`].

___

</details>

# Bonus: Higher-Kinded Types (HKT)

See [`higher_kinded_types`][higher-kinded] for a presentation about them.

### Real-life usage: `.sort_by_key()` that is fully generic over the key lending mode

As noted in this **6-year-old issue**:

  - [`slice::sort_by_key` has more restrictions than `slice::sort_by`](
    https://github.com/rust-lang/rust/issues/34162)

Such an API can easily be provided using the HKT API of this crate:

<details><summary>Click to see</summary>

```rust
use ::lending_iterator::higher_kinded_types::{*, Apply as A};

fn slice_sort_by_key<Key, Item, KeyGetter> (
    slice: &'_ mut [Item],
    mut get_key: KeyGetter,
)
where
    Key : HKT, // "Key : <'_>"
    for<'any>
        A!(Key<'any>) : Ord
    ,
    KeyGetter : FnMut(&Item) -> A!(Key<'_>),
{
    slice.sort_by(|a, b| Ord::cmp(
        &get_key(a),
        &get_key(b),
    ))
}

// ---- Demo ----

struct Client { key: String, version: u8 }

fn main() {
    let clients: &mut [Client] = &mut [];

    // Error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
    // clients.sort_by_key(|c| &c.key);

    // OK
    slice_sort_by_key::<HKT!(&str), _, _>(clients, |c| &c.key);

    // Important: owned case works too!
    slice_sort_by_key::<HKT!(u8), _, _>(clients, |c| c.version);
}
```

___

</details>

<!-- Fallback-to-hard-coded-paths links (otherwise shadowed in the `lib.rs`) -->

[`windows_mut()`]: https://docs.rs/lending-iterator/0.1.*/fn.windows_mut.html
[HKT!]: https://docs.rs/lending-iterator/0.1.*/lending_iterator/higher_kinded_types/macro.HKT.html
[higher-kinded]: https://docs.rs/lending-iterator/0.1.*/lending_iterator/higher_kinded_types
[`lending_iterator::adapters`]: https://docs.rs/lending-iterator/0.1.*/lending_iterator/lending_iterator/adapters
