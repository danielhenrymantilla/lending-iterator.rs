use ::core::num::NonZeroUsize;

/// The <code>impl [LendingIterator]</code> returned by
/// [`.skip()`][LendingIterator::skip()].
pub
struct Skip<I : LendingIterator> {
    pub(in crate)
    iter: I,

    pub(in crate)
    to_skip: Option<NonZeroUsize>,
}

fn ensure_skipped(it: &mut Skip<impl LendingIterator>) {
    if let Some(to_skip) = it.to_skip.take() {
        // nth(n) skips n+1
        it.iter.nth(to_skip.get() - 1);
    }
}

#[gat]
impl<I : LendingIterator> LendingIterator for Skip<I> {
    type Item<'next>
    where
        Self : 'next,
    =
        Item<'next, I>
    ;

    #[inline]
    fn next (self: &'_ mut Skip<I>)
      -> Option<Item<'_, I>>
    {
        ensure_skipped(self);
        self.iter.next()
    }

    #[inline]
    fn nth (
        self: &'_ mut Skip<I>,
        n: usize,
    ) -> Option<Item<'_, I>>
    {
        ensure_skipped(self);
        self.iter.nth(n)
    }
}
