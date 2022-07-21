//! [`LendingIterator`] adapters.
//!
//! # Example
//!
//! <details open class="custom"><summary><span class="summary-box"><span class="summary-to-see">Click to see</span><span class="summary-to-hide">Click to hide</span></span></summary>
//!
/*!  - ```rust
    use ::lending_iterator::prelude::*;

    let mut array = [0; 15];
    array[1] = 1;
    // Let's hand-roll our iterator lending `&mut` sliding windows:
    let mut iter = {
            // initial state:
        ::lending_iterator::repeat_mut((&mut array, 0))
            // main logic (lending _slices_):
            .filter_map::<HKT!(&mut [u16]), _>(|[], (array, start)| {
                let to_yield =
                    array
                        .get_mut(*start..)?
                        .get_mut(..3)?
                ;
                *start += 1;
                Some(to_yield)
            })
            // tiny type adaptor (lending _arrays_):
            .map_to_mut(|[], slice| <&mut [u16; 3]>::try_from(slice).unwrap())
        //   ⬑convenience (no need to turbofish an HKT) that is equivalent to:
        //  .map::<HKT!(&mut [u16; 3]), _>(|[], slice| <_>::try_from(slice).unwrap())
    };
    while let Some(&mut [a, b, ref mut next]) = iter.next() {
        *next = a + b;
    }
    assert_eq!(
        array,
        [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377],
    );
    ``` */
//!
//! Notice how any adapters that return a lending / dependent type (such as
//! `&mut …` for `.{filter_,}map()`) are required to take an extra `[]`
//! "dummy" parameter for the closure input. This is due to a technical
//! limitation of the language, and having to add `[], ` was the least
//! cumbersome way that I could find to work around it 😔
//!
//!   - See [`LendingIterator::map()`] for more info about it.
//!
//! </details>

use super::*;

match_! {(
    filter,
    filter_map,
    fuse,
    into_iter,
    map,
    skip,
    take,
) {(
    $(
        $(#[$attrs:meta])*
        $module:ident
    ),* $(,)?
) => (
    $(
        $(#[$attrs])*
        pub use self::$module::*;
        $(#[$attrs])*
        mod $module {
            use super::*;

            include!(concat!(stringify!($module), ".rs"));
        }
    )*
)}}
