use super::*;

match_! {(
    and_then,
    filter,
    from_fn,
    from_iter,
    fuse,
    into_iter,
    map,
    skip,
    take,
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
