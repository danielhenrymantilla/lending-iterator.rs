use super::*;

/// `dyn`-friendly (`dyn`-safe) version of [`LendingIterator`].
///
/// It is automagically implemented for all types implementing
/// [`LendingIterator`], and, conversely, [`LendingIterator`] is implemented for
/// <code>dyn [LendingIteratorDyn]</code>.
///
// / # A convenient alias
// /
// / Since <code>dyn \'usability + [LendingIteratorDyn]\<Item = …\></code>, on
// / top of yielding that `dyn` "stutter", is a mouthful, such `dyn Trait` type
// / can be named through the <code>[dynLendingIterator]\<\'usability, …\></code>
// / shorthand alias 🙂.
///
/// ### Limitations
///
/// Beware that such a trait does still not play well with contexts which are
/// generic over the "lending mode" ⚠️
///
/// So, if you intend to unify two heterogeneous [`LendingIterator`] under the
/// same <code>dyn LendingIteratorDyn</code>, make sure to hard-code the
/// dependency on the `'next`-lending lifetime.
///
/// Otherwise you might have to:
///   - involve [`CanonicalHKT`][crate::higher_kinded_types::CanonicalHKT] in
///     your signatures,
///   - and use [`.dyn_boxed_auto()`] rather than the [`.dyn_boxed()`] direct
///     shorthand.
///
/// [`.dyn_boxed_auto()`]: LendingIterator::dyn_boxed_auto
/// [`.dyn_boxed()`]: LendingIterator::dyn_boxed
///
/// ### Example: `dyn` coercion of a _fully generic_ `LendingIterator`:
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    fn coercions<'T, Item, T> (it: T)
    where
        Item : HKT,
        T : 'T + Send + Sync + LendingIterator,
        // THIS IS THE BOUND THAT YOU HAVE TO ADD TO MAKE IT WORK, for some reason:
        T : LendingIteratorDyn<Item = CanonicalHKT<Item>>, // 👈
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
///
pub
trait LendingIteratorDyn {
    type Item : ?Sized + HKT;

    fn dyn_next (
        self: &'_ mut Self,
    ) -> Option<A!(Self::Item<'_>)>
    ;

    /// Like [`LendingIterator::by_ref`], but `dyn`-friendly.
    ///
    /// Can be convenient to allow usage of `where Self : Sized` methods when
    /// dealing with a `Box<dyn LendingIteratorDyn…>`:
    ///
    /**  - ```rust ,compile_fail
    use ::lending_iterator::prelude::*;

    fn this_fails<'lt, Item : HKT> (
        mut it: Box<dyn 'lt + LendingIteratorDyn<Item = Item>>,
    )
    {
        // Error, the `for_each` method cannot be invoked on a trait object.
        it.for_each(|_| ());
    }
    ``` */
    ///
    /**    ```console
    error: the `for_each` method cannot be invoked on a trait object
       --> src/lending_iterator/dyn.rs:93:8
        |
    14  |       it.for_each(|_| ());
    ``` */
    ///
    /**  - ```rust
    use ::lending_iterator::prelude::*;

    fn this_works<'lt, Item : HKT> (
        mut it: Box<dyn 'lt + LendingIteratorDyn<Item = Item>>,
    )
    {
        // OK.
        it.by_ref_dyn().for_each(|_| ()); // `(&mut *it).for_each…` ought to work as well.
    }
    ``` */
    fn by_ref_dyn<'usability> (
        self: &'_ mut Self,
    ) -> &'_ mut (dyn 'usability + LendingIteratorDyn<Item = Self::Item>)
    where
        Self : 'usability,
    ;
}

/// `impl LendingIterator : LendingIteratorDyn`
impl<T : LendingIterator>
    LendingIteratorDyn
for
    T
{
    type Item = HKTItem<T>;

    fn dyn_next<'n> (
        self: &'n mut T,
    ) -> Option<A!(HKTItem<T><'n>)> // a pinch of curry for its flavor 😗👌
    {
        self.next()
    }

    fn by_ref_dyn<'usability> (
        self: &'_ mut Self,
    ) -> &'_ mut (dyn 'usability + LendingIteratorDyn<Item = Self::Item>)
    where
        Self : 'usability,
    {
        self
    }
}

with_auto_traits! {( $($AutoTraits:tt)* ) => (
    /// `dyn LendingIteratorDyn + … : LendingIterator`
    #[nou::gat]
    impl<'usability, Item : HKT>
        LendingIterator
    for
        (dyn
            'usability +
            LendingIteratorDyn<Item = Item> +
            $($AutoTraits)*
        )
    {
        type Item<'next>
        where
            Self : 'next,
        =
            A!(Item<'next>) // <T as LendingIteratorDyn<'next>>::Item
        ;

        fn next<'next> (
            self: &'next mut (dyn
                'usability +
                LendingIteratorDyn<Item = Item> +
                $($AutoTraits)*
            ),
        ) -> Option<A!(Item<'next>)> // Self::Item<'_>>
        {
            self.dyn_next()
        }
    }
)}

/// "Lift" and convert an <code>impl [LendingIterator]</code> into an
/// <code>impl [HKT][trait@HKT]</code>.
///
/// ```rust
/// # #[cfg(any())] macro_rules! ignore {
/// type HKTItem<I : LendingIterator> = HKT!(Item<'_, I>);
/// # }
/// ```
///
///   - It is therefore a [`CanonicalHKT`] (no need to apply it again).
///
/// The main property of this alias, and thus the connection between
/// <code>impl [LendingIterator]</code>s and <code>impl [HKT][trait@HKT]</code>,
/// is that:
///
/// ```rust
/// # #[cfg(any())] macro_rules! ignore {
/// // Given some `<'n, I : LendingIterator>`:
/// Apply!(HKTItem<I><'n>) = Feed<'n, HKT!(Item<'_, T>)> = Item<'n, I>
/// # }
/// ```
#[allow(type_alias_bounds)]
pub
type HKTItem<I : ?Sized + LendingIterator> =
    HKT!(Item<'_, I>)
;

#[doc(hidden)] // Let's not overwhelm users of the crate with info.
pub
trait DynCoerce<T, Item> : Sized {
    fn coerce(self: Self) -> T;
}

#[apply(cfg_alloc)]
r#dyn::with_auto_traits! {( $($AutoTraits:tt)* ) => (
    impl<'I, I : 'I, Item>
        DynCoerce<
            ::alloc::boxed::Box<dyn
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
          -> ::alloc::boxed::Box<dyn
                'I + LendingIteratorDyn<Item = CanonicalHKT<Item>> +
                $($AutoTraits)*
            >
        {
            ::alloc::boxed::Box::new(self)
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
