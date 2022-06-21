use super::*;

#[apply(Gat)]
impl<I, NewItemType> Lending<I, NewItemType>
where
    I : LendingIterator,
    NewItemType : HKT,
{
    pub
    fn map<'usability, F> (
        self: Lending<I, NewItemType>,
        f: F,
    ) -> Map<I, NewItemType, F>
    where
        F : for<'n> FnMut([&'n (); 0], Item<'n, I>) -> A!(NewItemType<'n>),
        Map<I, NewItemType, F> : LendingIterator,
    {
        let Self { iter, new_item_type } = self;

        #[gat]
        impl<I, NewItemType, F> LendingIterator
            for Map<I, NewItemType, F>
        where
            I : LendingIterator,
            NewItemType : HKT,
            F : for<'n> FnMut([&'n (); 0], <I as LendingIterator>::Item<'n>) -> A!(NewItemType<'n>),
        {
            type Item<'next> = A!(NewItemType<'next>);

            fn next (self: &'_ mut Map<I, NewItemType, F>)
              -> Option<A!(NewItemType<'_>)>
            {
                let Self { iter, f, .. } = self;
                iter.next().map(|item| f([], item))
            }
        }

        Map { iter, new_item_type, f }
    }

    pub
    fn and_then<'usability, F> (
        self: Lending<I, NewItemType>,
        f: F,
    ) -> AndThen<I, NewItemType, F>
    where
        for<'n>
            F : FnMut([&'n (); 0], <I as LendingIterator>::Item<'n>)
                  -> Option< A!(NewItemType<'n>) >
        ,
        AndThen<I, NewItemType, F> : LendingIterator,
    {
        let Self { iter, new_item_type } = self;

        #[gat]
        impl<I, NewItemType, F> LendingIterator
            for AndThen<I, NewItemType, F>
        where
            I : LendingIterator,
            NewItemType : HKT,
            for<'n>
                F : FnMut([&'n (); 0], <I as LendingIterator>::Item<'n>)
                      -> Option< A!(NewItemType<'n>) >
            ,
        {
            type Item<'next> = A!(NewItemType<'next>) ;

            fn next (self: &'_ mut AndThen<I, NewItemType, F>)
              -> Option< A!(NewItemType<'_>) >
            {
                let Self { iter, f, .. } = self;
                iter.next().and_then(|item| f([], item))
            }
        }

        AndThen { iter, new_item_type, f }
    }
}

#[apply(Gat)]
pub
struct Map<I, NewItemType, F>
where
    I : LendingIterator,
    NewItemType : HKT,
    F : for<'n> FnMut([&'n (); 0], <I as LendingIterator>::Item<'n>) -> A!(NewItemType<'n>),
{
    iter: I,
    new_item_type: PhantomData<NewItemType>,
    f: F,
}

#[apply(Gat)]
pub
struct AndThen<I, NewItemType, F>
where
    I : LendingIterator,
    NewItemType : HKT,
    for<'n>
        F : FnMut([&'n (); 0], <I as LendingIterator>::Item<'n>)
              -> Option< A!(NewItemType<'n>) >
    ,
{
    iter: I,
    new_item_type: PhantomData<NewItemType>,
    f: F,
}

pub
struct Filter<I, F> {
    pub(in super)
    iter: I,

    pub(in super)
    should_yield: F,
}

#[gat]
impl<I, F> LendingIterator for Filter<I, F>
where
    I : LendingIterator,
    F : FnMut(&'_ Item<'_, I>) -> bool,
{
    type Item<'next>
    where
        Self : 'next,
    =
        <I as LendingIterator>::Item<'next>
    ;

    fn next (self: &'_ mut Filter<I, F>)
      -> Option<Item<'_, I>>
    {
        use ::polonius_the_crab::prelude::*;
        let mut iter = &mut self.iter;
        polonius_loop!(|iter| -> Option<Item<'polonius, I>> {
            let ret = iter.next();
            if matches!(&ret, Some(it) if (self.should_yield)(it).not()) {
                polonius_continue!();
            }
            polonius_return!(ret);
        })
    }
}
