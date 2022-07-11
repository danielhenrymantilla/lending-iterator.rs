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
