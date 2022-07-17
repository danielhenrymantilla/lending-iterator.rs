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

/// [`LendingIterator`] adapters.
#[path = "adapters/_mod.rs"]
pub
mod adapters;

/// Functions, extension traits and types allowing direct construction of
/// [`LendingIterator`]s (no need for custom types or implementations!).
#[path = "constructors/_mod.rs"]
pub
mod constructors;

use r#dyn::*;
#[path = "dyn/_mod.rs"]
pub(in crate)
mod r#dyn;

mod impls;

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
        /// The "output" of this whole hand-rolled GAT:
        /// think of `LendingIteratorඞItem<'lt>::T` as of `LendingIterator::Item<'lt>`.
        ///
        /// ⚠️ **NEVER NAME THIS TRAIT OR ASSOC TYPE DIRECTLY** ⚠️ yadda yadda.
        type T;
    }
)?

#[allow(type_alias_bounds)]
/// `generic_associated_types`-agnostic shorthand for
/// <code>\<I as [LendingIterator]\>::Item\<\'lt\></code>
pub
type Item<'lt, I : LendingIterator> =
    Gat!(<I as LendingIterator>::Item<'lt>)
;

/// The meat of the crate. Trait similar to [`Iterator`] but **for the return
/// type of the `fn next(&'_ mut self)` method being allowed to depend on that
/// `'_`**.
///
/// <details open><summary>Click to hide</summary>
///
///   - That type is called the `Item<'_>` type, and is a
///     [`generic_associated_type`](#a-generic-associated-type).
///
///   - That difference is crucial both in terms of signature complexity
///     (as this crate's API ought to prove 😅) and borrowing semantics.
///
/// Mainly, when yielded, such `Item<'_>` is still `&mut` borrowing `*self`, so
/// **it won't be possible to further advance the iterator** (or anything else,
/// for that matter), **until the current item is no longer used.**
///
/// That is: **the `Item<'_>`s yielded by a [`LendingIterator`] cannot
/// coëxist!**
///
///   - this will thus impose serious usability limitations on it (_e.g_, no
///     `.collect()`ing whatsoever, since collecting items, by definition,
///     expects them to coëxist (within the collection)).
///
///       - For instance, there won't be a `for item in iter {` sugar on these
///         things, since that `for` sugar currently only blesses the stdlib
///         [`Iterator`] trait.
///
///         That being said, `while let Some(item) = iter.next() {` works just
///         as well, to be honest.
///
///   - but the dual / other side of that API restriction is that it is way
///     simpler / less constraining, _for implementors_, to implement this
///     trait.
///
///     The canonical example illustrating this difference is [`windows_mut()`][
///     constructors::windows_mut()], which is both an intuitive "iterator" we
///     can think of, and yet something for which it is _impossible_ to
///     implement [`Iterator`].
///
/// ## A Generic Associated Type
///
/// The core definition of this trait is thus:
///
/**  - ```rust
    #![feature(generic_associated_types)]

    trait LendingIterator {
        type Item<'next>
        where
            Self : 'next,
        ;

        fn next<'next> (
            self: &'next mut Self, // <- `Self : 'next` indeed!
        ) -> Option<Self::Item<'next>>
        ;
    }
    ``` */
///
/// As you can see, it involves that more complex `type Item` definition, which
/// is called a _generic associated type_ (GAT for short), and, it _currently_
/// requires the `nightly`-only `feature(generic_associated_types)`.
///
/// –Then how come this crate can work on stable?— you may ask.
///
/// The answer is that [(lifetime)-GATs can actually be emulated in stable Rust
/// through some extra slightly convoluted hoops][`::nougat`].
///
/// [`::nougat`]: https://docs.rs/nougat
///
/// That's why this crate uses those techniques (and the crate featuring them,
/// [`::nougat`]), to achieve Stable Rust support:
///
/**  - ```rust
    #[::nougat::gat] // 👈 Add this and now It Just Works™ on stable.
    trait LendingIterator {
        type Item<'next>
        where
            Self : 'next,
        ;

        fn next<'next> (
            self: &'next mut Self,
        ) -> Option<Self::Item<'next>>
        ;
    }
    ``` */
///
/// It does come with a few caveats, though: **the `LendingIterator::Item` item
/// is no longer really nameable**, at least not _directly_.
///
///   - The current implementation of [`::nougat]` uses a helper _higher-order_
///     super-trait, called
///     <code>for\<\'any\> [LendingIteratorඞItem]\<\'any\></code>, which has,
///     itself, a non-generic associated type, `::T`. That way,
///     `LendingIteratorඞItem<'next>::T` plays the role of
///     `LendingIterator::Item<'next>`.
///
///     **BUT THIS MAY change within semver-compatible changes of `nougat`**
///
///     Thence why that path should never be used, directly, by downstream code.
///
///     The only reason I am even talking about it and not having it
///     `#[doc(hidden)]` is that exposing it makes understanding the signatures
///     of the adapters multiple order of magnitude easier.
///
/// Thence the following "rules":
///
///   - Use <code>[Item]\<\'_, I\></code> instead of `I::Item<'_>`.
///
///       - you could technically import the `Gat!` macro from the `::nougat`
///         crate, and then use `Gat!(I::Item<'_>)` (this is how this crate
///         manages to define [`Item`], for instance). But it seems less
///         convenient than a type alias.
///
///       - within a `#[gat]`-annotated `trait` or `impl`, most of the
///         `…::Item<'_>` mentions will automagically be amended by the macro
///         (which is why the previous snippet works, despite its usage
///         of `Self::Item<'next>`).
///
///   - If implementing the trait yourself, you need to apply
///     <code>[#\[gat\]][crate::gat]</code> to the `impl` yourself.
///
///   - If reëxporting the trait yourself, you need to also apply
///     <code>[#\[gat(Item)\]][crate::gat]</code> to the `use` statement as
///     well, so people can implement the trait through the new path.
///
///  - [`LendingIterator`] is not really `dyn`-friendly (although IIUC, with
///     `feature(generic_associated_types)` it wouldn't have been either).
///
///     But you can use <code>dyn [LendingIteratorDyn]\<Item = …\> + …</code>
///     instead, which has been designed with `dyn`-friendlyness in mind 🙂.
///
/// </details>
$($($if_not_better_docs)?
    #[gat]
)?
pub
trait LendingIterator
where
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

    /// Query the `next()` `Item` of this `Self` iterator.
    ///
    /// [`LendingIterator`] counterpart of [`Iterator::next()`].
    fn next (
        self: &'_ mut Self,
    ) -> Option<Item<'_, Self>>
    ;

    /// [`LendingIterator`] counterpart of [`Iterator::filter()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::for_each()`].
    fn for_each<> (
        self: Self,
        mut f: impl FnMut(Item<'_, Self>),
    )
    where
        Self : Sized,
    {
        self.fold((), |(), item| f(item))
    }

    /// [`LendingIterator`] counterpart of [`Iterator::fold()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::try_for_each()`].
    fn try_for_each<Err> (
        self: &'_ mut Self,
        mut f: impl FnMut(Item<'_, Self>) -> Result<(), Err>,
    ) -> Result<(), Err>
    {
        self.try_fold((), |(), item| f(item))
    }

    /// [`LendingIterator`] counterpart of [`Iterator::try_fold()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::all()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::any()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::by_ref()`].
    fn by_ref<> (self: &'_ mut Self)
      -> &'_ mut Self
    where
        Self : Sized,
    {
        self
    }

    /// [`LendingIterator`] counterpart of [`Iterator::count()`].
    fn count<> (self: Self)
      -> usize
    where
        Self : Sized,
    {
        self.fold(0_usize, |acc, _| acc + 1)
    }

    /// [`LendingIterator`] counterpart of [`Iterator::find()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::fuse()`].
    fn fuse (self: Self)
      -> Fuse<Self>
    where
        Self : Sized,
    {
        Fuse(Some(self))
    }

    /// [`LendingIterator`] counterpart of [`Iterator::nth()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::position()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::skip()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::skip_while()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::take()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::take_while()`].
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

    /// [`LendingIterator`] counterpart of [`Iterator::map()`].
    ///
    ///   - **Turbofishing the `NewItemType` is mandatory**, otherwise you'll
    ///     run into issues with non-higher-order closures.
    ///
    ///     See the module-level documentation of [`crate::higher_kinded_types`]
    ///     for more info.
    ///
    ///     But the TL,DR is that you'd use it as:
    ///
    ///     <code>lending_iter.map::\<[HKT!]\(ReturnType\<\'_\>\), _>\(</code>
    ///
    ///   - the second idiosyncracy is that, for technical reasons[^1] related
    ///     to the maximally generic aspect of this API, the closure itself
    ///     cannot just be a `Self::Item<'_> -> Feed<'_, NewItemType>` closure,
    ///     and instead, requires that an extra `[]` dummy parameter be part
    ///     of the signature:
    ///
    ///     ```rust
    ///     # #[cfg(any)] macro_rules! ignore { /*
    ///     lending_iter.map::<HKT…, _>(|[], item| { … })
    ///                                  👆
    ///     # */ }
    ///     ```
    ///
    /// [^1]: In the case where `Self::Item<'_>` does _not_ depend on `'_`, the
    /// return type then technically can't depend on it either, so Rust
    /// complains about this (in a rather obtuse fashion). We solve this by
    /// requiring that extra `[]` parameter which acts as a convenient-to-write
    /// `PhantomData` which does depend on `'_`.
    fn map<NewItemType : HKT, F> (
        self: Self,
        f: F,
    ) -> Map<Self, F, NewItemType>
    where
        for<'next>
            F : FnMut(
                [&'next Self; 0],
                Item<'next, Self>,
            ) -> A!(NewItemType<'next>)
        ,
        Self : Sized,
    {
        Map { iter: self, map: f, _phantom_ty: <_>::default() }
    }

    pervasive_hkt_choices! {
        (map, Map)(
            /// Convenience method: same as [`.map()`][Self::map()], but for
            /// hard-coding the `HKT` parameter to
            /// <code>[HKTRef]\<R\> = [HKT!]\(\&R\)</code>.
            ///
            /// This alleviates the call-sites (no more turbofishing needed!)
            /// for such pervasive use cases 🙂
            map_to_ref: [R : ?Sized], HKTRef<R>, -> &'any R,
            /// Convenience method: same as [`.map()`][Self::map()], but for
            /// hard-coding the `HKT` parameter to
            /// <code>[HKTRefMut]\<R\> = [HKT!]\(\&mut R\)</code>.
            ///
            /// This alleviates the call-sites (no more turbofishing needed!)
            /// for such pervasive use cases 🙂
            map_to_mut: [R : ?Sized], HKTRefMut<R>, -> &'any mut R,
        ),
    }

    /// Convenience shorthand for
    /// <code>[.map…\(…\)][Self::map()][.into_iter()][Self::into_iter()]</code>.
    ///
    /// When the return type of the `.map()` closure is not lending
    /// / borrowing from `*self`, it becomes possible to call
    /// [.into_iter()][Self::into_iter()] on it right away.
    ///
    /// Moreover, it makes the `[], ` closure arg hack no longer necessary.
    ///
    /// This convenience function encompasses both things, thence returning
    /// an [`Iterator`] (not a [`LendingIterator`]!).
    fn map_into_iter<F, NonLendingItem> (
        self: Self,
        f: F,
    ) -> MapIntoIter<Self, F>
    where
        F : FnMut(Item<'_, Self>) -> NonLendingItem,
        Self : Sized,
    {
        MapIntoIter(self, f)
    }

    /// [`LendingIterator`] counterpart of [`Iterator::filter_map()`].
    ///
    /// All the caveats and remarks of [`.map()`][Self::map()] apply, go check
    /// them up.
    fn filter_map<NewItemType : HKT, F> (
        self: Self,
        f: F,
    ) -> FilterMap<Self, F, NewItemType>
    where
        for<'next>
            F : FnMut(
                [&'next Self; 0],
                Item<'next, Self>,
            ) -> Option<A!(NewItemType<'next>)>
        ,
        Self : Sized,
    {
        FilterMap { iter: self, map: f, _phantom_ty: <_>::default() }
    }

    pervasive_hkt_choices! {
        (filter_map, FilterMap)(
            /// Convenience method: same as
            /// [`.filter_map()`][Self::filter_map()], but for hard-coding the
            /// `HKT` parameter to <code>[HKTRef]\<R\> = [HKT!]\(\&R\)</code>.
            ///
            /// All the caveats and remarks of
            /// [`.map_to_ref()`][Self::map_to_ref()] apply, go check them up.
            filter_map_to_ref: [R : ?Sized], HKTRef<R>, -> Option<&'any R>,

            /// Convenience method: same as
            /// [`.filter_map()`][Self::filter_map()], but for hard-coding the
            /// `HKT` parameter to <code>[HKTRefMut]\<R\> = [HKT!]\(\&mut R\)</code>.
            ///
            /// All the caveats and remarks of
            /// [`.map_to_mut()`][Self::map_to_mut()] apply, go check them up.
            filter_map_to_mut: [R : ?Sized], HKTRefMut<R>, -> Option<&'any mut R>,
        ),
    }

    /// Convenience shorthand for
    /// <code>[.filter_map…\(…\)][Self::filter_map()][.into_iter()][Self::into_iter()]</code>.
    ///
    /// When the return type of the `.filter_map()` closure is not lending
    /// / borrowing from `*self`, it becomes possible to call
    /// [.into_iter()][Self::into_iter()] on it right away.
    ///
    /// Moreover, it makes the `[], ` closure arg hack no longer necessary.
    ///
    /// This convenience function encompasses both things, thence returning
    /// an [`Iterator`] (not a [`LendingIterator`]!).
    fn filter_map_into_iter<F, NonLendingItem> (
        self: Self,
        f: F,
    ) -> FilterMapIntoIter<Self, F>
    where
        F : FnMut(Item<'_, Self>) -> Option<NonLendingItem>,
        Self : Sized,
    {
        FilterMapIntoIter(self, f)
    }

    /// Convert a <code>Self : [LendingIterator]</code> into an [`Iterator`],
    /// **provided `Self::Item<'_>` does not depend on `'_`**.
    fn into_iter<Item> (
        self: Self,
    ) -> IntoIter<Self>
    where
        Self : for<'any> LendingIteratorඞItem<'any, T = Item>,
        Self : Sized,
    {
        IntoIter(self)
    }

    /// Converts this [`LendingIterator`] into a
    /// <code>[Box][::alloc::boxed::Box]\<dyn [LendingIteratorDyn]…\></code>.
    ///
    /// Note that the return `dyn Trait` will not be `Send` or implement any
    /// other auto-traits. For a more general albeit harder-on-type-inference
    /// alternative, see [`.dyn_boxed_auto()`][Self::dyn_boxed_auto()].
    #[apply(cfg_alloc)]
    fn dyn_boxed<'usability> (
        self: Self
    ) -> ::alloc::boxed::Box<dyn
            'usability + LendingIteratorDyn<Item = HKTItem<Self>>
        >
    where
        Self : 'usability,
        Self : Sized,
    {
        ::alloc::boxed::Box::new(self)
    }

    /// Converts this [`LendingIterator`] into a
    /// <code>[Box][::alloc::boxed::Box]\<dyn [LendingIteratorDyn]…\></code>.
    ///
    /// In order for it to work, the `Item` parameter has to be provided
    /// (probably funneled through a [`CanonicalHKT`]), as well as an explicit
    /// "landing type" (inference will probably fail to figure it out!).
    ///
    /// That is, `BoxedDynLendingIterator` is expected to be of the form:
    ///
    /// <code>[Box]\<dyn \'lt \[+ Send\] \[+ Sync\] + [LendingIteratorDyn]\<Item = [CanonicalHKT]\<…\>\>\></code>
    ///
    /// [Box]: ::alloc::boxed::Box
    /// [CanonicalHKT]: crate::prelude::CanonicalHKT
    fn dyn_boxed_auto<BoxedDynLendingIterator, Item : HKT> (self: Self)
      -> BoxedDynLendingIterator
    where
        Self : Sized + DynCoerce<BoxedDynLendingIterator, Item>,
    {
        Self::coerce(self)
    }
}
)}

macro_rules! pervasive_hkt_choices {(
    $(
        ($map:ident, $Map:ident)(
            $(
                $(#[$attr:meta])*
                $fname:ident: [$($R:tt)*], $HKT:ty, -> $Ret:ty,
            )*
        ),
    )*
) => (
    $(
        $(
            $(#[$attr])*
            fn $fname<$($R)*, F> (
                self: Self,
                f: F,
            ) -> $Map<Self, F, $HKT>
            where
                for<'any>
                    F : FnMut(
                        [&'any Self; 0],
                        Item<'any, Self>,
                    ) -> $Ret
                ,
                Self : Sized,
            {
                self.$map::<$HKT, F>(f)
            }
        )*
    )*
)} use pervasive_hkt_choices;

#[cfg(test)]
mod tests;
