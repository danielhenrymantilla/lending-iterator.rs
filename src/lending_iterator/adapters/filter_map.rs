/// The <code>impl [LendingIterator]</code> returned by [`.filter_map()`][
/// LendingIterator::filter_map()].
pub
struct FilterMap<I, F, NewItemType>
where
    I : LendingIterator,
    NewItemType : HKT,
    for<'any>
        F : FnMut(
            [&'any I; 0],
            Item<'any, I>,
        ) -> Option<A!(NewItemType<'any>)>
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
    for FilterMap<I, F, NewItemType>
where
    I : LendingIterator,
    NewItemType : HKT,
    for<'any>
        F : FnMut(
            [&'any I; 0],
            Item<'any, I>,
        ) -> Option<A!(NewItemType<'any>)>
    ,
{
    type Item<'next>
    where
        Self : 'next,
    =
        A!(NewItemType<'next>)
    ;

    fn next (
        self: &'_ mut FilterMap<I, F, NewItemType>,
    ) -> Option<A!(NewItemType<'_>)>
    {
        self.iter.next().and_then(|item| (self.map)([], item))
    }
}

/// The <code>impl [LendingIterator]</code> returned by
/// [`.filter_map_into_iter()`][LendingIterator::filter_map_into_iter()].
pub
struct FilterMapIntoIter<I, F>
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
    FilterMapIntoIter<I, F>
where
    I : LendingIterator,
    F : FnMut(Item<'_, I>) -> Option<R>,
{
    type Item = R;

    fn next (
        self: &'_ mut FilterMapIntoIter<I, F>,
    ) -> Option<R>
    {
        self.0.next().and_then(&mut self.1)
    }
}
