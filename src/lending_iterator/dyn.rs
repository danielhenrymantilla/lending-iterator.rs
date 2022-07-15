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

use alloc::boxed::Box;

#[allow(nonstandard_style)]
pub type dynLendingIterator<'usability, Item> =
    dyn 'usability + DynLendingIterator<Item = Item>
;

fn check<'r> (slice: &'r mut [i32])
  -> Box<dynLendingIterator<'r, HKT!(&mut [i32; 1])>>
{
    use crate::windows_mut;
    if true {
        from_fn::<HKT!(&mut [i32; 1]), _, _>(
            slice.iter_mut(),
            |iter| iter.next().map(::core::array::from_mut),
        )
        .dyn_boxed() // ::<HKT!(&mut [i32; 1])>()
        //     .into_lending_iter()
        //     .map::<HKT!(&mut [i32; 1]), _>(|[], at_slice| {
        //         ::core::array::from_mut(at_slice)
        //     })
        // // windows_mut::<_, 1>(slice)
        //     .dyn_boxed()
    } else {
        windows_mut::<_, 2>(slice)
            .map::<HKT!(&mut [i32; 1]), _>(|[], window| {
                ::core::array::from_mut(&mut window[0])
            })
            .dyn_boxed() // ::<HKT!(&mut [i32; 1])>()
    }
}

fn f2<'I, I : 'I + LendingIterator> (i: I)
{
    i   .dyn_boxed() // ::<HKTItem<I>>()
        .dyn_ref()
        .fold((), |(), _| ());
}
// ``` */

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
/// `'next`-lending lifetime. Else you might have to involve [`Canonical`][
/// crate::higher_kinded_types::CanonicalHKT] in your signatures.
pub
trait DynLendingIterator { // <Item : HKT> {
    type Item : ?Sized + HKT;

    fn next (
        self: &'_ mut Self,
    ) -> Option<A!(Self::Item<'_>)>
    ;
}

#[nou::gat]
impl<'usability, Item : HKT>
    LendingIterator
for
    dynLendingIterator<'usability, Item>
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(Item<'next>) // <T as DynLendingIterator<'next>>::Item
    ;

    fn next<'next> (
        self: &'next mut dynLendingIterator<'usability, Item>,
    ) -> Option<A!(Item<'next>)> // Self::Item<'_>>
    {
        DynLendingIterator::next(self)
    }
}

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
trait CoerceToDynLendingIterator<Item : HKT> {
    fn dyn_ref<'r, 'usability> (
        self: &'r mut Self,
    ) -> &'r mut dynLendingIterator<'usability, Item>
    where
        Self : 'usability,
    ;
}

impl<T : LendingIterator>
    CoerceToDynLendingIterator<HKTItem<Self>>
for
    T
{
    fn dyn_ref<'r, 'T> (
        self: &'r mut T,
    ) -> &'r mut dynLendingIterator<'T, HKTItem<Self>>
    where
        T : 'T,
    {
        self
    }
}

impl<Item : HKT>
    CoerceToDynLendingIterator<Item>
for
    dynLendingIterator<'_, Item>
{
    fn dyn_ref<'r, 'usability> (
        self: &'r mut Self,
    ) -> &'r mut dynLendingIterator<'usability, Item>
    where
        Self : 'usability,
    {
        self
    }
}
