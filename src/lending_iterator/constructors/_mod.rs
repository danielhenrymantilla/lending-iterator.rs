use super::*;

match_! {(
    from_fn,
    from_iter,
    #[apply(cfg_futures)]
    from_stream,
    repeat_mut,
    windows_mut_,
) {(
    $(
        $(#[$attrs:meta])*
        $module:ident
    ),* $(,)?
) => (
    $(
        $(#[$attrs])*
        pub use self::$module::*;
        $(#[$attrs])*
        pub(in crate)
        mod $module {
            use super::*;

            include!(concat!(stringify!($module), ".rs"));
        }
    )*
)}}

/// Extension trait based convenience method version of [`from_iter()`].
///
/// [`from_iter()`]: crate::from_iter()
#[extension(pub trait into_lending_iter)]
impl<I, IntoIter> I
where
    I : IntoIterator<IntoIter = IntoIter>,
    IntoIter : Iterator,
{
    /// Extension trait based convenience method version of [`from_iter()`].
    ///
    /// [`from_iter()`]: crate::from_iter()
    fn into_lending_iter (
        self,
    ) -> constructors::FromIter<IntoIter>
    {
        constructors::from_iter(self)
    }
}

/// Extension trait based convenience method version of [`windows_mut()`].
#[extension(pub trait windows_mut)]
impl<T> [T] {
    /// Extension trait based convenience method version of [`windows_mut()`].
    fn windows_mut<const WINDOW_SIZE: usize> (&mut self)
      -> constructors::WindowsMut<&mut [T], WINDOW_SIZE>
    {
        constructors::windows_mut(self)
    }
}
