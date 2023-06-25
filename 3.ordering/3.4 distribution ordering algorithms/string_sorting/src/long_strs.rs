use std::collections::hash_map::HashMap;
use std::str;

#[allow(dead_code)]
pub fn sort(strs: &mut [&str]) {
    // classic approach is to create array with len = lenₘₐₓ(strs)`
    // let assume using HashMap is more economic since for big lenₘₐₓ
    // many lens will not be present among `strs`
    let mut len_bucs = HashMap::<usize, Vec<&[u8]>>::new();

    // time complexity classicaly:
    // ϴ(lenₘₐₓ) for array initialization
    // ϴ(n) for putting each into respective
    //
    // space complexity classicaly:
    // ϴ(lenₘₐₓ) for len-arrays (assuming initial len=0)
    // ϴ(n) for each &str copy (assuming expansion by 1 [not typical])
    //
    // let accept HashMap time complexity opaque but space complexity obviously downgrades to
    // ϴ(nₗₑₙₛ) + ϴ(n)
    for byts in strs.iter().map(|x| x.as_bytes()) {
        let key = byts.len() - 1;

        if let Some(buc) = len_bucs.get_mut(&key) {
            buc.push(byts);
        } else {
            let mut buc = Vec::<&[u8]>::new();
            buc.push(byts);
            len_bucs.insert(key, buc);
        }
    }

    // only basic ASCII alphabet + some other chars support
    // let reuse `len_bucs` approach
    // spare 1ˢᵗ 32 position in exchange for opaque time complexity
    let mut alpha_bucs = HashMap::<u8, Vec<&[u8]>>::new(); // UTF-8 is 1–4 bytes, but ASCII fits 1
    let alpha_bucs_keys = (32u8..=126).collect::<Vec<u8>>();

    for key in alpha_bucs_keys.iter() {
        alpha_bucs.insert(*key, Vec::new());
    }

    let strs_len = strs.len();
    let mut output = Vec::<&[u8]>::with_capacity(strs_len);

    // another opaque trade-off
    // what complexity is to sort `len_bucs_keys`?, of which len?
    let mut len_bucs_keys = len_bucs.keys().collect::<Vec<&usize>>();
    len_bucs_keys.sort();
    len_bucs_keys.reverse();

    for len_k in len_bucs_keys {
        for s in len_bucs.get(len_k).unwrap() {
            to_alpha_bucs(&mut alpha_bucs, s, *len_k);
        }

        for o in output.iter() {
            to_alpha_bucs(&mut alpha_bucs, o, *len_k);
        }

        output.clear();

        for alpha_k in alpha_bucs_keys.iter() {
            let buc = alpha_bucs.get_mut(alpha_k).unwrap();

            if buc.len() == 0 {
                continue;
            }

            for s in buc.iter() {
                output.push(s);
            }

            buc.clear();
        }
    }

    for i in 0..strs_len {
        strs[i] = str::from_utf8(output[i]).unwrap();
    }
}

fn to_alpha_bucs<'a>(alpha_bucs: &mut HashMap<u8, Vec<&'a [u8]>>, s: &'a [u8], offset: usize) {
    use std::ops::Index;

    // can panic at `alpha_bucs.get_mut` when using `String` composed of out of support elements
    alpha_bucs.get_mut(s.index(offset)).unwrap().push(s);
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn basic_test() {
        let mut strs = ["bb", "aa", "a", "b"];

        let mut criterion = strs.clone();
        criterion.sort();

        sort(&mut strs);
        assert_eq!(criterion, strs);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn no_supp_for_empty_str() {
        // support for empty strings is trivial so keep it omitted
        let mut test = [""];
        sort(&mut test);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn unsupp_elems_test1() {
        let mut data = [str::from_utf8(&[31]).unwrap()];
        sort(&mut data);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn unsupp_elems_test2() {
        let mut data = [str::from_utf8(&[127]).unwrap()];
        sort(&mut data);
    }
}
