// *public* helpers!

//! Helper traits and types to work with some of the more advanced higher-order
//! APIs.
//!
#![doc = include_str!("higher_kinded_types.md")]

/// A trait to help express Higher Kinded Types.
///
/// Use `: HKT` as a trait bound when intending to received parameters such as
/// `StringRefHkt` above.
///
/// This can be useful when needing to nudge type inference so as to imbue
/// closures with the appropriate higher-order signature that a fully generic
/// signature, such as [`crate::lending_iterator::lending_iter_from_fn`]'s.
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

impl<'lt, F : ?Sized, R> WithLifetime<'lt> for F
where
    F : FnOnce(&'lt ()) -> R,
{
    type T = R;
}

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
        <$lt:lifetime> => $T:ty $(,)?
    ) => (
        $crate::ඞ::PhantomData::<
            dyn for<$lt> $crate::higher_kinded_types::WithLifetime<$lt, T = $T>
        >
    );

    (
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
#[macro_export]
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
        $crate::Apply! { $T, <$lt> }
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
