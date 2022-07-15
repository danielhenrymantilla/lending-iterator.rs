macro_rules! match_ {(
    ( $($input:tt)* ) $rules:tt
) => (
    macro_rules! __recurse__ $rules
    __recurse__! { $($input)* }
)}

macro_rules! cfg_match {
    ({ $($tt:tt)* }) => ({ cfg_match! { $($tt)* } });
    (_ => { $($expansion:tt)* } $(,)?) => ( $($expansion)* );
    (
        $cfg:meta => $expansion:tt $(,
        $($($rest:tt)+)? )?
    ) => (
        #[cfg($cfg)]
        cfg_match! { _ => $expansion } $($(

        #[cfg(not($cfg))]
        cfg_match! { $($rest)+ } )?)?
    );
}

attribute_alias! {
    #[apply(public_macro!)] =
        #[cfg_attr(feature = "better-docs",
            ::macro_vis::macro_vis(pub),
        )]
        #[cfg_attr(not(feature = "better-docs"),
            macro_export
        )]
    ;
}

attribute_alias! {
    #[apply(cfg_alloc)] =
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "better-docs",
            doc(cfg(feature = "alloc")),
        )]
    ;
}
