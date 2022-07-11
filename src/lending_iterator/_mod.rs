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
        higher_kinded_types::{*, Apply as A},
    },
    self::{
        adapters::*,
    }
};

#[path = "adapters/_mod.rs"]
pub
mod adapters;

mod impls;

#[cfg(test)]
mod tests;

#[allow(type_alias_bounds)]
/// `generic_associated_types`-agnostic shorthand for
/// <code>\<I as [LendingIterator]\>::Item\<\'lt\></code>
pub
type Item<'lt, I : LendingIterator> =
    Gat!(<I as LendingIterator>::Item<'lt>)
;

pub
fn lending_iter_from_fn<ItemType, State, Next> (
    state: State,
    next: Next,
) -> FromFn<State, ItemType, Next>
where
    ItemType : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(ItemType<'_>) >,
{
    FromFn { state, item_type: <_>::default(), next }
}

#[cfg_attr(feature = "better-docs",
    doc(notable_trait),
)]
pub trait LendingIteratorඞItem<'next, Bounds = &'next Self> {
    type T;
}

// #[gat]
pub
trait LendingIterator : for<'next> LendingIteratorඞItem<'next> {
    // type Item<'next>
    // where
    //     Self : 'next,
    // ;

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

    fn try_for_each<Err> (
        self: &'_ mut Self,
        mut f: impl FnMut(Item<'_, Self>) -> Result<(), Err>,
    ) -> Result<(), Err>
    {
        self.try_fold((), |(), item| f(item))
    }

    fn fold<Acc> (
        mut self: Self,
        acc: Acc,
        mut f: impl FnMut(Acc, Item<'_, Self>) -> Acc,
    ) -> Acc
    where
        Self : Sized,
    {
        self.try_fold(acc, |acc, item| Ok::<_, ǃ>(f(acc, item)))
            .unwrap_or_else(|unreachable: ǃ| match unreachable {})
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

    fn all<> (
        self: &'_ mut Self,
        mut predicate: impl FnMut(Item<'_, Self>) -> bool,
    ) -> bool
    where
        Self : Sized,
    {
        self.try_for_each(
                |item| if predicate(item) {
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
        self.all(|item| predicate(item).not())
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
        // Skip<Self>
        //     : for<'next> LendingIterator<Item<'next> = Item<'next, Self>>
        // ,
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
        // Skip<Self>
        //     : for<'next> LendingIterator<Item<'next> = Item<'next, Self>>
        // ,
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
        // SkipWhile<Self, F>
        //     : for<'next> LendingIterator<Item<'next> = Item<'next, Self>>
        // ,
    {
        SkipWhile { iter: self, predicate }
    }

    fn take (
        self: Self,
        count: usize,
    ) -> Take<Self>
    where
        Self : Sized,
        // Take<Self>
        //     : for<'next> LendingIterator<Item<'next> = Item<'next, Self>>
        // ,
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
        Self : Sized,
        F : FnMut(&Self::Item) -> bool,
        // TakeWhile<Self, F>
        //     : for<'next> LendingIterator<Item<'next> = Item<'next, Self>>
        // ,
    {
        TakeWhile(self)
    }

    #[must_use = "call `.with()` to provide the mapping closure"]
    fn map_lending<NewItemType : HKT> (
        self: Self,
        _phantom_ty: NewItemType,
    ) -> MapLending<Self, NewItemType>
    where
        Self : Sized,
    {
        MapLending(self, _phantom_ty)
    }

    #[must_use = "call `.with()` to provide the closure"]
    fn and_then_lending<NewItemType : HKT> (
        self: Self,
        _phantom_ty: NewItemType,
    ) -> AndThenLending<Self, NewItemType>
    where
        Self : Sized,
    {
        AndThenLending(self, _phantom_ty)
    }

    fn map_lending_ref<R, F>(
        self: Self,
        f: F,
    ) -> With<MapLending<Self, HKT!(&R)>, F>
    where
        Self : Sized,
        F : for<'any> FnMut([&'any (); 0], Item<'any, Self>) -> &'any R,
    {
        self.map_lending(HKT!(&R))
            .with(f)
    }

    fn map_lending_mut<R, F>(
        self: Self,
        f: F,
    ) -> With<MapLending<Self, HKT!(&mut R)>, F>
    where
        Self : Sized,
        F : for<'any> FnMut([&'any (); 0], Item<'any, Self>) -> &'any mut R,
    {
        self.map_lending(HKT!(&mut R))
            .with(f)
    }

    fn and_then_lending_ref<R, F>(
        self: Self,
        f: F,
    ) -> With<AndThenLending<Self, HKT!(&R)>, F>
    where
        Self : Sized,
        F : for<'any> FnMut([&'any (); 0], Item<'any, Self>) -> Option<&'any R>,
    {
        self.and_then_lending(HKT!(&R))
            .with(f)
    }

    fn and_then_lending_mut<R, F>(
        self: Self,
        f: F,
    ) -> With<AndThenLending<Self, HKT!(&mut R)>, F>
    where
        Self : Sized,
        F : for<'any> FnMut([&'any (); 0], Item<'any, Self>) -> Option<&'any mut R>,
    {
        self.and_then_lending(HKT!(&mut R))
            .with(f)
    }
}

// pub use
// #[doc(hidden)]
// pub
// enum lending_iter_from_fn<ItemType, State, Next>
// where
//     ItemType : HKT,
//     Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
// {
//     lending_iter_from_fn,
//     #[doc(hidden)]
//     __(::core::marker::PhantomData<(fn(&()) -> &Self, ǃ)>),
// }

// enum __ {}
// macro_rules! emit {( $($_:tt)* ) => ( $($_)* )} use emit;
// macro_rules! fake_fn {(
//     $(#[doc = $doc:expr])*
//     $pub:vis
//     fn $fname:ident [$($generics:tt)*] (
//         $(
//             $arg:tt : $Arg:ty
//         ),* $(,)?
//     ) $( -> $Ret:ty )?
//     where {
//         $($wc:tt)*
//     }
//     $body:block
// ) => (
//   #[cfg(not(doc))]
//   emit! {
//   ::paste::paste! {
//     use [<__helper__ $fname>]::$fname;
//     #[allow(unused_imports)]
//     $pub use [<__helper__ $fname>]::$fname::*;
//     #[allow(nonstandard_style)]
//     mod [<__helper__ $fname>] {
//         use super::*;

//         #[allow(nonstandard_style)]
//         #[doc(hidden)]
//         pub
//         enum $fname<$($generics)*>
//         where
//             $($wc)*
//         {
//             $fname,
//             #[doc(hidden)]
//             __(::core::marker::PhantomData<(
//                 ǃ, fn(&()) -> &mut Self,
//             )>),
//         }
//     }
//   }

//     impl<$($generics)*> ::core::ops::Deref
//         for $fname<$($generics)*>
//     where
//         $($wc)*
//     {
//         type Target = fn($($Arg),*) $(-> $Ret)?;

//         fn deref (
//             self: &'_ Self,
//         ) -> &'_ Self::Target
//         {
//             impl<$($generics)*> $fname<$($generics)*>
//             where
//                 $($wc)*
//             {
//                 const FN_PTR: fn($($Arg),*) $(-> $Ret)? = |$($arg),*| $body;
//             }

//             &Self::FN_PTR
//         }
//     }
//   }
//     #[cfg(doc)]
//     $(#[doc = $doc])*
//     $pub
//     fn $fname<$($generics)*> (
//         $( $arg: $Arg ),*
//     ) $(-> $Ret)?
//     where
//         $($wc)*
//     $body
// )} use fake_fn;
