/// Holds pivot indexes.
struct Pivs {
    min_ix: isize,
    max_ix: isize,
}

#[allow(dead_code)]
fn sort(slc: &mut [usize], min_ix: isize, max_ix: isize) {
    if min_ix < max_ix {
        let pivs = part(slc, min_ix as usize, max_ix as usize);

        let piv_min_ix = pivs.min_ix;
        let piv_max_ix = pivs.max_ix;

        sort(slc, min_ix, piv_min_ix - 1);
        sort(slc, piv_min_ix + 1, piv_max_ix - 1);
        sort(slc, piv_max_ix + 1, max_ix);
    }
}

fn part(slc: &mut [usize], min_ix: usize, max_ix: usize) -> Pivs {
    let mut lef_piv = slc[min_ix];
    let mut rig_piv = slc[max_ix];

    let mut swap;
    // swap pivots if their oder is reversal
    if lef_piv > rig_piv {
        swap = lef_piv;
        lef_piv = rig_piv;
        rig_piv = swap;
    }

    let mut lef_wri_ix = min_ix + 1;
    let mut rig_wri_ix = max_ix - 1;

    let mut rea_ix = lef_wri_ix;

    // = is needed for instance because of 3-item array
    // +---+---+----+
    // | 9 | 8 | 10 |
    // +---+---+----+
    // lef_wri_ix = 1, rig_wri_ix = 1, rea_ix = 1 at start
    //
    // or when rea_ix simply advances to rig_wri_ix
    // for instance
    // +---+---+---+----+
    // | 9 | 8 | 7 | 10 |
    // +---+---+---+----+
    // after 1ˢᵗ iteration (8 was written to itself)
    // lef_wri_ix = 2, rig_wri_ix = 2, rea_ix = 2
    while rea_ix <= rig_wri_ix {
        if slc[rea_ix] < lef_piv {
            swap = slc[rea_ix];
            slc[rea_ix] = slc[lef_wri_ix];
            slc[lef_wri_ix] = swap;

            lef_wri_ix = lef_wri_ix + 1;
        } else if slc[rea_ix] > rig_piv {
            // find some lesser then pivot not read before
            // ending on current rea_ix
            while slc[rig_wri_ix] > rig_piv && rea_ix < rig_wri_ix {
                rig_wri_ix = rig_wri_ix - 1;
            }

            swap = slc[rea_ix];
            slc[rea_ix] = slc[rig_wri_ix];
            slc[rig_wri_ix] = swap;

            rig_wri_ix = rig_wri_ix - 1;

            // tech: `continue` is sufficient but then cycle count will be 
            // less predictible and `else if` condition can be checked in vain

            if slc[rea_ix] < lef_piv {
                swap = slc[rea_ix];
                slc[rea_ix] = slc[lef_wri_ix];
                slc[lef_wri_ix] = swap;

                lef_wri_ix = lef_wri_ix + 1;
            }
        }
        rea_ix = rea_ix + 1;
    }
    lef_wri_ix = lef_wri_ix - 1;
    rig_wri_ix = rig_wri_ix + 1;

    slc[min_ix] = slc[lef_wri_ix];
    slc[max_ix] = slc[rig_wri_ix];

    slc[lef_wri_ix] = lef_piv;
    slc[rig_wri_ix] = rig_piv;

    Pivs {
        min_ix: lef_wri_ix as isize,
        max_ix: rig_wri_ix as isize,
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    // fn sort()

    #[test]
    fn sort_basic_test() {
        let mut arr = [18, 43, 43, 0, 9, 11, 23, 3, 4, 5, 1, 0, 99, 12, 2];
        let max_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, max_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_2_elem_arr() {
        let mut arr = [10, 1];

        sort(&mut arr, 0, 1);
        assert_eq!([1, 10], arr);

        let mut arr = [1, 2];
        sort(&mut arr, 0, 1);
        assert_eq!([1, 2], arr);
    }

    #[test]
    fn sort_3_elem_arr() {        

        let mut arr = [2, 1, 3];
        sort(&mut arr, 0, 2);
        assert_eq!([1, 2, 3], arr);

        let mut arr = [2, 4, 3];
        sort(&mut arr, 0, 2);
        assert_eq!([2, 3, 4], arr);
    }

    #[test]
    fn sort_equal_pivots() {
        let mut arr = [10, 43, 43, 0, 9, 11, 23, 3, 4, 5, 1, 0, 99, 12, 10];
        let max_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, max_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_all_greater_than_p1() {
        let mut arr = [9, 12, 11, 10, 13];
        let max_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, max_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_all_lesser_than_p1() {
        let mut arr = [9, 8, 7, 6, 13];
        let max_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, max_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_all_greater_than_p2() {
        let mut arr = [9, 16, 15, 14, 13];
        let max_ix = arr.len() - 1;
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, max_ix as isize);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn sort_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        let max_ix = arr.len() - 1;
        let criterion = arr.clone();

        sort(&mut arr, 0, max_ix as isize);
        assert_eq!(criterion, arr);
    }

    // fn part()

    #[test]
    fn part_basic_test() {
        let mut arr = [18, 43, 43, 0, 9, 11, 23, 3, 4, 5, 1, 0, 99, 12, 2];
        let max_ix = arr.len() - 1;
        let criterion = [1, 0, 0, 2, 9, 11, 12, 3, 4, 5, 18, 43, 99, 43, 23];

        part(&mut arr, 0, max_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_equal_pivots() {
        let mut arr = [10, 43, 43, 0, 9, 11, 23, 3, 4, 5, 1, 0, 99, 12, 10];
        let max_ix = arr.len() - 1;
        let criterion = [3, 0, 1, 0, 9, 5, 4, 10, 10, 11, 43, 43, 99, 12, 23];

        part(&mut arr, 0, max_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_all_greater_than_p1() {
        let mut arr = [9, 12, 11, 10, 13];
        let max_ix = arr.len() - 1;
        let criterion = arr.clone();

        part(&mut arr, 0, max_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_all_lesser_than_p1() {
        let mut arr = [9, 8, 7, 6, 13];
        let max_ix = arr.len() - 1;
        let criterion = [6, 8, 7, 9, 13];

        part(&mut arr, 0, max_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_all_greater_than_p2() {
        let mut arr = [9, 16, 15, 14, 13];
        let max_ix = arr.len() - 1;
        let criterion = [9, 13, 15, 14, 16];

        part(&mut arr, 0, max_ix);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_3_elem_arr() {        

        let mut arr = [2, 1, 3];
        part(&mut arr, 0, 2);
        assert_eq!([1, 2, 3], arr);

        let mut arr = [2, 4, 3];
        part(&mut arr, 0, 2);
        assert_eq!([2, 3, 4], arr);
    }

    #[test]
    fn part_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        let max_ix = arr.len() - 1;
        let criterion = arr.clone();

        part(&mut arr, 0, max_ix);
        assert_eq!(criterion, arr);
    }
}
