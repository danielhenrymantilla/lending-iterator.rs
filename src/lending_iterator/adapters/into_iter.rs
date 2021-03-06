/// The <code>impl [Iterator]</code> (not a [`LendingIterator`]!) returned by
/// [`.into_iter()`][LendingIterator::into_iter()].
///
/// Note: since this wrapper only exists to avoid coherence issues,
/// it is **guaranteed** to be a `#[repr(transparent)]` wrapper around its
/// inner `I`.
///
/// This is a property `unsafe` code can rely on: it can thus use transmute to
/// construct it.
///
/// It is also a property that will be upheld within future versions (should
/// this property ever be broken in the future, the change would then be a
/// semver-breaking one, and the type would be renamed to avoid footguns).
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
