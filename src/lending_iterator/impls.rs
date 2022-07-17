//! Boring impls go here.

use super::*;

#[gat]
impl<'r, I : ?Sized + LendingIterator>
    LendingIterator
for
    &'r mut I
{
    type Item<'next>
    where
        &'r mut I : 'next,
    =
        Item<'next, I>
    ;

    fn next<'next> (
        self: &'next mut &'r mut I,
    ) -> Option<Item<'next, Self>>
    {
        (*self).next()
    }
}

#[gat]
impl<'r, I : ?Sized + LendingIterator>
    LendingIterator
for
    ::core::pin::Pin<&'r mut I>
where
    I : ::core::marker::Unpin,
{
    type Item<'next>
    where
        &'r mut I : 'next,
    =
        Item<'next, I>
    ;

    fn next<'next> (
        self: &'next mut ::core::pin::Pin<&'r mut I>,
    ) -> Option<Item<'next, Self>>
    {
        (&mut **self).next()
    }
}

#[apply(cfg_alloc)]
#[gat]
impl<I : ?Sized + LendingIterator>
    LendingIterator
for
    ::alloc::boxed::Box<I>
{
    type Item<'next>
    where
        ::alloc::boxed::Box<I> : 'next,
    =
        Item<'next, I>
    ;

    fn next<'next> (
        self: &'next mut ::alloc::boxed::Box<I>,
    ) -> Option<Item<'next, Self>>
    {
        (&mut **self).next()
    }
}

#[apply(cfg_alloc)]
#[gat]
impl<I : ?Sized + LendingIterator>
    LendingIterator
for
    ::core::pin::Pin<::alloc::boxed::Box<I>>
where
    I : ::core::marker::Unpin,
{
    type Item<'next>
    where
        ::core::pin::Pin<::alloc::boxed::Box<I>> : 'next,
    =
        Item<'next, I>
    ;

    fn next<'next> (
        self: &'next mut ::core::pin::Pin<::alloc::boxed::Box<I>>,
    ) -> Option<Item<'next, Self>>
    {
        (&mut **self).next()
    }
}
