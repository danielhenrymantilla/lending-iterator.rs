#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
#![no_std]
#![forbid(unsafe_code)]

#[macro_use]
extern crate higher_order_closure;

#[macro_use]
extern crate macro_rules_attribute;

#[macro_use]
extern crate nougat;

#[macro_use]
extern crate polonius_the_crab;

pub
mod helpers;

pub
mod lending_iterator;

mod utils;

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
    };
}

#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}
