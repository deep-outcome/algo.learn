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

    // EXPECTATIONS:
    // ⓵
    // let expect that our "ASCII alphabet" is quite long
    // thousand and thousand of letters available
    // but still they must upheld property that their codepoint
    // fits into usize (index type)
    //
    // this is true for any UTF-8 code point (4 bytes per one at max)
    // but not for user perceived character (grapheme clusters)
    // that are technically unlimited in length
    // (see http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
    // (check https://glitchtextgenerator.com/)
    //
    // (let say our "big" alphabet is alphabet combining many other alphabets where each
    // known letter is represented by exactly one code point)
    //
    // ⓶
    // if such "big" alphabet is used either there will be some discrete support for code
    // points "in between" or there must be some correlation that will make correction of
    // code point values to `alpha_bucs` index

    // from this point of view is more apt to use just some kind of map so no corrections nor
    // discrete support is present and also using some external lib like unicode_segmentation
    // sorting on whole graphemes can be implemented (for instance by using base letter as key
    // and some fine-sorting on modifiers in collection under key, check e.g.
    // https://www.w3.org/International/articles/definitions-characters/index.en#characters)
    // furthermore whole alphabet initialization for first iteration can be omitted

    // SC: ϴ(lenₐₗₚₕₐ)
    let mut alpha_bucs = Vec::<Vec<&[u8]>>::with_capacity(95);

    let strs_len = strs.len();

    // SC: ϴ(n)
    let mut output = Vec::<&[u8]>::with_capacity(strs_len);

    // TC: Ο(lenₘₐₓ² + lenₐₗₚₕₐ)
    // TC: Ω(lenₘₐₓ + lenₐₗₚₕₐ)
    // TC: Ο ∩ Ω = (lenₘₐₓ + lenₐₗₚₕₐ)
    // TC: ϴ(lenₘₐₓ + lenₐₗₚₕₐ) + ϴ(Σᵢlenᵢ)
    // TC: ϴ(Σᵢlenᵢ) = ϴ(s)
    // TC: ϴ(s + lenₐₗₚₕₐ) (since lenₐₗₚₕₐ is not negligible)
    // TC: s = sum of all lengths

    let mut alpha_max = 94;
    let mut curr_ix = len_bucs_len - 1;
    loop {
        // TC: Ο(lenₐₗₚₕₐ)
        // by narrowing our big alphabet down to max num in next iteration
        // some initializations are spared
        for _ in 0..=alpha_max {
            alpha_bucs.push(Vec::new());
        }

        for s in &mut len_bucs[curr_ix] {
            to_alpha_bucs(&mut alpha_bucs, s, curr_ix);
        }

        for o in output.iter_mut() {
            to_alpha_bucs(&mut alpha_bucs, o, curr_ix);
        }

        output.clear();

        // TC: ϴ(lenₐₗₚₕₐ) (not whole cyclus)
        for alpha_ix in 0..=alpha_max {
            let buc = &alpha_bucs[alpha_ix];

            if buc.len() == 0 {
                continue;
            }

            for s in buc.iter() {
                output.push(s);
            }
        }

        alpha_bucs.clear();

        if curr_ix == 0 {
            break;
        }
        curr_ix -= 1;

        alpha_max = 0;

        // iteration over previous `curr_ix` is suboptimal since this could be done
        // in respective loop above
        // for simplicity of this logic and that logic this optimization is
        // omitted
        for i in curr_ix..len_bucs_len {
            for s in &len_bucs[i] {
                let next = s[curr_ix] as usize - 32;

                if next > alpha_max {
                    alpha_max = next;
                }
            }
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
    fn load_test() {
        let mut strs = ["dfeau", "aqab", "aaca", "aqaa", "bbbz", "box", "y", "x"];

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
}
