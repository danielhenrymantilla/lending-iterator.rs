use {
    ::core::{
        marker::Unpin,
    },
    ::futures::{
        stream::{
            Next,
            Stream,
            StreamExt,
        },
    },
};

/// "Lifts" and converts an [`Unpin`] [`Stream`] into an
/// <code>impl [LendingIterator]</code> which lends futures (an `S`,
/// say).
///
/// [`Stream`]: https://docs.rs/futures/^0.3.21/futures/stream/trait.Stream.html
/// [`StreamExt`]: https://docs.rs/futures/^0.3.21/futures/stream/trait.StreamExt.html
/// [`BoxFuture`]: https://docs.rs/futures/^0.3.21/futures/future/type.BoxFuture.html
///
/**  - ```rust
    # ::futures::executor::block_on(async {
    use {
        ::futures::{
            prelude::*,
        },
        ::lending_iterator::{
            prelude::*,
        },
    };


    let stream = stream::iter(1..=3);
    let mut lending_iterator = lending_iterator::from_stream(stream);
    loop {
        if let Some(item) = lending_iterator.next().unwrap().await {
            println!("Got `{}`", item);
        } else {
            break;
        }
    }
    # })
    ``` */
///
/// This could be useful to get access to the more lifetime-powerful adapters of
/// this crate (compared to the adapters on [`StreamExt`], for instance).
///
///   - That being said, said adapters often requires the explicitly-turbofished
///     `HKT` parameter, which in turn requires **nameable types**.
///
///     Sadly, most <code>impl [Future]</code> are unnameable!. So you'll mostly
///     need to use [`BoxFuture`]s and whatnot.
///
///     For `nightly` users, however, `type_alias_impl_trait` can be leveraged
///     to avoid unnecessary [`Box`]ing. Keep it in mind!
///
/// [Future]: ::core::future::Future
/// [`Box`]: ::alloc::boxed::Box
///
/// For instance, here is what a `&mut`-based "unfold" async iterator
/// construction would look like:
///
/**  - ```rust
    # ::futures::executor::block_on(async {
    use {
        ::futures::{
            future::BoxFuture,
            prelude::*,
        },
        ::lending_iterator::{
            prelude::*,
        },
    };

    let mut iter =
        lending_iterator::repeat_mut(0)
            .map::<HKT!(BoxFuture<'_, Option<i32>>), _>(
                |[], state: &mut _| async move {
                    if *state <= 2 {
                        let yielded = *state * 2;
                        *state += 1;
                        Some(yielded)
                    } else {
                        None
                    }
                }
                .boxed()
            )
    ;
    let mut result = vec![];
    loop {
        if let Some(item) = iter.next().unwrap().await {
            result.push(item);
        } else {
            break;
        }
    }
    assert_eq!(result, [0, 2, 4]);
    # })
    ``` */
///
#[apply(cfg_futures)]
pub
fn from_stream<S : Stream + Unpin> (
    stream: S,
) -> FromStream<S>
{
    FromStream(stream)
}

/// The <code>impl [LendingIterator]</code> returned by [`from_stream()`].
#[apply(cfg_futures)]
pub
struct FromStream<S : Stream + Unpin>(
    S,
);

#[apply(cfg_futures)]
#[gat]
impl<S : Stream + Unpin>
    LendingIterator
for
    FromStream<S>
{
    type Item<'next>
    where
        Self : 'next,
    =
        Next<'next, S>
    ;

    fn next (
        self: &'_ mut FromStream<S>,
    ) -> Option<
            Next<'_, S>
        >
    {
        Some(self.0.next())
    }
}
