use super::*;

match_! {(
    filter,
    filter_map,
    fuse,
    into_iter,
    map,
    skip,
    take,
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
        mod $module {
            use super::*;

            include!(concat!(stringify!($module), ".rs"));
        }
    )*
)}}
