pub
struct FromFn<Item, State, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    pub
    state: State,

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
