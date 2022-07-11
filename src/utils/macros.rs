macro_rules! match_ {(
    ( $($input:tt)* ) $rules:tt
) => (
    macro_rules! __recurse__ $rules
    __recurse__! { $($input)* }
)}
