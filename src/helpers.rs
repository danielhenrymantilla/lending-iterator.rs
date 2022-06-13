// *public* helpers!

//! Helper traits and types to work with some of the more advanced higher-order
//! APIs.

/// A trait to help express Higher Kinded Types.
///
/// See <https://docs.rs/polonius-the-crab>'s extensive documentation to
/// know the difference between, for instance:
///
/// ```rust
/// # use ::lending_iterator::helpers::HKT;
/// #
/// type StringRefNaïve<'lt> = &'lt str;
/// // and
/// type StringRef = HKT!(<'lt> &'lt str);
/// ```
///
///   - Answer: while `StringRefNaïve<'lt>` and
///     `<StringRefHkt as WithLifetime<'lt>::T` are both equal to
///     `&'lt str`, `StringRefNaïve`, **alone**, is not a type _per se_
///     (_e.g._, **it can't be turbofished to functions**), whereas `StringRef`
///     is (it can be turbofished).
///
/// ## Short definition
///
/// A higher-kinded type is a (full / standalone) type which "is generic" /
/// which can be fed generic parameters to obtain other types.
///
/// ### Why / what for:
///
/// Consider the following pseudo-code:
///
/// ```rust ,ignore
/// struct StructOfArrays<ArrayKind : <T>> {
///     array_of_i32s: ArrayKind<i32>,
///     array_of_strings: ArrayKind<String>,
/// }
///
/// type StructOfVecs = StructOfArrays<Vec>;
/// type StructOfVecDeques = StructOfArrays<VecDeque>;
/// ```
///
///   - Or even:
///     <code>type StructOfPairs = StructOfArrays\< [HKT!]\(\<T\> \[T; 2\]\) \> </code>;
///
/// Well, in this case we have this `ArrayKind` which is expected to be a
/// generic type parameter, **which is a `<T>` generic type itself**.
///
/// This generic param is thus generic as well. It is thus **a generic generic**.
///
/// This is thus an alternative (and a bit hand-waved) definition of HKTs.
///
/// ## Back to real Rust
///
/// This, in real Rust, comes with three challenges:
///
///   - Expressing that `ArrayKind : <T>` constraint. In other words, encoding
///     the `<T>`-ness property into a trait.
///
///     ```rust ,ignore
///     trait HKT : /* magic */ { /* magic */ }
///     ```
///
///   - Applying / feeding a `<T>` type parameter to it to query the resulting
///     type.
///
///     Given our base idea of using a trait, this last aspect will involve
///     querying an associated type; either through a generic helper trait, or
///     with the associated type being, itself, generic:
///
///       - With `generic_associated_types`:
///
///          1. We could envision using:
///             ```rust ,ignore
///             trait HKT { type Assoc<T>; }
///             ```
///
///          1. And then querying the type would be done with:
///             ```rust ,ignore
///             <ArrayKind as HKT>::Assoc<T>
///             ```
///
///       - Without `generic_associated_types`:
///
///         There is still an option in the case of a generic lifetime parameter
///         (`<'lt>` instead of `<T>`):
///
///          1. We define:
///             ```rust ,ignore
///             trait WithLifetime<'lt> { type Assoc; }
///             ```
///
///          1. And then we alias `trait HKT = for<'any> WithLifetime<'any>;`
///
///          1. Querying the type is then done with:
///             ```rust ,ignore
///             <Type as WithLifetime<'lt>>::Assoc
///             ```
///
///   - Providing implementors or implementations of that trait:
///
///       - neither `Vec` nor `VecDeque` are,
///         _alone_, types. They're "syntactical type paths" which can be fed a
///         type parameter to then refer to one of the `Vec{,Deque}<T>`
///         types.
///
///       - So we'll need to define _ad-hoc_ implementors of this genericity, as
///         hinted by the `HKT!(<T> [T; 2])` example syntax.
///
///     So, while it would be possible to manually implement:
///
///       - the `generic_associated_type`-based trait:
///
///         `impl HKT for … { type Assoc<'lt> = …; }`,
///
///       - or even the without-`generic_…_types` polyfill:
///
///         `impl<'lt> WithLifetime<'lt> for … { type Assoc = …; }`
///
///     , the truth is that we don't necessarily need to write all these
///     implementations if we are able to somehow magically produce appropriate
///     type implementors "on demand" (in an _ad-hoc_ fashion).
///
///     And it turns out there is! `dyn` to the rescue! Indeed, `dyn Trait<…>`
///     is a standalone / "on demand"-queryable type, which does implement
///     `Trait<…>`.
///
///     This yields `dyn for<T> HKT<Assoc<T> = …>` in the general case, and
///     `dyn for<'lt> WithLifetime<'lt, Assoc = …>` in the polyfill case.
///
///     #### A convenience macro shorthand
///
///     Since remembering it all is hard, writing it in full, cumbersome, and
///     reading it, noisy, this crate offers a convenience macro shorthand:
///
///     ```rust
///     use ::lending_iterator::helpers::HKT;
///
///     type StringRef = HKT!(<'lt> &'lt str);
///     ```
///
/// Use `: HKT` as a trait bound when intending to received parameters such as
/// `StringRefHkt` above.
///
/// This can be useful when needing to nudge type inference so as to imbue
/// closures with the appropriate higher-order signature that a fully generic
/// signature, such as [`crate::lending_iterator::lending`]'s.
pub
trait HKT
where
    Self : for<'any> WithLifetime<'any>,
{}
impl<'lt, T : ?Sized> HKT for T
where
    Self : for<'any> WithLifetime<'any>,
{}

