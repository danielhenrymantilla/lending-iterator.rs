pub
struct Take<I : LendingIterator> {
    pub(in crate)
    iter: I,

    pub(in crate)
    remaining: usize,
}

#[gat]
impl<I : LendingIterator> LendingIterator for Take<I> {
    type Item<'next>
    where
        Self : 'next,
    =
        Item<'next, I>
    ;

    fn next (self: &'_ mut Self)
      -> Option<Item<'_, I>>
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            self.iter.next()
        } else {
            None
        }
    }

    #[inline]
    fn nth (
        self: &'_ mut Take<I>,
        n: usize,
    ) -> Option<Item<'_, I>>
    {
        if n < self.remaining {
            self.remaining -= n + 1;
            self.iter.nth(n)
        } else {
            if self.remaining > 0 {
                self.iter.nth(self.remaining - 1);
                self.remaining = 0;
            }
            None
        }
    }
}
