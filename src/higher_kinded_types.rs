// *public* helpers!

//! Helper traits and types to work with some of the more advanced higher-order
//! APIs.
//!
#![doc = include_str!("higher_kinded_types.md")]

/// A trait to help express [Higher Kinded Types][self].
///
/// Use `: HKT` as a trait bound when intending to received parameters such as
/// `StringRefHkt` above.
///
/// This can be useful when needing to nudge type inference so as to imbue
/// closures with the appropriate higher-order signature that a fully generic
/// signature, such as [`lending_iterator::from_fn`][crate::from_fn()]'s.
///
/// See [the module documentation for more info][self].
pub
trait HKT
where
    Self : for<'any> WithLifetime<'any>,
{}
impl<T : ?Sized> HKT for T
where
    Self : for<'any> WithLifetime<'any>,
{}

/// [`HKT`][trait@HKT]'s internals.
///
/// Mainly expected to be used **to query** the type off an `impl HKT` obtained
/// by [Apply]ing a `'lt`, like this:
///
/// ```rust
/// use ::lending_iterator::higher_kinded_types::{HKT, WithLifetime};
///
/// type StringRef = HKT!(<'lt> => &'lt str);
///
/// fn example<'s>(s: <StringRef as WithLifetime<'s>>::T) -> &'s str {
///     s
/// }
/// ```
///
/// That is, given some `Type : HKT`, and some lifetime `'lt`, you can feed
/// / apply the lifetime `'lt` to the `Type` by using:
///
///   - ```rust
///     # #[cfg(any())] macro_rules! {
///     <Type as WithLifetime<'lt>>::T
///     # }
///     ```
///
///   - or <code>[Feed]\<\'lt, X\></code>
///
///   - or <code>[Apply!]\(X\<\'lt\>\)</code>
///
/// ### It can be used to manually implement `HKT`
///
/// To `impl HKT` for some type, you can't do `impl HKT for MyType`.
/// Instead, you'd have to `impl<'lt> WithLifetime<'lt> for MyType`.
///
///   - But such use case is not strongly supported by this crate: it is thus
///     likely that you'll  run into "add `: 'static`" kind of requirements
///     when doing so (because I haven't attached implicit bounds here, contrary
///     to [`::polonius_the_crab::HKT`]).
pub
trait WithLifetime<'lt> {
    /// Implicit `: Sized` bound not removed for convenience.
    type T;
}

// Make it `PhantomData`-transitive, to allow instantiating _ad-hoc_ HKTs.
impl<'lt, ImplHKT : ?Sized + HKT>
    WithLifetime<'lt>
