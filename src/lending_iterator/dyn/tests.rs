#![allow(unused)]
use super::*;
use {
    CanonicalHKT as Eta,
};

fn check<'r> (slice: &'r mut [i32])
  -> Box<dyn 'r + LendingIteratorDyn<Item = HKT!(&mut [i32; 1])>>
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
    i
        .by_ref_dyn()
        .fold((), |(), _| ());
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
