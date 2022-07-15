// /*! ```rust
// use ::lending_iterator::lending_iterator::{
//     DynLendingIterator,
//     LendingIterator,
//     LendingIteratorඞItem,
// };

// Feed<'n, HKTItem<T>> = Feed<'n, HKT!(<'n> => Item<'n, T>)> = Item<'n, T>;
#[allow(type_alias_bounds)]
pub(crate) type HKTItem<I : ?Sized + LendingIterator> =
    HKT!(Item<'_, I>)
;

pub(crate) extern crate alloc;
pub(crate) use alloc::boxed::Box;

pub trait Helper<'usability, Item, AutoTraits : ?Sized + 'static, ImplicitBounds = &'usability Item> {
    type T : ?Sized
        + 'usability
        // + DynLendingIterator<Item = Item>
    ;
}

with_auto_traits! {( $($($AutoTraits:tt)+)? ) => (
    // #[allow()]
    impl<'usability, Item>
        Helper<'usability, Item, ($(dyn $($AutoTraits)+)?)>
    for
        ()
    {
        type T = dyn 'usability $(+ $($AutoTraits)+)? + DynLendingIterator<Item = Item>;
    }
)}

#[allow(nonstandard_style, type_alias_bounds)]
pub type dynLendingIterator<'usability, Item, AutoTraits : ?Sized = ()> =
    <() as Helper<'usability, Item, AutoTraits>>::T
    // dyn 'usability + DynLendingIterator<Item = Item>
;

use super::*;

/// `dyn`-friendly (`dyn`-safe) version of [`LendingIterator`].
///
/// It is automagically implemented for all types implementing
/// [`LendingIterator`], and, conversely, [`LendingIterator`] is implemented for
/// <code>dyn [DynLendingIterator]</code>.
///
/// # A convenient alias
///
/// Since <code>dyn \'usability + [DynLendingIterator]\<Item = …\></code>, on
/// top of yielding that `dyn` "stutter", is a mouthful, such `dyn Trait` type
/// can be named through the <code>[dynLendingIterator]\<\'usability, …\></code>
/// shorthand alias 🙂.
///
/// ### Limitations
///
/// Beware that such a trait does still not play well with contexts which are
/// generic over the "lending mode" ⚠️
///
/// So, if you intend to unify two heterogeneous [`LendingIterator`] under the
/// same [`dynLendingIterator`], make sure to hard-code the dependency on the
/// `'next`-lending lifetime. Else you might have to involve [`CanonicalHKT`][
/// crate::higher_kinded_types::CanonicalHKT] in your signatures.
pub
trait DynLendingIterator { // <Item : HKT> {
    type Item : ?Sized + HKT;

    fn next (
        self: &'_ mut Self,
    ) -> Option<A!(Self::Item<'_>)>
    ;
}

with_auto_traits! {( $($($AutoTraits:tt)+)? ) => (
    #[nou::gat]
    impl<'usability, Item : HKT>
        LendingIterator
    for
        dyn 'usability $(+ $($AutoTraits)+)? + DynLendingIterator<Item = Item>
    {
        type Item<'next>
        where
            Self : 'next,
        =
            A!(Item<'next>) // <T as DynLendingIterator<'next>>::Item
        ;

        fn next<'next> (
            self: &'next mut (dyn 'usability $(+ $($AutoTraits)+)? + DynLendingIterator<Item = Item>),
        ) -> Option<A!(Item<'next>)> // Self::Item<'_>>
        {
            DynLendingIterator::next(self)
        }
    }
)}

impl<T : LendingIterator>
    DynLendingIterator
for
    T
{
    type Item = HKTItem<T>;

    fn next<'n> (
        self: &'n mut T,
    ) -> Option<A!(HKTItem<T><'n>)> // a pinch of curry for its flavor 😗👌
    {
        LendingIterator::next(self)
    }
}

pub
trait CoerceToDynLendingIterator<Item : HKT, AutoTraits : ?Sized> {
    fn dyn_ref<'r, 'T> (
        self: &'r mut Self,
    ) -> &'r mut dynLendingIterator<'T, Item, AutoTraits>
    where
        Self : 'T,
        () : Helper<'T, Item, AutoTraits>,
    ;
}

with_auto_traits! {( $($($AutoTraits:tt)+)? ) => (
    impl<T>
        CoerceToDynLendingIterator<HKTItem<Self>, ($(dyn $($AutoTraits)+)?)>
    for
        T
    where
        T : LendingIterator + $($($AutoTraits)+)?,
    {
        fn dyn_ref<'r, 'T> (
            self: &'r mut T,
        ) -> &'r mut dynLendingIterator<'T, HKTItem<Self>, ($(dyn $($AutoTraits)+)?)>
        where
            T : 'T,
            // () : Helper<'T, HKTItem<Self>, ($(dyn $($AutoTraits)+)?)>,
        {
            self
        }
    }

    impl<Item : HKT>
        CoerceToDynLendingIterator<Item, ($(dyn $($AutoTraits)+)?)>
    for
        dyn '_ $(+ $($AutoTraits)+)? + DynLendingIterator<Item = Item>
    {
        fn dyn_ref<'r, 'usability> (
            self: &'r mut Self,
        ) -> &'r mut dynLendingIterator<'usability, Item, ($(dyn $($AutoTraits)+)?)>
        where
            Self : 'usability,
            // () : Helper<'usability, Item, ($(dyn $($AutoTraits)+)?)>,
        {
            self
        }
    }
)}

macro_rules! with_auto_traits {( $($rules:tt)* ) => (
    macro_rules! __emit__ { $($rules)* }
    __emit__! {}
    __emit__! { Send }
    __emit__! { Sync }
    __emit__! { Send + Sync }
)} pub(in crate) use with_auto_traits;

#[cfg(any(doc, test))]
#[path = "dyn/tests.rs"]
mod tests;
