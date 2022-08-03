/// Main _ad-hoc_ / closure-based constructor of `LendingIterator`s.
///
/// It expects the both necessary and sufficient elements for such an impl:
///
///   - a `State`, which will play a role akin to that of `Self` in a manual
///     impl;
///
///   - a `fn next` "method" on it. There is actually a certain level of
///     flexibility gained from this being a closure rather than a stateless
///     associated function.
///
///     For instance, non-lent state (such as an internal mutable counter) can
///     be implicity captured by such a closure, without having to funnel it
///     through the lendable `State`.
///
/// ## Example
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    struct Person {
        name: String,
        age: u8,
    }

    fn example (person: &mut Person)
      -> impl '_ + LendingIterator
    /* or:
      -> impl '_ + LendingIteratorDyn<Item = HKT!(&str)>
     */
    {
        lending_iterator::from_fn::<HKT!(&str), _, _>(
            person,
            |p| if p.age > 0 {
                Some(&p.name)
            } else {
                None
            },
        )
    }
    ``` */
///
/// ## Remarks
///
/// It can also be viewed as a convenience layer over:
///
/// <code>::lending_iterator::[repeat_mut]\(state\)<br/>    [.filter_map]::\<Item, _\>\(move |\[\], it| next\(it\)\)</code>
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    struct Person {
        name: String,
        age: u8,
    }

    fn example (person: &mut Person)
      -> impl '_ + LendingIterator
    /* or:
      -> impl '_ + LendingIteratorDyn<Item = HKT!(&str)>
     */
    {
        lending_iterator::repeat_mut(person)
            .filter_map::<HKT!(&str), _>(
                |[], p| if p.age > 0 {
                    Some(&p.name)
                } else {
                    None
                },
            )
    }
    ``` */
///
/// [repeat_mut]: crate::repeat_mut()
/// [.filter_map]: crate::LendingIterator::filter_map
///
/// ### `FromFn`
///
/// The returned `struct` —[`FromFn`]— can also be used directly, to benefit
/// from "named arguments", at the cost of having to provide a `PhantomData`
/// parameter.
pub
fn from_fn<Item, State, Next> (
    state: State,
    next: Next,
) -> FromFn<Item, State, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    FromFn { state, next, _phantom: <_>::default() }
}

/// The <code>impl [LendingIterator]</code> returned by [`from_fn()`].
///
/// ## Example
///
/**  - ```rust
    use ::lending_iterator::prelude::*;

    struct Person {
        name: String,
        age: u8,
    }

    fn example (person: &mut Person)
      -> impl '_ + LendingIterator
    /* or:
      -> impl '_ + LendingIteratorDyn<Item = HKT!(&str)>
     */
    {
        lending_iterator::FromFn::<HKT!(&str), _, _> {
            state: person,
            next: |p| if p.age > 0 {
                Some(&p.name)
            } else {
                None
            },
            _phantom: <_>::default(),
        }
    }
    ``` */
pub
struct FromFn<Item, State, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    /// The state owned by this [`LendingIterator`].
    ///
    ///  - Think of `Self` within a manual implementation of the trait;
    ///
    ///  - Think of [`repeat_mut()`].
    pub
    state: State,

    /// The "`fn next()`" of a "manual implementation of the trait".
    ///
    /// Trick: since it's only required to be a closure, this `Next` closure
    /// can capture state on its own, provided it does not need to lend from it.
    ///
    /// This can lead to slightly more lightweight `FromFn` / `from_fn` calls:
    ///   - put the lent / borrowed state inside `.state`,
    ///   - let the rest of the state be implicitly `move`-captured by this closure.
    pub
    next: Next,

    /// The signature of `fn next` in a `PhantomData`.
    pub
    _phantom: PhantomData<
        fn(&mut State) -> Option<A!(Item<'_>)>,
    >,
}

#[gat]
impl<Item, State, Next>
    LendingIterator
for
    FromFn<Item, State, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(Item<'next>)
    ;

    fn next (self: &'_ mut FromFn<Item, State, Next>)
      -> Option< A!(Item<'_>) >
    {
        let Self { state, next, .. } = self;
        next(state)
    }
}
