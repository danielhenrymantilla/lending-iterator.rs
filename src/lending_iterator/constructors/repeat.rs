/// I wish we could just use this…
#[cfg(any())]
pub
fn repeat<State> (state: State)
  -> impl   LendingIterator +
            for<'any>
                LendingIteratorඞItem<'any, T = &'any mut State>
            +
{
    from_fn::<HKT!(&mut State), _, _>(state, |it| Some(it))
}
pub
fn repeat<State> (state: State)
  -> Repeat<State>
{
    Repeat(state)
}

pub
struct Repeat<State>(
    State,
);

#[gat]
impl<State>
    LendingIterator
for
    Repeat<State>
{
    type Item<'next>
    where
        Self : 'next,
    =
        &'next mut State
    ;

    fn next (
        self: &'_ mut Repeat<State>,
    ) -> Option<&'_ mut State>
    {
        Some(&mut self.0)
    }
}
