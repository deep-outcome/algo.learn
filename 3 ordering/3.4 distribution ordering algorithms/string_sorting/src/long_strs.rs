use std::collections::hash_map::HashMap;
use std::str;

#[allow(dead_code)]
pub fn sort(strs: &mut [&str]) {
    // classic approach is to create array with len = lenₘₐₓ(strs)
    // let assume using HashMap is more economic since for big lenₘₐₓ
    // many lens will not be present among `strs`
    let mut len_bucs = HashMap::<usize, Vec<&[u8]>>::new();

    // TC classicaly:
    // ϴ(lenₘₐₓ) for len-arrays initialization
    // ϴ(n) for putting each into respective
    //
    // SC classicaly:
    // ϴ(lenₘₐₓ) for len-arrays assuming initial 0 capacity
    // ϴ(n) for each &str copy assuming capacity grow by one at time
    //
    // HashMap time complexity is opaque but space complexity obviously downgrades to
    // ϴ(nₗₑₙₛ + n) holding same assumptions
    //
    // let assume TC ϴ(n)
    for byts in strs.iter().map(|x| x.as_bytes()) {
        let ix = byts.len() - 1;

        if let Some(buc) = len_bucs.get_mut(&ix) {
            buc.push(byts);
        } else {
            let mut buc = Vec::<&[u8]>::new();
            buc.push(byts);
            len_bucs.insert(ix, buc);
        }
    }

    // only basic ASCII alphabet + some other chars support
    // so any goes into 1 byte
    // SC: ϴ(lenₐₗₚₕₐ)
    let mut alpha_bucs = Vec::<Vec<&[u8]>>::with_capacity(95);

    // TC: ϴ(lenₐₗₚₕₐ)
    for _ in 0..95 {
        alpha_bucs.push(Vec::new());
    }

    let strs_len = strs.len();

    // SC: ϴ(n)
    let mut output = Vec::<&[u8]>::with_capacity(strs_len);

    // another opaque trade-off
    // what complexity is to sort-reverse`len_bucs_keys`?
    let mut len_bucs_keys = len_bucs.keys().collect::<Vec<&usize>>();
    len_bucs_keys.sort();
    len_bucs_keys.reverse();

    // TC: ϴ(nₗₑₙₛ * lenₐₗₚₕₐ) + ϴ(Σᵢlenᵢ)
    for len_ix in len_bucs_keys {
        for s in len_bucs.get(len_ix).unwrap() {
            to_alpha_bucs(&mut alpha_bucs, s, *len_ix);
        }

        for o in output.iter() {
            to_alpha_bucs(&mut alpha_bucs, o, *len_ix);
        }

        output.clear();

        // TC: ϴ(lenₐₗₚₕₐ)
        for alpha_ix in 0..95 {
            let buc = &mut alpha_bucs[alpha_ix];

            if buc.len() == 0 {
                continue;
            }

            for s in buc.iter() {
                output.push(s);
            }

            // it does not make much sense to re-allocate `alpha_bucs` each iteration
            // but `clear` does not truncate capacity so
            // so Ο(n) expectation on memory consumed by `alpha_bucs` is slightly
            // odd
            buc.clear();
        }
    }

    // TC: ϴ(n)
    for i in 0..strs_len {
        strs[i] = str::from_utf8(output[i]).unwrap();
    }
}

fn to_alpha_bucs<'a>(alpha_bucs: &mut Vec<Vec<&'a [u8]>>, s: &'a [u8], len_ix: usize) {
    // can panic at `alpha_bucs[…]` when using `String` composed of out of support elements
    alpha_bucs[s[len_ix] as usize - 32].push(s);
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
        // support for empty strings is trivial, so keep it omitted
        let mut test = [""];
        sort(&mut test);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn unsupp_elems_test1() {
        let mut test = [str::from_utf8(&[31]).unwrap(), "aaa"];
        sort(&mut test);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 95 but the index is 95")]
    fn unsupp_elems_test2() {
        let mut test = [str::from_utf8(&[127]).unwrap(), "aaa"];
        sort(&mut test);
    }
}
