use std::fmt::Display;

trait KeyProvider {
    fn key(&self) -> usize;
}

impl KeyProvider for u32 {
    fn key(&self) -> usize {
        *self as usize
    }
}

#[allow(dead_code)]
fn sort<T>(slc: &mut [T])
where
    T: KeyProvider + Display + Clone,
{
    if slc.len() < 2 {
        return;
    }

    sort_unccked(slc)
}

fn sort_unccked<T>(slc: &mut [T])
where
    T: KeyProvider + Display + Clone,
{
    // support for up to 5 same keys
    const KEYS_LEN: usize = 1_000_001 * 5;
    let mut cask = Vec::<Option<T>>::with_capacity(KEYS_LEN);

    unsafe { cask.set_len(KEYS_LEN) };

    for i in 0..KEYS_LEN {
        cask[i] = None;
    }

    'a: for t in slc.iter() {
        let mut key = t.key();

        key += key * 4;

        for _ in 1..=5 {
            let val = &cask[key];
            if val.is_none() {
                cask[key] = Some(t.clone());
                continue 'a;
            }

            key += 1;
        }

        panic!("Maximum repetition count exceeded. Item `{}`.", t);
    }

    let slc_len = slc.len();
    let mut ix = 0;

    for x in cask {
        if let Some(y) = x {
            slc[ix] = y;

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
        let mut vals = [9, 5, 3, 2, 0];

        let mut criterion = vals.clone();
        criterion.sort();

        sort_unccked(&mut vals);
        assert_eq!(criterion, vals);
    }

    #[test]
    fn repetition_test() {
        let mut vals = [9, 5, 3, 5, 2, 5, 0, 5, 5, 3, 4, 4, 6, 4, 4, 7, 8, 4, 9];

        let mut criterion = vals.clone();
        criterion.sort();

        sort_unccked(&mut vals);
        assert_eq!(criterion, vals);
    }

    #[test]
    #[should_panic(expected = "Maximum repetition count exceeded. Item `5`.")]
    fn too_much_repetition_test() {
        let mut vals = [9, 5, 3, 5, 2, 5, 0, 5, 5, 5];

        let mut criterion = vals.clone();
        criterion.sort();

        sort_unccked(&mut vals);
        assert_eq!(criterion, vals);
    }

    #[test]
    fn heavy_test() {
        let mut vals = [
            9, 5, 3, 5, 2, 5, 0, 5, 5, 3, 4, 4, 6, 4, 4, 7, 8, 4, 9, 1, 1, 3, 3, 1_000_000, 1000,
            1000, 9, 9,
        ];

        let mut criterion = vals.clone();
        criterion.sort();

        sort_unccked(&mut vals);
        assert_eq!(criterion, vals);
    }
}
