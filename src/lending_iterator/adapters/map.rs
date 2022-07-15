pub
struct Map<I, F, NewItemType>
where
    I : LendingIterator,
    NewItemType : HKT,
    for<'any>
        F : FnMut(
            [&'any (); 0],
            Item<'any, I>,
        ) -> A!(NewItemType<'any>)
    ,
{
    pub(in crate)
    iter: I,

    pub(in crate)
    map: F,

    pub(in crate)
    _phantom_ty: ::core::marker::PhantomData<fn() -> NewItemType>,
}

#[gat]
impl<I, NewItemType, F> LendingIterator
    for Map<I, F, NewItemType>
where
    I : LendingIterator,
    NewItemType : HKT,
    for<'any>
        F : FnMut(
            [&'any (); 0],
            Item<'any, I>,
        ) -> A!(NewItemType<'any>)
    ,
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(NewItemType<'next>)
    ;

    fn next (
        self: &'_ mut Map<I, F, NewItemType>,
    ) -> Option<A!(NewItemType<'_>)>
    {
        self.iter.next().map(|item| (self.map)([], item))
    }
}

pub
struct MapIntoIter<I, F>
(
    pub(in crate) I,
    pub(in crate) F,
)
where
    I : LendingIterator,
    for<'any>
        F : crate::utils::FnMut<Item<'any, I>>
    ,
;

impl<I, F, R>
    Iterator
for
    MapIntoIter<I, F>
where
    I : LendingIterator,
    F : FnMut(Item<'_, I>) -> R,
{
    type Item = R;

    fn next (
        self: &'_ mut MapIntoIter<I, F>,
    ) -> Option<R>
    {
        self.0.next().map(&mut self.1)
    }
}
