/// The <code>impl [LendingIterator]</code> returned by [`.filter()`][
/// LendingIterator::filter()].
pub
struct Filter<I, F>
where
    I : LendingIterator,
    F : FnMut(&'_ Item<'_, I>) -> bool,
{
    pub(in crate)
    iter: I,

    pub(in crate)
    should_yield: F,
}

#[gat]
impl<I, F> LendingIterator
    for Filter<I, F>
where
    I : LendingIterator,
    F : FnMut(&'_ Item<'_, I>) -> bool,
{
    type Item<'next>
    where
        Self : 'next,
    =
        Item<'next, I>
    ;

    fn next (
        self: &'_ mut Filter<I, F>,
    ) -> Option<Item<'_, I>>
    {
        self.iter.find(&mut self.should_yield)
    }
}
