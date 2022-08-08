#[doc(no_inline)]
pub use {
    crate::{
        apply, gat, Gat,
        higher_kinded_types::{
            Apply,
            CanonicalHKT,
            Feed,
            HKT,
            HKTItem,
            HKTRef,
            HKTRefMut,
        },
        lending_iterator::{
            constructors::{
                into_lending_iter as _,
                windows_mut as _,
            },
            Item,
            LendingIteratorDyn,
        },
        windows_mut,
    },
};

#[nou::gat(Item)]
pub use crate::lending_iterator::LendingIterator;
