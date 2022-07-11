pub
struct Fuse<I : LendingIterator>(
    pub(in crate) Option<I>,
);

#[gat]
impl<I : LendingIterator> LendingIterator for Fuse<I> {
    type Item<'next>
    where
        Self : 'next,
    =
        Item<'next, I>
    ;

    fn next (self: &'_ mut Self)
      -> Option<Item<'_, I>>
    {
        self.0.as_mut().and_then(I::next)
    }
}
