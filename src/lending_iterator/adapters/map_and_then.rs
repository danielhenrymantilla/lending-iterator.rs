pub
struct With<_0, _1>(
    pub(in crate) _0,
    pub(in crate) _1,
);

pub
struct MapLending<I : LendingIterator, R : HKT> (
    pub(in crate) I,
    pub(in crate) R,
);

impl<I, R> MapLending<I, R>
where
    I : LendingIterator,
    R : HKT,
{
    pub
    fn with<F> (
        self: MapLending<I, R>,
        f: F,
    ) -> With<MapLending<I, R>, F>
    where
        F : for<'n> FnMut([&'n (); 0], Item<'n, I>) -> A!(R<'n>),
    {
        With(self, f)
    }
}

#[gat]
impl<I, R, F>
    LendingIterator
for
    With<MapLending<I, R>, F>
where
    I : LendingIterator,
    R : HKT,
    F : for<'n> FnMut([&'n (); 0], Item<'n, I>) -> A!(R<'n>),
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(R<'next>)
    ;

    fn next (
        self: &'_ mut With<MapLending<I, R>, F>,
    ) -> Option<A!(R<'_>)>
    {
        //      0w0
        self. 0_0.0_0 .next().map(|item| (self.1)([], item))
    }
}

pub
struct AndThenLending<I : LendingIterator, R : HKT> (
    pub(in crate) I,
    pub(in crate) R,
);

impl<I, R> AndThenLending<I, R>
where
    I : LendingIterator,
    R : HKT,
{
    pub
    fn with<F> (
        self: AndThenLending<I, R>,
        f: F,
    ) -> With<Self, F>
    where
        F : for<'n> FnMut([&'n (); 0], Item<'n, I>) -> Option<A!(R<'n>)>,
    {
        With(self, f)
    }
}

#[gat]
impl<I, R, F>
    LendingIterator
for
    With<AndThenLending<I, R>, F>
where
    I : LendingIterator,
    R : HKT,
    F : for<'n> FnMut([&'n (); 0], Item<'n, I>) -> Option<A!(R<'n>)>,
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(R<'next>)
    ;

    fn next (
        self: &'_ mut With<AndThenLending<I, R>, F>,
    ) -> Option<A!(R<'_>)>
    {
        //      0w0
        self. 0_0.0_0 .next().and_then(|item| (self.1)([], item))
    }
}
