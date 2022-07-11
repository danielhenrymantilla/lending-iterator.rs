use super::*;

#[test]
fn inlined_windows_mut ()
{
    let mut array = [0, 1, 2, 3, 4, 5, 6];
    let slice = &mut array[..];
    let mut start = 0;
    let mut window_iter =
        lending_iter_from_fn::<HKT!(&mut [u8]), _, _>(slice, |it| Some(it))

            .and_then_lending(HKT!(&mut [u8])) // <'n> => &'n mut [u8]))
            .with(|[], slice| Some({
                let to_yield = slice.get_mut(start ..)?.get_mut(..2)?;
                start += 1;
                to_yield
            }))

            // .map_lending(HKT!(&mut [u8; 2]))
            .map_lending_mut::<[u8; 2], _>(|[], slice| slice.try_into().unwrap())

            .filter(|&&mut [fst, _]| fst != 0)
    ;
    while let Some(&mut [fst, ref mut snd]) = <_ as LendingIterator>::next(&mut window_iter) {
        *snd += fst;
    }
    assert_eq!(
        [0, 1, 3, 6, 10, 15, 21],
        array,
    );
}
