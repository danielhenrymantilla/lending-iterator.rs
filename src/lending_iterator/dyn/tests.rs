#![allow(unused)]
use super::*;
use {
    CanonicalHKT as Eta,
};

fn check<'r> (slice: &'r mut [i32])
  -> Box<dynLendingIterator<'r, HKT!(&mut [i32; 1])>>
{
    if true {
        from_fn::<HKT!(&mut [i32; 1]), _, _>(
            slice.iter_mut(),
            |iter| iter.next().map(::core::array::from_mut),
        )
        .dyn_boxed()
    } else {
        crate::windows_mut::<_, 2>(slice)
            .map::<HKT!(&mut [i32; 1]), _>(|[], window| {
                ::core::array::from_mut(&mut window[0])
            })
            .dyn_boxed()
    }
}

fn f2<'I, I : 'I + LendingIterator> (i: I)
{
    i   .dyn_boxed()
        .dyn_ref()
        .fold((), |(), _| ());
}

fn coercions<'T, Item : HKT, T : 'T + Clone + LendingIterator + Send + Sync> (
    it: T,
)
where
    for<'any>
        T : LendingIteratorඞItem<'any, T = A!(Item<'any>)>
    ,
{
    extern crate alloc;
    use alloc::boxed::Box;

    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>,                 >> =
        it.clone().dyn_boxed_auto()
    ;
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>, dyn Send        >> =
        it.clone().dyn_boxed_auto()
    ;
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>, dyn Sync        >> =
        it.clone().dyn_boxed_auto()
    ;
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>, dyn Send + Sync >> =
        it.clone().dyn_boxed_auto()
    ;
}

/** ```rust ,compile_fail
use ::lending_iterator::{
    higher_kinded_types::{*, CanonicalHKT as Eta},
    lending_iterator::*,
};

type HKTItem<T> = HKT!(Item<'_, T>);

fn coercions<'T, Item, T> (
    it: T,
)
where
    Item : HKT,
    T : 'T + Clone + LendingIterator + Send,
{
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>,                 >> =
        it.clone().dyn_boxed_auto()
    ;
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>, dyn Send        >> =
        it.clone().dyn_boxed_auto()
    ;
    // Fails here:
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>, dyn Sync        >> =
        it.clone().dyn_boxed_auto()
    ;
    let _: Box<dynLendingIterator<'T, Eta<HKTItem<T>>, dyn Send + Sync >> =
        it.clone().dyn_boxed_auto()
    ;
}
``` */
extern {}
