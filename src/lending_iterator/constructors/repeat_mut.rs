/// I wish we could just use this…
#[cfg(any())]
pub
fn repeat_mut<State> (state: State)
  -> impl for<'any> LendingIteratorඞItem<'any, T = &'any mut State>
        + LendingIterator
{
    from_fn::<HKT!(&mut State), _, _>(state, |it| Some(it))
}

/// Returns an infinite <code>impl [LendingIterator]</code> which lends
/// `&'next mut State` items.
///
/// Useful as an entry-point for the other combinators and adapters.
///
/// It is also conceptually interesting since it features one of the simplest
/// `.next()` implementations, as an "identity" function: `|self| Some(self)`.
///
/// And yet, such a `next()` implementation would have been impossible to
/// feature using an [`Iterator`], since the returned item would not be allowed
/// to keep borrowing from `*self`.
///
/// ## Example
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    let iter =
        lending_iterator::repeat_mut("Globby")
            .take(7)
            .map_into_iter(|&mut globby| globby)
    ;

    assert_eq!(
        iter.collect::<Vec<_>>(),
        ["Globby", "Globby", "Globby", "Globby", "Globby", "Globby", "Globby"],
    );
    ``` */
pub
fn repeat_mut<State> (state: State)
  -> RepeatMut<State>
{
    RepeatMut(state)
}

/// The <code>impl [LendingIterator]</code> returned by [`repeat_mut()`].
pub
struct RepeatMut<State>(
    State,
);

#[gat]
impl<State>
    LendingIterator
for
    RepeatMut<State>
{
    type Item<'next>
    where
        Self : 'next,
    =
        &'next mut State
    ;

    fn next (
        self: &'_ mut RepeatMut<State>,
    ) -> Option<&'_ mut State>
    {
        Some(&mut self.0)
    }
}
