/*!
[`windows_mut()`]: windows_mut()
[HKT!]: higher_kinded_types::HKT!
[higher-kinded]: higher_kinded_types
[`lending_iterator::adapters`]: lending_iterator::adapters
*/
#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
    feature(doc_cfg, doc_notable_trait),
    // for macro_vis
    feature(decl_macro, rustc_attrs),
)]
#![no_std]
#![forbid(unsafe_code)]
#![allow(nonstandard_style, uncommon_codepoints)]

#[macro_use]
mod utils;

#[doc(inline)]
pub use self::{
    lending_iterator::{
        LendingIterator,
        constructors::{
            FromFn,
            from_fn,
            from_iter,
            repeat_mut,
            windows_mut_::windows_mut,
        },
    },
};

/// <code>[#\[::nougat::gat\]]</code>
///
/// [#\[::nougat::gat\]]: https://docs.rs/nougat/~0.2.4/nougat/attr.gat.html
///
/// Using this attribute is needed when implementing `LendingIterator` for your
/// own types, as well as **for reëxporting the `LendingIterator` trait**,
/// itself, in a way that remains compatible with further downstream
/// implementations down the line: `#[gat(Item)] use …::LendingIterator;`.
///
///   - See the documentation of <code>[#\[::nougat::gat\]]</code> for more info
///     about this.
pub use ::nou::gat;

/// <code>[#\[::macro_rules_attribute::apply\]]</code>
///
/// [#\[::macro_rules_attribute::apply\]]: https://docs.rs/macro_rules_attribute/~0.1.2/macro_rules_attribute/attr.apply.html
///
/// Using this in conjunction with [`Gat!`] to get access to expressing
/// `LendingIterator<Item<…> = …>` kind of trait bounds on the annotated item.
///
///   - See the documentation of
///     <code>[#\[::macro_rules_attribute::apply\]]</code> for more info
///     about this.
pub use ::macro_rules_attribute::apply;

/// <code>[::nougat::Gat!]</code>
///
/// [::nougat::Gat!]: https://docs.rs/nougat/~0.2.4/nougat/macro.Gat.html
///
/// You can use this macro around a type to get access to:
///
///   - `<I as LendingIterator>::Item<'lt>` (if for some reason you did not
///     like <code>[Item]\<\'lt, I\></code>);
///
///   - `impl for<'n> LendingIterator<Item<'n> = …>`.
///
/// [Item]: crate::lending_iterator::Item
///
/// You can also use it in conjunction with <code>#\[[apply]\]</code>, as in
/// <code>#\[[apply]\([Gat!]\)\]</code>, to annotate an item with it so as to:
///
///   - get the previous functionality applied to all types occurrences in that
///     annotated item;
///
///   - get `LendingIterator<Item<'…> = …>` kind of trait bounds (_e.g._, as a
///     in `I : …` clauses, or as a super trait) to also work anywhere on the
///     annotated item (this is something no _targeted_ macro could ever
///     support, due to language limitations).
///
/// ## Example
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    #[apply(Gat!)]
    fn my_iter_1<T> (slice: &'_ mut [T])
      -> impl '_ + for<'n> LendingIterator<Item<'n> = &'n mut [T; 2]>
    {
        windows_mut::<T, 2>(slice)
    }
    // same as:
    fn my_iter_2<T> (slice: &'_ mut [T])
      -> Gat!(impl '_ + for<'n> LendingIterator<Item<'n> = &'n mut [T; 2]>)
    {
        windows_mut::<T, 2>(slice)
    }

    #[apply(Gat!)]
    fn print_all<I, T> (mut iter: I)
    where
        T : ::core::fmt::Debug,
        // Trait bound on GAT
        for<'n>
            <I as LendingIterator>::Item<'n> : Send
        ,
        // Equality constraint on GAT
        I : for<'n> LendingIterator<Item<'n> = &'n mut [T; 2]>,
    {
        iter.for_each(|&mut [ref x, ref y]| {
            dbg!(x, y);
        });
    }
    ``` */
///
/// ___
///
///   - See the documentation of <code>[::nougat::Gat!]</code> for more info
///     about this.
pub use ::nou::Gat;

#[doc(inline)]
#[apply(cfg_futures)]
pub use self::lending_iterator::constructors::from_stream;

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
extern crate extension_traits;

#[macro_use]
extern crate macro_rules_attribute;

extern crate nougat as nou;

#[macro_use]
extern crate polonius_the_crab;

pub
mod higher_kinded_types;

#[path = "lending_iterator/_mod.rs"]
pub
mod lending_iterator;

/// The crate's prelude.
pub
mod prelude;

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod ඞ {
    pub use {
        ::core::{ // or `std`
            self,
        },
        ::lending_iterator_proc_macros::{
            self,
        },
    };
}

#[doc(hidden)] /** Not part of the public API */ pub
enum HKT<T : ?Sized> {
    HKT,
    _ඞ((
        ::never_say_never::Never,
        ::core::marker::PhantomData<T>,
    )),
}
#[doc(hidden)] /** Not part of the public API */ pub
use HKT::*;

#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}
