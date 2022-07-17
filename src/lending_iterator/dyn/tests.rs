#![allow(unused)]
use {
    ::alloc::{
        boxed::Box,
    },
    crate::{
        self as lending_iterator,
        prelude::*,
    },
    self::CanonicalHKT as Eta,
};

// Check `dyn`-unification when dealing with a non-generic way of lending.
fn check<'r, T> (slice: &'r mut [T])
  -> Box<dyn 'r + LendingIteratorDyn<Item = HKT!(&mut [T; 1])>>
{
    if true {
        lending_iterator::from_fn::<HKT!(&mut [T; 1]), _, _>(
            slice.iter_mut(),
            |iter| iter.next().map(::core::array::from_mut),
        )
        .dyn_boxed()
    } else {
        crate::windows_mut::<_, 2>(slice)
            .map::<HKT!(&mut [T; 1]), _>(|[], window| {
                ::core::array::from_mut(&mut window[0])
            })
            .dyn_boxed()
    }
}

fn f2<'I, I : 'I + LendingIterator + Send, Item : HKT> (i: I)
where
    I : LendingIteratorDyn<Item = CanonicalHKT<Item>>,
    // for<'any>
    //     I : LendingIteratorඞItem<'any, T = A!(Item<'any>)>
    // ,
{
    let mut i: Box<dyn
        'I + Send
        + LendingIteratorDyn<Item = CanonicalHKT<Item>>
    > =
        i.dyn_boxed_auto()
    ;
    i.fold((), |(), _| ());
}

/// ### Example: `dyn` coercion of a _fully generic_ `LendingIterator`:
///
/// WITH MISSING `Sync`!
///
/**  - ```rust ,compile_fail
    use ::lending_iterator::prelude::*;

    fn coercions<'T, Item, T> (it: T)
    where
        Item : HKT,
        T : 'T + Send + LendingIterator,
        // T : Sync
        T : LendingIteratorDyn<Item = CanonicalHKT<Item>>,
    {
        match () {
            _ => {
                let _: Box<dyn 'T + LendingIteratorDyn<Item = CanonicalHKT<Item>>> =
                    it.dyn_boxed_auto()
                ;
            },
            _ => {
                let _: Box<dyn 'T + LendingIteratorDyn<Item = CanonicalHKT<Item>> + Send> =
                    it.dyn_boxed_auto()
                ;
            },
            _ => {
                let _: Box<dyn 'T + LendingIteratorDyn<Item = CanonicalHKT<Item>> + Sync> =
                    it.dyn_boxed_auto()
                ;
            },
            _ => {
                let _: Box<dyn 'T + LendingIteratorDyn<Item = CanonicalHKT<Item>> + Send + Sync> =
                    it.dyn_boxed_auto()
                ;
            },
        }
    }
    ``` */
extern {}
