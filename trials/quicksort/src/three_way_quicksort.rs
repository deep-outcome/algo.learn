#[derive(Debug)]
// pivotal range expressed with upper and lower exclusive index
struct PivotalExRange(isize, isize);

// 3-way partioning
fn part(slc: &mut [usize], min_ix: usize, max_ix: usize) -> PivotalExRange {
    // indexes for seeking nums in respective sides
    let mut gre_seek_ix = min_ix;

    // since pivot is choosen last one
    // it is fine to start seeking one earlier
    let mut les_seek_ix = max_ix - 1;

    // indexes for writing pivot equals to nose and tail
    let mut left_piv_ex_ix = gre_seek_ix;
    let mut right_piv_ex_ix = les_seek_ix;

    let pivot = slc[max_ix];
    let mut swap;

    loop {
        // seek for ≥ from left
        while slc[gre_seek_ix] < pivot {
            gre_seek_ix += 1;
        }

        // seek for ≤ from right
        while slc[les_seek_ix] > pivot && les_seek_ix != min_ix {
            les_seek_ix -= 1;
        }

        // this line is little opaque
        // let check with some cases when condition renders true
        //
        // 1. pivot is already in place and ∀ slc[0–n-2] < pivot
        // +---+---+---+---+---+
        // | 3 | 1 | 4 | 2 | 5 |
        // +---+---+---+---+---+
        // les_seek_ix=3, gre_seek_ix=4, gre_seek_ix > les_seek_ix
        //
        // 2. pivot is in reversal place and ∀ slc[0–n-2] > pivot
        // +---+---+---+---+---+
        // | 3 | 5 | 4 | 2 | 1 |
        // +---+---+---+---+---+
        // les_seek_ix=0, gre_seek_ix=0, gre_seek_ix == les_seek_ix
        //
        // 3. there is only one equal and respective side
        // are ∀ slc[0–k-1] < pivot ∧ slc[k+1–n] > pivot
        // +---+---+---+---+---+
        // | 2 | 1 | 5 | 6 | 5 |
        // +---+---+---+---+---+
        // les_seek_ix=2, gre_seek_ix=2, gre_seek_ix == les_seek_ix
        //
        // 4. …
        if gre_seek_ix as usize >= les_seek_ix {
            break;
        }

        // organize lesser ones onto left and
        // greater ones onto right side
        swap = slc[gre_seek_ix];
        slc[gre_seek_ix] = slc[les_seek_ix];
        slc[les_seek_ix] = swap;

        // organize all left pivot equals to
        // left start
        if slc[gre_seek_ix] == pivot {
            swap = slc[gre_seek_ix];
            slc[gre_seek_ix] = slc[left_piv_ex_ix];
            slc[left_piv_ex_ix] = swap;

            left_piv_ex_ix += 1;
        }

        // same for right side
        if slc[les_seek_ix] == pivot {
            swap = slc[les_seek_ix];
            slc[les_seek_ix] = slc[right_piv_ex_ix];
            slc[right_piv_ex_ix] = swap;

            right_piv_ex_ix -= 1;
        }

        // proceed with next indexes
        gre_seek_ix += 1;
        les_seek_ix -= 1;
    }

    slc[max_ix] = slc[gre_seek_ix];
    slc[gre_seek_ix] = pivot;

    // usize would underflow when pivot belongs to 0 index
    let mut left_mid_piv_ex_ix: isize = gre_seek_ix as isize - 1;
    let mut right_mid_piv_ex_ix = gre_seek_ix + 1;

    let mut read_ix;

    // move all pivot equals from nose next to pivot on left
    read_ix = min_ix;
    while read_ix < left_piv_ex_ix {
        swap = slc[read_ix];
        slc[read_ix] = slc[left_mid_piv_ex_ix as usize];
        slc[left_mid_piv_ex_ix as usize] = swap;

        read_ix += 1;
        left_mid_piv_ex_ix -= 1;
    }

    // move all pivot equals from tail next to pivot on right
    read_ix = max_ix - 1;
    while read_ix > right_piv_ex_ix {
        swap = slc[read_ix];
        slc[read_ix] = slc[right_mid_piv_ex_ix];
        slc[right_mid_piv_ex_ix] = swap;

        read_ix -= 1;
        right_mid_piv_ex_ix += 1;
    }

    PivotalExRange(left_mid_piv_ex_ix, right_mid_piv_ex_ix as isize)
}

