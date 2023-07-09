use std::str;

#[allow(dead_code)]

fn sort(strs: &mut [&str]) {
    if strs.len() < 2 {
        return;
    }

    sort_focus(strs);
}

fn sort_focus(strs: &mut [&str]) {
    // TC: ϴ(n) (imagine flat for loop)
    let len_bucs_len = strs.iter().map(|x| x.len()).max().unwrap();

    // SC: ϴ(lenₘₐₓ)
    let mut len_bucs = Vec::<Vec<&[u8]>>::with_capacity(len_bucs_len);

    // TC: ϴ(lenₘₐₓ)
    // SC: ϴ(0) assuming 0 initial capacity
    for _ in 0..len_bucs_len {
        len_bucs.push(Vec::new())
    }

    // TC: ϴ(n) for each
    // SC: ϴ(n) for each assuming capacity grow by one at time
    for s in strs.iter() {
        len_bucs.get_mut(s.len() - 1).unwrap().push(s.as_bytes());
    }

    // only basic ASCII alphabet + some other chars support
    // so any goes into 1 byte
    // SC: ϴ(lenₐₗₚₕₐ)
    let mut alpha_bucs = Vec::<Vec<&[u8]>>::with_capacity(95);

    let strs_len = strs.len();

    // SC: ϴ(n)
    let mut output = Vec::<&[u8]>::with_capacity(strs_len);

    // TC: Ο(lenₘₐₓ² * lenₐₗₚₕₐ)
    // TC: Ω(lenₘₐₓ * lenₐₗₚₕₐ)
    // TC: Ο ∩ Ω = (lenₘₐₓ * lenₐₗₚₕₐ)
    // TC: ϴ(lenₘₐₓ * lenₐₗₚₕₐ) + ϴ(Σᵢlenᵢ)
    // TC: ϴ(Σᵢlenᵢ) = ϴ(s)
    // TC: s = sum of all lengths
    for len_ix in (0..len_bucs_len).rev() {
        // TC: ϴ(lenₐₗₚₕₐ)
        for _ in 0..=94 {
            // allocation in each iteration is suboptimal
            // preallocation and `clear` on other hand
            // can break Ο(n) space complexity (capacity is not "cleared")
            alpha_bucs.push(Vec::new());
        }

        for s in &len_bucs[len_ix] {
            to_alpha_bucs(&mut alpha_bucs, s, len_ix);
        }

        for o in output.iter() {
            to_alpha_bucs(&mut alpha_bucs, o, len_ix);
        }

        output.clear();

        // TC: ϴ(lenₐₗₚₕₐ)
        for alpha_ix in 0..=94 {
            let buc = &alpha_bucs[alpha_ix];

            if buc.len() == 0 {
                continue;
            }

            for s in buc.iter() {
                output.push(s);
            }
        }

        alpha_bucs.clear();
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
        // to add support for empty strings is trivial, so keep it omitted
        let mut test = ["", ""];
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
