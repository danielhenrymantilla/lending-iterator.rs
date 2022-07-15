#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
    feature(doc_notable_trait),
)]
#![no_std]
#![forbid(unsafe_code)]
#![allow(nonstandard_style, uncommon_codepoints)]

#[doc(inline)]
pub use self::windows_mut::windows_mut;

// #[macro_use]
// extern crate higher_order_closure;

// #[macro_use]
// extern crate macro_rules_attribute;

// #[macro_use]
extern crate nougat as nou;

#[macro_use]
extern crate polonius_the_crab;

#[macro_use]
mod utils;

pub
mod higher_kinded_types;

pub
mod windows_mut;

#[path = "lending_iterator/_trait.rs"]
pub
mod lending_iterator;

/// The crate's prelude.
pub
mod prelude {
    // …
}

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod ඞ {
    pub use {
        ::core::{ // or `std`
            self,
            marker::PhantomData,
        },
        ::lending_iterator_proc_macros::{
            self,
        },
    };
}

#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}

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

// #[doc(hidden)] /** Not part of the public API */ pub
// struct HKT<T : ?Sized>(fn(&T));
