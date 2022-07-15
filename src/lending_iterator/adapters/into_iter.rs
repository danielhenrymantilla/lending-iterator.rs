/// Note: since this wrapper only exists to avoid coherence issues,
/// it is **guaranteed** to be a `#[repr(transparent)]` wrapper around its
/// inner `I`.
///
/// This is a property `unsafe` code can rely on: it can thus use transmute to
/// construct it.
pub
struct IntoIter<I : ?Sized + LendingIterator>(
    pub I,
);

impl<Item, I : ?Sized + LendingIterator>
    Iterator
for
    IntoIter<I>
where
    for<'any>
        I : LendingIteratorඞItem<'any, T = Item>
    ,
{
    type Item = Item;

    fn next (
        self: &'_ mut IntoIter<I>,
    ) -> Option<Item>
    {
        self.0.next()
    }
}