/// 3-way quicksort is optimized for sorting arrays having many same items
/// key to this is partioning capable of processing all pivot occurences in
/// one run
#[allow(dead_code)]
fn sort(slc: &mut [usize], min_ix: isize, max_ix: isize) {
    assert!(min_ix > -2, "-1 is only one allowed min_ix negative value.");
    assert!(max_ix > -2, "-1 is only one allowed max_ix negative value.");

    if min_ix >= max_ix {
        return;
    }

    let piv_ex_rang = part(slc, min_ix as usize, max_ix as usize);

    // piv_ex_rang.0 can easily be -1 when pivot belongs to 0 index
    // piv_ex_rang.1 can easily be max_ix+1 when pivot belongs to max_ix
    sort(slc, min_ix, piv_ex_rang.0);
    sort(slc, piv_ex_rang.1, max_ix);
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    // fn sort() tests

    #[test]
    fn sort_min_ge_max() {
        let criterion = [2, 1];
        let mut arr = criterion.clone();

        sort(&mut arr, 1, 1);
        assert_eq!(criterion, arr);

        sort(&mut arr, 2, 1);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_basic_test() {
        let mut arr = [1, 2, 4, 3, 9, 8, 7];
        let last_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, last_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_pivot_in_place() {
        let mut arr = [1, 7, 2, 4, 3, 5, 8, 9];
        let last_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, last_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_pivot_in_reversal_place() {
        let mut arr = [9, 2, 4, 3, 5, 8, 0];
        let last_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, last_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_complex_test() {
        let mut arr = [6, 5, 5, 4, 4, 3, 3, 1, 2, 3];
        let mut criterion = arr.clone();
        criterion.sort();
        let last_ix = arr.len() - 1;

        sort(&mut arr, 0, last_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_complex_test2() {
        let mut arr = [6, 5, 3, 3, 2, 1, 1, 2, 3];
        let mut criterion = arr.clone();
        criterion.sort();
        let last_ix = arr.len() - 1;

        sort(&mut arr, 0, last_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_complex_test3() {
        let mut arr = [6, 5, 3, 3, 2, 1, 1, 2, 3, 6, 5, 5, 4, 4, 1, 2, 3, 3, 3];
        let mut criterion = arr.clone();
        criterion.sort();
        let last_ix = arr.len() - 1;

        sort(&mut arr, 0, last_ix as isize);
        assert_eq!(criterion, arr);
    }

    // fn part() tests

    // basically there is no greater then pivot
    #[test]
    fn part_pivot_in_place() {
        // part should just swap pivot with itself
        let mut arr = [8, 7, 5, 6, 3, 1, 9];
        let last_ix = arr.len() - 1;
        let criterion = arr.clone();

        part(&mut arr, 0, last_ix);
        assert_eq!(criterion, arr);
    }

    // basically there is no lesser then pivot
    // specific case of pivot belongs somewhere case
    #[test]
    fn part_pivot_in_reversal_place() {
        // part should swap pivot with 1st gt (ge) num
        let mut arr = [2, 4, 3, 0];
        let last_ix = arr.len() - 1;

        part(&mut arr, 0, last_ix);
        assert_eq!([0, 4, 3, 2], arr);
    }

    #[test]
    fn part_equal_one_found_by_both_seekers() {
        // part should just swap pivot with another one
        let mut arrs = [[7, 6, 8, 8], [8, 10, 9, 8]];

        for a in arrs.iter_mut() {
            let last_ix = a.len() - 1;
            let criterion = a.clone();

            part(a, 0, last_ix);
            assert_eq!(criterion, *a);
        }
    }

    #[test]
    fn part_swapping_lessers_with_greaters() {
        // lessers should be on left and greaters on right
        // of pivot
        let mut arr = [6, 5, 2, 1, 3];
        let criterion = [1, 2, 3, 6, 5];
        let last_ix = arr.len() - 1;

        part(&mut arr, 0, last_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_nose_filling_exhausting() {
        let mut arr = [6, 5, 5, 4, 4, 3, 3, 1, 2, 3];
        let criterion = [1, 2, 3, 3, 3, 4, 5, 5, 6, 4];
        let last_ix = arr.len() - 1;

        part(&mut arr, 0, last_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_tail_filling_exhausting() {
        let mut arr = [6, 5, 3, 3, 2, 1, 1, 2, 3];
        let criterion = [2, 1, 1, 2, 3, 3, 3, 6, 5];
        let last_ix = arr.len() - 1;

        part(&mut arr, 0, last_ix);
        assert_eq!(criterion, arr);
    }
}
