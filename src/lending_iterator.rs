use {
    ::core::{
        marker::PhantomData,
        ops::Not,
    },
    ::nougat::{
        *,
    },
    // ::polonius_the_crab::{HKT, WithLifetime},
    crate::{
        higher_kinded_types::{*, Apply as A},
        utils::{
            FnMutOption,
        },
    },
    // self::ext::{
    //     LendingIteratorExt,
    // },
};

mod ext;

#[gat]
pub
trait LendingIterator {
    type Item<'next>
    where
        Self : 'next,
    ;

    fn next (
        self: &'_ mut Self,
    ) -> Option<Self::Item<'_>>
    ;

    fn lending<NewItemType : HKT> (self)
      -> Lending<Self, NewItemType>
    where
        Self : Sized,
    {
        Lending {
            iter: self,
            new_item_type: <_>::default(),
        }
    }

    fn filter<F> (self, should_yield: F)
      -> ext::Filter<Self, F>
    where
        Self : Sized,
        F : FnMut(&'_ Self::Item<'_>) -> bool,
    {
        ext::Filter { iter: self, should_yield }
    }
}

pub
struct Lending<I : LendingIterator, NewItemType : HKT> {
    iter: I,
    new_item_type: PhantomData<NewItemType>,
}

/// `generic_associated_types`-agnostic shorthand for
/// <code>\<I as [LendingIterator]\>::Item\<\'lt\></code>
pub
type Item<'lt, I> = Gat!(<I as LendingIterator>::Item<'lt>);

pub
struct FromFnBuilder<ItemType : HKT>(
    PhantomData<ItemType>,
);

#[must_use = "missing `.iter_from_fn()` call"]
pub
fn lending_iterator<ItemType : HKT> (
    // item_type: ItemType,
) -> FromFnBuilder<ItemType>
{
    FromFnBuilder(<_>::default())
}

impl<Item : HKT> FromFnBuilder<Item> {
    pub
    fn iter_from_fn<State, Next> (
        self: Self,
        state: State,
        next: Next,
    ) -> FromFn<State, Item, Next>
    where
        Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
    {
        FromFn { state, item_type: <_>::default(), next }
    }
}

pub
struct FromFn<State, Item, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    state: State,
    item_type: PhantomData<fn() -> Item>,
    next: Next,
}

#[gat]
impl<State, Item, Next>
    LendingIterator
for
    FromFn<State, Item, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(Item<'next>)
    ;

    fn next (self: &'_ mut FromFn<State, Item, Next>)
      -> Option< A!(Item<'_>) >
    {
        let Self { state, next, .. } = self;
        next(state)
    }
}

// macro_rules! HKT {(
//     < $lt:lifetime > $T:ty
// ) => (
//     PhantomData::< dyn for<$lt> WithLifetime<$lt, T = $T> >
// )}

#[macro_export]
macro_rules! lending {(
    < $lt:lifetime > $T:ty $(,)?
) => (
    lending_iterator::<$crate::HKT!(<$lt> $T)>()
)}
pub use lending;

#[test]
fn inlined_windows_mut ()
{
    let mut array = [0, 1, 2, 3, 4, 5, 6];
    let slice = &mut array[..];
    let mut start = 0;
    let mut window_iter =
        lending!(<'n> &'n mut [u8])
            .iter_from_fn(slice, |it| Some(it))

            .lending::<HKT!(<'n> &'n mut [u8])>()
            .and_then(|[], slice| Some({
                let to_yield = slice.get_mut(start ..)?.get_mut(..2)?;
                start += 1;
                to_yield
            }))

            .lending::<HKT!(<'n> &'n mut [u8; 2])>()
            .map(|[], slice| slice.try_into().unwrap())

            .filter(|&&mut [fst, _]| fst != 0)
        // lending!(<'n> &'n mut [u8; 2])
        //     .iter_from_fn(slice, |slice| Some({
        //         let to_yield = slice.get_mut(start ..)?.get_mut(..2)?;
        //         start += 1;
        //         to_yield.try_into().unwrap()
        //     }))
    ;
    while let Some(&mut [fst, ref mut snd]) = window_iter.next() {
        *snd += fst;
    }
    assert_eq!(
        [0, 1, 3, 6, 10, 15, 21],
        array,
    );
}

// struct Infinite;

// #[gat]
// impl LendingIterator for Infinite {
//     type Item<'next>
//     where
//         Self : 'next,
//     =
//         &'next mut Self
//     ;

//     fn next (
//         self: &'_ mut Self,
//     ) -> Option<&'_ mut Self>
//     {
//         Some(self)
//     }
// }

// struct WindowsMut<Slice, const WIDTH: usize> {
//     slice: Slice,
//     /// This is unfortunately needed for a non-`unsafe` implementation.
//     start: usize,
// }

// #[gat]
// impl<'lt, T, const WIDTH: usize>
//     LendingIterator
// for
//     WindowsMut<&'lt mut [T], WIDTH>
// {
//     type Item<'next>
//     where
//         Self : 'next,
//     =
//         &'next mut [T; WIDTH]
//     ;

//     fn next (self: &'_ mut WindowsMut<&'lt mut [T], WIDTH>)
//       -> Option<&'_ mut [T; WIDTH]>
//     {
//         let to_yield =
//             self.slice
//                 .get_mut(self.start ..)?
//                 .get_mut(.. WIDTH)?
//         ;
//         self.start += 1;
//         Some(to_yield.try_into().unwrap())
//     }
// }

// fn _check<I : LendingIterator> (mut iter: I)
// {
//     let _ = _check::<Infinite>;
//     let _ = _check::<WindowsMut<&'_ mut [u8], 2>>;
//     while let Some(_item) = iter.next() {
//         // …
//     }
// }

// /// `T : MyFnMut<A> <=> T : FnMut(A) -> _`
// trait MyFnMut<A> : FnMut(A) -> Self::Ret {
//     type Ret;
// }
// impl<F : ?Sized + FnMut(A) -> R, A, R> MyFnMut<A> for F {
//     type Ret = R;
// }

// struct Map<I, F>(I, F);

// #[gat]
// impl<I, F> LendingIterator for Map<I, F>
// where
//     I : LendingIterator,
//     for<'any>
//         F : MyFnMut<Item<'any, I>>
//     ,
// {
//     type Item<'next>
//     where
//         Self : 'next,
//     =
//         <F as MyFnMut<Item<'next, I>>>::Ret
//     ;

//     fn next (self: &'_ mut Map<I, F>)
//       -> Option<
//             <F as MyFnMut<Item<'_, I>>>::Ret
//         >
//     {
//         self.0.next().map(&mut self.1)
//     }
// }
