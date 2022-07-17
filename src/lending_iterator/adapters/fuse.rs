/// The <code>impl [LendingIterator]</code> returned by [`.fuse()`][
/// LendingIterator::fuse()].
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
        let mut this = self;
        let got_none = polonius!(|this| -> Option<Item<'polonius, I>> {
            if let Some(iter) = &mut this.0 {
                if let item @ Some(_) = iter.next() {
                    polonius_return!(item);
                } else {
                    true
                }
            } else {
                false
            }
        });
        if got_none {
            this.0 = None;
        }
        None
    }
}
