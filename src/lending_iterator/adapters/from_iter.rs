/// Note: since this wrapper only exists to avoid coherence issues,
/// it is **guaranteed** to be a `#[repr(transparent)]` wrapper around its
/// inner `I`.
///
/// This is a property `unsafe` code can rely on: it can thus use transmute to
/// construct it.
#[repr(transparent)]
pub
struct FromIter<I : ?Sized + Iterator>(
    pub I,
);

#[gat]
impl<I : ?Sized + Iterator>
    LendingIterator
for
    FromIter<I>
{
    type Item<'__>
    where
        Self : '__,
    =
        I::Item
    ;

    fn next (
        self: &'_ mut FromIter<I>,
    ) -> Option<I::Item>
    {
        self.0.next()
    }
}
