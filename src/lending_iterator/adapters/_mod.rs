use super::*;

match_! {(
    filter,
    from_fn,
    fuse,
    map_and_then,
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
