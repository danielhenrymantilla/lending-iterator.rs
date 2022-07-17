/*!
[`windows_mut()`]: windows_mut
[HKT!]: higher_kinded_types::HKT!
[higher-kinded]: higher_kinded_types
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

#[doc(inline)]
pub use self::{
    lending_iterator::{
        LendingIterator,
        constructors::{
            FromFn,
            from_fn,
            from_iter,
            repeat,
            windows_mut,
        },
    },
};

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
extern crate macro_rules_attribute;

extern crate nougat as nou;

#[macro_use]
extern crate polonius_the_crab;

#[macro_use]
mod utils;

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