/// [`HKT`][trait@HKT]'s internals.
///
/// Mainly expected to be used **to query** the type off an `impl HKT` obtained
/// by [Apply]ing a `'lt`, like this:
///
/// ```rust
/// use ::lending_iterator::helpers::{HKT, WithLifetime};
///
/// type StringRef = HKT!(<'lt> &'lt str);
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
///   - or <code>[Apply!]\(X\<\'lt\>\)</code>
///
/// ### It can be used to manually implement `HKT`
///
/// To `impl HKT` for some type, you can't do `impl HKT for MyType {`.
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

impl<'lt, ImplHKT : ?Sized + HKT>
    WithLifetime<'lt>
for
    ::core::marker::PhantomData<ImplHKT>
{
    type T = Apply!(ImplHKT<'lt>);
}

/// _Ad-hoc_ `impl HKT` type.
#[macro_export]
macro_rules! HKT {
    (
        <$lt:lifetime> $T:ty $(,)?
    ) => (
        $crate::ඞ::PhantomData::<
            dyn for<$lt> $crate::helpers::WithLifetime<$lt, T = $T>
        >
    );
}

#[allow(type_alias_bounds)]
/// Given a <code>Type : [HKT][trait@HKT]</code>, `Feed<'lt, Type>` "feeds" /
/// applies the `<'lt>` to `Type`.
///
/// ```rust
/// use ::lending_iterator::helpers::{HKT, Feed};
///
/// type StrRef = HKT!(<'lt> &'lt str);
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
/// use ::lending_iterator::helpers::{HKT, Apply};
///
/// type StrRef = HKT!(<'lt> &'lt str);
///
/// const EXAMPLE: Apply!(StrRef<'static>) = "This is a `&'static str`";
/// ```
///
/// It's really just sugar for
/// <code>\<Type as [WithLifetime]\<\'lt\>::T</code>.
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
#[macro_export]
macro_rules! Apply {
    (
        $HKT:ty, <$lt:lifetime> $(,)?
    ) => (
        $crate::helpers::Feed<$lt, $HKT>
    );

    (
            $($(@$leading:tt)?
        :: )? $(
        $HKT:ident
        )::+
        <$lt:lifetime>
        $(,)?
    ) => (
        $crate::Apply!(
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

type Foo<T> = HKT!(<'lt> T);
type Bar = Apply!(Foo<()>,<'static>);

#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! ඞ_munch_Apply {
    (
        [acc: $($T:tt)*]
        $current:tt
        $a:tt $b:tt $c:tt
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
        $crate::helpers::Feed<$lt, $T>
    );

    (
        $($bad_input:tt)*
    ) => (
        $crate::ඞ::compile_error! {
            "Usage: `Apply!(Type<'lifetime>)`"
        }
    );
}

#[doc(inline)]
pub use crate::{HKT, Apply};
// type Foo<T, 'lt> = <T as WithLifetime<'lt>>::T;
