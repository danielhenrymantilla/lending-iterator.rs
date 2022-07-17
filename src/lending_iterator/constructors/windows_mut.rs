pub
fn windows_mut<'lt, T, const WINDOW_SIZE: usize> (
    slice: &'lt mut [T],
) -> WindowsMut<&'lt mut [T], WINDOW_SIZE>
{
    WindowsMut {
        slice,
        start: 0,
    }
}

pub
struct WindowsMut<Slice, const WINDOW_SIZE: usize> {
    slice: Slice,
    start: usize,
}

#[gat]
impl<'lt, T, const WINDOW_SIZE: usize>
    LendingIterator
for
    WindowsMut<&'lt mut [T], WINDOW_SIZE>
{
    type Item<'next>
    where
        Self : 'next,
    =
        &'next mut [T; WINDOW_SIZE]
    ;

    fn next (
        self: &'_ mut WindowsMut<&'lt mut [T], WINDOW_SIZE>,
    ) -> Option<&'_ mut [T;  WINDOW_SIZE]>
    {
        self.nth(0)
    }

    #[inline]
    fn nth (
        self: &'_ mut WindowsMut<&'lt mut [T], WINDOW_SIZE>,
        n: usize,
    ) -> Option<&'_ mut [T;  WINDOW_SIZE]>
    {
        let new_start = self.start.checked_add(n)?;
        let slice =
            self.slice
                .get_mut(new_start ..)?
                .get_mut(.. WINDOW_SIZE)?
        ;
        self.start = new_start + 1;
        Some(slice.try_into().unwrap())
    }
}
