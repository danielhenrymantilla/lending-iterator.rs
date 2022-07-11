pub
struct FromFn<State, Item, Next>
where
    Item : HKT,
    Next : FnMut(&'_ mut State) -> Option< A!(Item<'_>) >,
{
    pub(in crate)
    state: State,

    pub(in crate)
    item_type: PhantomData<fn() -> Item>,

    pub(in crate)
    next: Next,
}

#[gat]
impl<State, Item, Next>
    LendingIterator
for
    FromFn<State, Item, Next>
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

    fn next (self: &'_ mut FromFn<State, Item, Next>)
      -> Option< A!(Item<'_>) >
    {
        let Self { state, next, .. } = self;
        next(state)
    }
}
