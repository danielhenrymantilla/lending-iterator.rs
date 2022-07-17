/// "Lifts" / converts an \[`Into`\][`Iterator`] into an
/// <code>impl [LendingIterator]</code>
///
///   - This is a free function version of the [`.into_lending_iter()`] method
///     provided by the eponymous [extension trait](https://docs.rs/extension_traits).
///
///     That is, feel free to check out that extension method, since in practice
///     it's even more ergonomic to use.
///
/// [`.into_lending_iter()`]: trait@super::into_lending_iter#impl-into_lending_iter<I%2C%20IntoIter>
///
///   - See the [`.into_iter()`] method on [`LendingIterator`] for the reverse operation.
///
/// [`.into_iter()`]: LendingIterator::into_iter
pub
fn from_iter<I : IntoIterator> (it: I)
  -> FromIter<I::IntoIter>
{
    constructors::FromIter(it.into_iter())
}

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
