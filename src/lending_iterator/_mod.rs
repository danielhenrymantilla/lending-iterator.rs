//! Trait and helper adapter definitions.

use {
    ::core::{
        marker::PhantomData,
        ops::Not,
    },
    ::never_say_never::{
        Never as ǃ,
    },
    ::nougat::{
        *,
    },
    crate::{
        higher_kinded_types::{*, Apply as A, HKTItem},
    },
    self::{
        adapters::*,
    }
};

pub use self::{
    r#dyn::LendingIteratorDyn,
};

#[path = "adapters/_mod.rs"]
pub
mod adapters;

use r#dyn::*;
#[path = "dyn.rs"]
pub(in crate)
mod r#dyn;

mod impls;

pub
mod windows_mut;

pub
fn from_iter<I : IntoIterator> (it: I)
  -> FromIter<I::IntoIter>
{
    it.into_iter().into_lending_iter()
}

#[allow(type_alias_bounds)]
/// `generic_associated_types`-agnostic shorthand for
/// <code>\<I as [LendingIterator]\>::Item\<\'lt\></code>
pub
type Item<'lt, I : LendingIterator> =
    Gat!(<I as LendingIterator>::Item<'lt>)
;

pub
fn from_fn<Item, State, Next> (
    state: State,
    next: Next,
) -> FromFn<Item, State, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    FromFn { state, _phantom: <_>::default(), next }
}

pub
trait IntoLendingIterator : IntoIterator + Sized {
    fn into_lending_iter (
        self: Self,
    ) -> FromIter<Self::IntoIter>
    {
        impl<T : IntoIterator> IntoLendingIterator for T {}
        FromIter(self.into_iter())
    }
}

// dyn::DynCoercions<
//     HKT!(&mut [T; WINDOW_SIZE]),
//     // vs.
//     HKTItem<
//         WindowsMut<&'lt mut [T], WINDOW_SIZE>
//     >
//     // i.e.
//     HKT!(Item<'_, WindowsMut<&'lt mut [T], WINDOW_SIZE>>)
//     // i.e.
//     HKT!()
// >
// for
// WindowsMut<&'lt mut [T], WINDOW_SIZE>

