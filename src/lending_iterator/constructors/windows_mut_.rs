/// Creates an <code>impl [LendingIterator]</code> over the **sliding windows**
/// of a slice, **with `&mut` / exclusive** access over them (yields `&mut [T]`
/// slices).
///
/// This is thus "simply" the `mut` counter-part of [`windows()`],
/// but for the `mut` playing a key role w.r.t. semantics and borrows: indeed,
/// **the different sliding windows may overlap**! This goes against the
/// exclusivity of `&mut` references, so **the different windows cannot
/// coëxist!**
///
/// This is something that the traditional [`Iterator`] trait cannot express,
/// thence this [`LendingIterator`] definition which can.
///
///   - This is a free function version of the [`.windows_mut()`] method
///     provided by the eponymous [extension trait](https://docs.rs/extension_traits).
///
///     That is, feel free to check out that extension method, since in practice
///     it's even more ergonomic to use.
///
/// [`windows()`]: https://doc.rust-lang.org/1.62.0/std/primitive.slice.html#method.windows
/// [`.windows_mut()`]: trait@super::windows_mut#impl-windows_mut<T>-for-%5BT%5D
pub
fn windows_mut<T, const WINDOW_SIZE: usize> (
    slice: &mut [T],
) -> WindowsMut<&mut [T], WINDOW_SIZE>
{
    WindowsMut {
        slice,
        start: 0,
    }
}

/// The <code>impl [LendingIterator]</code> returned by [`windows_mut()`].
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

    fn next<'next> (
        self: &'next mut WindowsMut<&'lt mut [T], WINDOW_SIZE>,
    ) -> Option<&'next mut [T;  WINDOW_SIZE]>
    {
        self.nth(0)
    }

    #[inline]
    fn nth<'a> (
        self: &'a mut WindowsMut<&'lt mut [T], WINDOW_SIZE>,
        n: usize,
    ) -> Option<&'a mut [T;  WINDOW_SIZE]>
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
