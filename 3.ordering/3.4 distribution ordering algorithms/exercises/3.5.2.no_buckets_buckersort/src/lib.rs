use std::fmt::Display;
use std::ptr;

trait KeyProvider {
    fn key(&self) -> usize;
}

impl KeyProvider for u32 {
    fn key(&self) -> usize {
        *self as usize
    }
}

#[allow(dead_code)]
fn sort<T>(slc: &mut [&T])
where
    T: KeyProvider + Display,
{
    if slc.len() < 2 {
        return;
    }

    sort_unccked(slc)
}

fn sort_unccked<T>(slc: &mut [&T])
where
    T: KeyProvider + Display,
{
    // support for up to 5 same keys
    const KEYS_LEN: usize = 1_000_001 * 5;
    let mut cask = Vec::<*const T>::with_capacity(KEYS_LEN);

    unsafe { cask.set_len(KEYS_LEN) };

    for i in 0..KEYS_LEN {
        cask[i] = ptr::null();
    }

    'a: for t in slc.iter() {
        let mut key = t.key();

        key += key * 4;

        for _ in 1..=5 {
            let val = cask[key];
            if val.is_null() {
                cask[key] = *t;
                continue 'a;
            }

            key += 1;
        }

        panic!("Maximum repetition count exceeded. Item `{}`.", t);
    }

    let slc_len = slc.len();
    let mut ix = 0;

    for x in cask {
        if x.is_null() {
            continue;
        } else {
            slc[ix] = unsafe { x.as_ref() }.unwrap();

            ix = ix + 1;
            if ix == slc_len {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn basic_test() {
        let vals = [9, 5, 3, 2, 0];

        let iter = vals.iter();
        let mut criterion: Vec<&u32> = iter.clone().collect();
        criterion.sort();

        let mut vals: Vec<&u32> = iter.collect();
        sort_unccked(&mut vals);

        assert_eq!(criterion, vals);
    }

    #[test]
    fn repetition_test() {
        let vals = [9, 5, 3, 5, 2, 5, 0, 5, 5, 3, 4, 4, 6, 4, 4, 7, 8, 4, 9];

        let iter = vals.iter();
        let mut criterion: Vec<&u32> = iter.clone().collect();
        criterion.sort();

        let mut vals: Vec<&u32> = iter.collect();
        sort_unccked(&mut vals);

        assert_eq!(criterion, vals);
    }

    #[test]
    #[should_panic(expected = "Maximum repetition count exceeded. Item `5`.")]
    fn too_much_repetition_test() {
        let vals = [9, 5, 3, 5, 2, 5, 0, 5, 5, 5];

        let iter = vals.iter();
        let mut criterion: Vec<&u32> = iter.clone().collect();
        criterion.sort();

        let mut vals: Vec<&u32> = iter.collect();
        sort_unccked(&mut vals);

        assert_eq!(criterion, vals);
    }

    #[test]
    fn heavy_test() {
        let vals = [
            9, 5, 3, 5, 2, 5, 0, 5, 5, 3, 4, 4, 6, 4, 4, 7, 8, 4, 9, 1, 1, 3, 3, 1_000_000, 1000,
            1000, 9, 9,
        ];

        let iter = vals.iter();
        let mut criterion: Vec<&u32> = iter.clone().collect();
        criterion.sort();

        let mut vals: Vec<&u32> = iter.collect();
        sort_unccked(&mut vals);

        assert_eq!(criterion, vals);
    }
}