macro_rules! with_cfg_better_docs {( $($rules:tt)* ) => (
    macro_rules! __emit__ { $($rules)* }

    #[cfg(feature = "better-docs")]
    __emit__! {
        true
    }

    #[cfg(not(feature = "better-docs"))]
    __emit__! {
        false
    }
)}
with_cfg_better_docs! {(
    $(true $($if_better_docs:tt)?)?
    $(false $($if_not_better_docs:tt)?)?
) => (


$($($if_better_docs)?
    /// ⚠️ **NEVER NAME THIS TRAIT DIRECTLY** ⚠️
    /// Implementation detail of `#[gat] trait LendingIterator`.
    ///
    ///   - ⚠️ **The exact name of this trait may change within semver-compatible
    ///     releases** ⚠️
    ///
    /// The only reason this trait is even exposed to begin with is because of
    /// the `notable_trait` feature greatly improving the readability of
    /// [`LendingIterator`]'s adapters.
    #[doc(notable_trait)]
    pub trait LendingIteratorඞItem<'next, Bounds = &'next Self> {
        type T;
    }
)?

$($($if_not_better_docs)?
    #[gat]
)?
pub
trait LendingIterator
where
    // Self : DynCoercions<HKTItem<Self>>,
    $($($if_better_docs)?
        Self : for<'next> LendingIteratorඞItem<'next>,
    )?
{
    $($($if_not_better_docs)?
        type Item<'next>
        where
            Self : 'next,
        ;
    )?

    fn next (
        self: &'_ mut Self,
    ) -> Option<Item<'_, Self>>
    ;

    fn filter<F> (
        self: Self,
        should_yield: F,
    ) -> Filter<Self, F>
    where
        Self : Sized,
        F : FnMut(&'_ Item<'_, Self>) -> bool,
    {
        Filter { iter: self, should_yield }
    }

    fn for_each<> (
        self: Self,
        mut f: impl FnMut(Item<'_, Self>),
    )
    where
        Self : Sized,
    {
        self.fold((), |(), item| f(item))
    }

    fn fold<Acc> (
        mut self: Self,
        acc: Acc,
        mut f: impl FnMut(Acc, Item<'_, Self>) -> Acc,
    ) -> Acc
    where
        Self : Sized,
    {
        self.try_fold(acc, |acc, item| Ok(f(acc, item)))
            .unwrap_or_else(|unreachable: ǃ| unreachable)
    }

    fn try_for_each<Err> (
        self: &'_ mut Self,
        mut f: impl FnMut(Item<'_, Self>) -> Result<(), Err>,
    ) -> Result<(), Err>
    {
        self.try_fold((), |(), item| f(item))
    }

    fn try_fold<Acc, Err> (
        self: &'_ mut Self,
        mut acc: Acc,
        mut f: impl FnMut(Acc, Item<'_, Self>) -> Result<Acc, Err>,
    ) -> Result<Acc, Err>
    {
        while let Some(item) = self.next() {
            acc = f(acc, item)?;
        }
        Ok(acc)
    }

    fn all<> (
        self: &'_ mut Self,
        mut predicate: impl FnMut(Item<'_, Self>) -> bool,
    ) -> bool
    where
        Self : Sized,
    {
        self.try_for_each(
                move |item| if predicate(item) {
                    Ok(())
                } else {
                    Err(())
                },
            )
            .is_ok()
    }

    fn any<> (
        self: &'_ mut Self,
        mut predicate: impl FnMut(Item<'_, Self>) -> bool,
    ) -> bool
    where
        Self : Sized,
    {
        self.all(move |item| predicate(item).not())
            .not()
    }

    fn by_ref<> (self: &'_ mut Self)
      -> &'_ mut Self
    where
        Self : Sized,
    {
        self
    }

    fn count<> (self: Self)
      -> usize
    where
        Self : Sized,
    {
        self.fold(0_usize, |acc, _| acc + 1)
    }

    fn find<'find> (
        self: &'find mut Self,
        mut predicate: impl 'find + FnMut(&Item<'_, Self>) -> bool,
    ) -> Option<Item<'find, Self>>
    where
        Self : Sized,
    {
        use ::polonius_the_crab::prelude::*;
        let mut this = self;
        polonius_loop!(|this| -> Option<Item<'polonius, Self>> {
            let ret = this.next();
            if matches!(ret, Some(ref it) if predicate(it).not()) {
                polonius_continue!();
            }
            polonius_return!(ret);
        })
    }

    fn fuse (self: Self)
      -> Fuse<Self>
    where
        Self : Sized,
    {
        Fuse(Some(self))
    }

    fn nth (
        self: &'_ mut Self,
        n: usize,
    ) -> Option<Item<'_, Self>>
    {
        if let Some(n_minus_one) = n.checked_sub(1) {
            self.skip(n_minus_one);
        }
        self.next()
    }

    fn position<F> (
        self: &'_ mut Self,
        mut predicate: impl FnMut(Item<'_, Self>) -> bool,
    ) -> Option<usize>
    where
        Self : Sized,
    {
        match
            self.try_fold(0, |i, item| if predicate(item) {
                Err(i)
            } else {
                Ok(i + 1)
            })
        {
            | Err(position) => Some(position),
            | Ok(_) => None,
        }
    }

    fn skip (
        self: Self,
        count: usize,
    ) -> Skip<Self>
    where
        Self : Sized,
    {
        Skip {
            iter: self,
            to_skip: count.try_into().ok(),
        }
    }

    #[cfg(TODO)]
    fn skip_while<F> (
        self: Self,
        predicate: F,
    ) -> SkipWhile<Self, F>
    where
        F : FnMut(Item<'_, Self>) -> bool,
        Self : Sized,
    {
        SkipWhile { iter: self, predicate }
    }

    fn take (
        self: Self,
        count: usize,
    ) -> Take<Self>
    where
        Self : Sized,
    {
        Take {
            iter: self,
            remaining: count,
        }
    }

    #[cfg(TODO)]
    fn take_while<F> (
        self: Self,
        f: F,
    ) -> TakeWhile<Self, F>
    where
        F : FnMut(&Self::Item) -> bool,
        Self : Sized,
    {
        TakeWhile(self)
    }

    fn map<NewItemType : HKT, F> (
        self: Self,
        f: F,
    ) -> Map<Self, F, NewItemType>
    where
        for<'next>
            F : FnMut(
                [&'next (); 0],
                Item<'next, Self>,
            ) -> A!(NewItemType<'next>)
        ,
        Self : Sized,
    {
        Map { iter: self, map: f, _phantom_ty: <_>::default() }
    }

    fn and_then<NewItemType : HKT, F> (
        self: Self,
        f: F,
    ) -> AndThen<Self, F, NewItemType>
    where
        for<'next>
            F : FnMut(
                [&'next (); 0],
                Item<'next, Self>,
            ) -> Option<A!(NewItemType<'next>)>
        ,
        Self : Sized,
    {
        AndThen { iter: self, map: f, _phantom_ty: <_>::default() }
    }

    pervasive_hkt_choices! {
        (map, Map)(
            map_to_ref: [R : ?Sized] [&'any R] -> &'any R,
            map_to_mut: [R : ?Sized] [&'any mut R] -> &'any mut R,
            // map_to_owned: [R] [R] -> R,
        ),
        (and_then, AndThen)(
            and_then_to_ref: [R : ?Sized] [&'any R] -> Option<&'any R>,
            and_then_to_mut: [R : ?Sized] [&'any mut R] -> Option<&'any mut R>,
            // and_then_to_owned: [R] [R] -> Option<R>,
        ),
    }

    fn map_into_iter<F, Owned> (
        self: Self,
        f: F,
    ) -> MapIntoIter<Self, F>
    where
        F : FnMut(Item<'_, Self>) -> Owned,
        Self : Sized,
    {
        MapIntoIter(self, f)
    }

    fn and_then_into_iter<F, Owned> (
        self: Self,
        f: F,
    ) -> AndThenIntoIter<Self, F>
    where
        F : FnMut(Item<'_, Self>) -> Option<Owned>,
        Self : Sized,
    {
        AndThenIntoIter(self, f)
    }

    fn into_iter<Item> (
        self: Self,
    ) -> IntoIter<Self>
    where
        Self : for<'any> LendingIteratorඞItem<'any, T = Item>,
        Self : Sized,
    {
        IntoIter(self)
    }

    #[apply(cfg_alloc)]
    fn dyn_boxed<'usability> (
        self: Self
    ) -> Box<dyn 'usability + LendingIteratorDyn<Item = HKTItem<Self>>>
    where
        Self : 'usability,
        Self : Sized,
    {
        Box::new(self)
    }

    fn dyn_boxed_auto<BoxedDynLendingIterator, Item : HKT> (self: Self)
      -> BoxedDynLendingIterator
    where
        Self : Sized + DynCoerce<BoxedDynLendingIterator, Item>,
    {
        Self::coerce(self)
    }
}
)}

#[doc(hidden)] // Let's not overwhelm users of the crate with info.
pub
trait DynCoerce<T, Item> : Sized {
    fn coerce(self: Self) -> T;
}

#[apply(cfg_alloc)]
r#dyn::with_auto_traits! {( $($AutoTraits:tt)* ) => (
    impl<'I, I : 'I, Item>
        DynCoerce<
            Box<dyn
                'I + LendingIteratorDyn<Item = CanonicalHKT<Item>> +
                $($AutoTraits)*
            >,
            Item,
        >
    for
        I
    where
        Item : HKT,
        I : LendingIteratorDyn<Item = CanonicalHKT<Item>>,
        I : $($AutoTraits)* ,
    {
        fn coerce (self: I)
          -> Box<dyn
                'I + LendingIteratorDyn<Item = CanonicalHKT<Item>> +
                $($AutoTraits)*
            >
        {
            Box::new(self)
        }
    }
)}

macro_rules! pervasive_hkt_choices {(
    $(
        ($map:ident, $Map:ident)(
            $(
                $([$attr:meta])*
                $fname:ident: [$($R:tt)*] [$HKT:ty] -> $Ret:ty,
            )*
        ),
    )*
) => (
    $(
        $(
            $([$attr])*
            fn $fname<$($R)*, F> (
                self: Self,
                f: F,
            ) -> $Map<Self, F, HKT!(<'any> => $HKT)>
            where
                for<'any>
                    F : FnMut(
                        [&'any (); 0],
                        Item<'any, Self>,
                    ) -> $Ret
                ,
                Self : Sized,
            {
                self.$map::<HKT!(<'any> => $HKT), F>(f)
            }
        )*
    )*
)} use pervasive_hkt_choices;

#[cfg(test)]
mod tests;
