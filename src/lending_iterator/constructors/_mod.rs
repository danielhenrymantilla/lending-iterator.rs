use super::*;

match_! {(
    from_fn,
    from_iter,
    repeat,
    windows_mut,
) {(
    $(
        $module:ident
    ),* $(,)?
) => (
    $(
        pub use self::$module::*;
        mod $module {
            use super::*;

            include!(concat!(stringify!($module), ".rs"));
        }
    )*
)}}
