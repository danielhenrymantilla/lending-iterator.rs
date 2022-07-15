//! Crate not intended for direct use.
//! Use https:://docs.rs/lending-iterator instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template
#![allow(nonstandard_style, unused_imports)]

use ::core::{
    mem,
    ops::Not as _,
};
use ::proc_macro::{
    TokenStream,
};
use ::proc_macro2::{
    Span,
    TokenStream as TokenStream2,
    TokenTree as TT,
};
use ::quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    Result, // Explicitly shadow it
    spanned::Spanned,
};

///
#[proc_macro] pub
fn HKT (
    input: TokenStream,
) -> TokenStream
{
    HKT_impl(input.into())
    //  .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`lending_iterator::HKT!`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}

fn hkt_lifetime(span: Span) -> Lifetime {
    Lifetime::new("'ඞ", span)
}

fn HKT_impl (
    input: TokenStream2,
) -> Result<TokenStream2>
{
    use ::syn::visit_mut;

    let mut input: Type = parse2(input)?;
    visit_mut::VisitMut::visit_type_mut(
        &mut {
            struct ReplaceLifetimeVisitor;
            impl visit_mut::VisitMut for ReplaceLifetimeVisitor {
                fn visit_lifetime_mut (
                    self: &'_ mut Self,
                    lifetime: &'_ mut Lifetime,
                )
                {
                    if lifetime.ident == "_" {
                        *lifetime = hkt_lifetime(lifetime.span());
                    }
                }

                fn visit_type_reference_mut (
                    self: &'_ mut Self,
                    ty_ref: &'_ mut TypeReference,
                )
                {
                    visit_mut::visit_type_reference_mut(self, ty_ref);
                    let and_token = &ty_ref.and_token;
                    ty_ref.lifetime.get_or_insert_with(|| {
                        hkt_lifetime(and_token.span())
                    });
                }

                // Stop uneliding when encountering `fn()` pointer signatures or
                // `Fn` trait bounds.
                fn visit_parenthesized_generic_arguments_mut (
                    self: &'_ mut Self,
                    _: &'_ mut ParenthesizedGenericArguments,
                )
                {
                    /* do not subrecurse: stop visiting! */
                }
                fn visit_type_bare_fn_mut (
                    self: &'_ mut Self,
                    _: &'_ mut TypeBareFn,
                )
                {
                    /* do not subrecurse: stop visiting! */
                }
            }
            ReplaceLifetimeVisitor
        },
        &mut input,
    );
    Ok(input.into_token_stream())
}
