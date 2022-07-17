#[doc(no_inline)]
pub use {
    crate::{
        lending_iterator::{
            constructors::{
                into_lending_iter as _,
                windows_mut as _,
            },
            Item,
            LendingIteratorDyn,
        },
        gat,
        higher_kinded_types::{
            Apply,
            CanonicalHKT,
            Feed,
            HKT,
            HKTItem,
            HKTRef,
            HKTRefMut,
        },
        windows_mut,
    },
};

#[nou::gat(Item)]
pub use crate::lending_iterator::LendingIterator;