for
    ::core::marker::PhantomData<ImplHKT>
{
    type T = Apply!(ImplHKT<'lt>);
}
// When only working in the type-level realm, using `PhantomData` yields very
// long and heavy-weight paths for the HKT types.
// Thence the usage of an aptly-named shorthand-wrapper.
impl<'lt, ImplHKT : ?Sized + HKT>
    WithLifetime<'lt>
for
    crate::HKT<ImplHKT>
{
    type T = Apply!(ImplHKT<'lt>);
}

/// _Ad-hoc_ <code>impl [HKT][trait@HKT]</code> type.
///
/// See [the module documentation for more info][self] for more info.
///
/// ### Examples
///
/**  - ```rust
    use ::lending_iterator::higher_kinded_types::HKT;

    // All these three define the same `HKT` type:
    type StrRef = HKT!(<'any> => &'any str);
    type StrRefElided = HKT!(&str);
    type StrRefElided2 = HKT!(&'_ str);

    type LifetimeParamsWorkToo = HKT!(::std::borrow::Cow<'_, str>);
    ``` */
#[apply(public_macro!)]
macro_rules! HKT {
    (
        <$lt:lifetime> => $T:ty $(,)?
    ) => (
        $crate::HKT::<
            dyn for<$lt> $crate::higher_kinded_types::WithLifetime<$lt, T = $T>
        >
    );

    (
        $(@docs.rs
            "Lifetime elision case: use `'_` or `&[mut] …` to replace the \
            so-elided lifetimes with a HKT-higher-order one."
        )?
        $T:ty $(,)?
    ) => (
        $crate::higher_kinded_types::HKT!(
            // It is very sad that using `fn(&()) -> $T` or variants based off
            // it does not seem to yield an actually usable HKT type.
            //
            // So fall back to manually uneliding the lifetimes using a
            // proc-macro 😔.
            <'ඞ /* ' */> => $crate::ඞ::lending_iterator_proc_macros::HKT!($T)
        )
    );
}

#[allow(type_alias_bounds)]
/// Given a <code>Type : [HKT][trait@HKT]</code>, `Feed<'lt, Type>` "feeds" /
/// applies the `<'lt>` to `Type`.
///
/// ```rust
/// use ::lending_iterator::higher_kinded_types::{HKT, Feed};
///
/// type StrRef = HKT!(<'lt> => &'lt str);
///
/// const EXAMPLE: Feed<'static, /* to */ StrRef> = "This is a `&'static str`";
/// ```
///
///   - It's really just sugar for
///     <code>\<Type as [WithLifetime]\<\'lt\>::T</code>.
///
///   - For a more natural param ordering, consider using
///     <code>[Apply!]\(Type\<\'lt\>)</code>
pub
type Feed<'lt, T : ?Sized + HKT> = <T as WithLifetime<'lt>>::T;

/// Given a <code>Type : [HKT][trait@HKT]</code>, `Apply!(Type<'lt>)` "feeds" /
/// applies `<'lt>` to `Type`.
///
/// ```rust
/// use ::lending_iterator::higher_kinded_types::{HKT, Apply};
///
/// type StrRef = HKT!(<'lt> => &'lt str);
///
/// const EXAMPLE: Apply!(StrRef<'static>) = "This is a `&'static str`";
/// ```
///
/// It's really just sugar for
/// <code>[Feed]\<\'lt, Type\></code>.
///
/// ## Usage
///
///   - `Apply!(Type<'lifetime>)` (may involve munching when `Type` is complex)
///
///   - `Apply!(Type, <'lifetime>)` (instantly parsed)
///
/// ## Non-macro alternative
///
/// If you don't like using macros in type position, rather than
/// using `Apply!(Type<'lifetime>)` or `Apply!(Type, <'lifetime>)`, you can use
/// <code>[Feed]\<\'lifetime, Type\></code>.
#[apply(public_macro!)]
macro_rules! Apply {
    (
        $HKT:ty, <$lt:lifetime> $(,)?
    ) => (
        $crate::higher_kinded_types::Feed<$lt, $HKT>
    );

    (
            $($(@$leading:tt)?
        :: )? $(
        $HKT:ident
        )::+
        <$lt:lifetime>
        $(,)?
    ) => (
        $crate::higher_kinded_types::Apply!(
                $($($leading)?
            :: )? $(
            $HKT
            )::+
            ,
            <$lt>
        )
    );

    (
        $($fallback_to_tt_munching_input:tt)*
    ) => (
        $crate::ඞ_munch_Apply! {
            [acc: ]
            $($fallback_to_tt_munching_input)*
        }
    );
}

#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! ඞ_munch_Apply {
    // Trailing comma case.
    (
        $acc:tt
        <$lt:lifetime> ,
    ) => (
        $crate::ඞ_munch_Apply! {
            $acc
            <$lt>
        }
    );

    (
        [acc: $($T:tt)*]
        $current:tt
        $a:tt $b:tt $c:tt // More than 3 (`<` `'lt` `>`) tokens left.
        $($rest:tt)*
    ) => (
        $crate::ඞ_munch_Apply! {
            [acc: $($T)* $current ]
            $a $b $c $($rest)*
        }
    );

    (
        [acc: $T:ty]
        <$lt:lifetime>
    ) => (
        $crate::higher_kinded_types::Apply! { $T, <$lt> }
    );

    (
        $($bad_input:tt)*
    ) => (
        $crate::ඞ::core::compile_error! {
            "Usage: `Apply!(Type<'lifetime>)`"
        }
    );
}

/// \[eta-expansion\] Projects an arbitrary <code>impl [HKT]</code> to its
/// [`HKT!`] "canonical" (η-expanded) form.
///
///   - To illustrate, let's consider a non-canonical <code>impl [HKT]</code>
///     type:
///
///     ```rust
///      use ::lending_iterator::higher_kinded_types::*;
///
///      enum StrRef {}
///      impl<'lt> WithLifetime<'lt> for StrRef {
///          type T = &'lt str;
///      }
///     ```
///
///     Then, we have <code>StrRef : [HKT]</code> (and for any `'lt`,
///     <code>[Apply!]\(StrRef\<\'lt\>\) = \&\'lt str</code>).
///
///     And yet, **<code>StrRef ≠ [HKT!]\(\&str\)</code>**, since the latter is
///     actually something along the lines of
///     `dyn for<'lt> WithLifetime<'lt, T = &'lt str>`, which is clearly not,
///     **nominally**, our `StrRef` type.
///
///     This [`CanonicalHKT`] operation then represents an operation which
///     "extracts" the inherent `HKT` semantics of the given `impl HKT` type
///     (_e.g._, `<'n> => &'n str` for both `StrRef` and `HKT!(&str)`), to then
///     wrap them into / apply them to / project them to a [`HKT!`] type
///     (_e.g._, `HKT!(&str)`).
///
///     So, while <code>StrRef ≠ [HKT!]\(\&str\)</code>, we do have
///     <code>[CanonicalHKT]\<StrRef\> = [HKT!]\(\&str\)</code> 👌
///
/// [HKT]: trait@HKT
///
/// It's a projection, in the mathematical sense, since the operation is
/// _idempotent_: for any `T : HKT`,
/// ```rust
/// # #[cfg(any())] macro_rules! ignore {
/// CanonicalHKT<CanonicalHKT<T>> = CanonicalHKT<T>
/// # }
/// ```
///
/// Proof:
///  1. `CanonicalHKT<T> = HKT!(hkt-ness of T)`;
///  1. `CanonicalHKT<U = HKT!(…)> = HKT!(hkt-ness of HKT!(…)) = HKT!(…) = U`.
///  1. Replace with `U = CanonicalHKT<T>`.
///
/// Thence the usefulness of this tool: given a generic `Item : HKT`, certain
/// "round-tripping" operations such as going from [`LendingIterator`] to
/// <code>dyn [LendingIteratorDyn]</code> "and back" is unlikely to have kept
/// the very same HKT type in place: it may itself have "suffered" from a
/// `CanonicalHKT` lift-up by such process.
///
/// [`LendingIterator`]: crate::lending_iterator::LendingIterator
/// [LendingIteratorDyn]: crate::lending_iterator::LendingIteratorDyn
///
/// Thus, APIs expecting to work with such things may avoid compile errors by
/// preventively `CanonicalHKT`-lifting their own `Item : HKT` types in the
/// signatures… 😅
///
/// # Example
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    fn unify<'usability, I, J, Item> (i: I, j: J)
      -> [Box<dyn 'usability + LendingIteratorDyn<Item = CanonicalHKT<Item>>>; 2]
                                //                       ^^^^^^^^^^^^^    ^
                                // without it, this snippet would fail to compile.
    where
        Item : HKT,
        I : 'usability + LendingIterator,
        J : 'usability + LendingIterator,
        // Extra bounds required for the `dyn` coercion:
        I : LendingIteratorDyn<Item = CanonicalHKT<Item>>,
        J : LendingIteratorDyn<Item = CanonicalHKT<Item>>,
    {
        [
            i.dyn_boxed_auto(),
            j.dyn_boxed_auto(),
        ]
    }

    // Uncomment this to make the above function fail.
    // type CanonicalHKT<T> = T;
    ``` */
///
/// If we un-comment the above `CanonicalHKT` alias which shadows it with a
/// no-op (_i.e._, if we remove the `CanonicalHKT`s from the snippet above
/// altogether), we get the following error message:
///
/**  - ```console
    error[E0308]: mismatched types
      --> src/higher_kinded_types.rs:300:9
       |
    9  | fn unify<'usability, I, J, Item> (i: I, j: J)
       |                            ---- this type parameter
    ...
    21 |         i.dyn_boxed(),
       |         ^^^^^^^^^^^^^ expected type parameter `Item`, found enum `lending_iterator::HKT`
       |
       = note: expected struct `Box<(dyn LendingIteratorDyn<Item = Item> + 'usability)>`
                  found struct `Box<dyn LendingIteratorDyn<Item = lending_iterator::HKT<(dyn for<'ඞ> WithLifetime<'ඞ, for<'ඞ> T = <Item as WithLifetime<'ඞ>>::T> + 'static)>>>`
    ``` */
///
/// Mostly, notice the mismatch with `Item`:
///
/// ```rust
/// # #[cfg(any())] macro_rules! ignore {
/// lending_iterator::HKT<(dyn for<'ඞ> WithLifetime<'ඞ, /* for<'ඞ> */ T = <Item as WithLifetime<'ඞ>>::T> + 'static)>
/// // i.e.
/// lending_iterator::HKT<(dyn for<'n> WithLifetime<'n, T = Apply!(Item<'n>)>)>
/// // i.e.
/// HKT!(<'n> => Apply!(Item<'n>))
/// // i.e.
/// CanonicalHKT<Item>
/// # }
/// ```
///
/// Contrary to the generic `Item` which may be of any shape, these types are
/// [HKT!]-constructed <code>impl [HKT]</code> types, hence the type mismatch.
///
/// But if we lift `Item` so that it be, itself, an [`HKT!`]-constructed
/// <code>impl [HKT]</code> type, that is, if we use
/// <code>[CanonicalHKT]\<Item\></code> rather than `Item`, we no longer are in
/// that situation and thus avoid the issue.
#[allow(type_alias_bounds)]
pub type CanonicalHKT<T : ?Sized + HKT> = HKT!(Feed<'_, T>);

/// Pervasive [`HKT!`] choice: <code>[HKT!]\<\&T\></code>
///
/// ```rust
/// # #[cfg(any())] macro_rules! ignore {
/// type HKTRef<T : ?Sized> = HKT!(&T);
/// # }
/// ```
#[allow(type_alias_bounds)]
pub
type HKTRef<T : ?Sized> = HKT!(&T);

/// Pervasive [`HKT!`] choice: <code>[HKT!]\<\&mut T\></code>
///
/// ```rust
/// # #[cfg(any())] macro_rules! ignore {
/// type HKTRefMut<T : ?Sized> = HKT!(&mut T);
/// # }
/// ```
#[allow(type_alias_bounds)]
pub
type HKTRefMut<T : ?Sized> = HKT!(&mut T);

#[doc(inline)]
pub
use crate::lending_iterator::r#dyn::HKTItem;

cfg_match! {
    feature = "better-docs" => {},
    _ => {
        pub use macro_imports_helper::{Apply, HKT};
        mod macro_imports_helper {
            pub use {Apply, HKT};
        }
    },
}
