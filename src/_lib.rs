#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
    feature(doc_notable_trait),
)]
#![no_std]
// #![forbid(unsafe_code)]
#![allow(uncommon_codepoints)]

// #[macro_use]
// extern crate higher_order_closure;

// #[macro_use]
// extern crate macro_rules_attribute;

// #[macro_use]
extern crate nougat;

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
