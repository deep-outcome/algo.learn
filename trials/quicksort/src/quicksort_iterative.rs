#[allow(dead_code)]
fn sort(slc: &mut [usize]) {
    let mut ixes_stack = Vec::<usize>::new();

    ixes_stack.push(0);
    ixes_stack.push(slc.len() - 1);

    let mut min_ix;
    let mut max_ix;

    while ixes_stack.len() != 0 {
        max_ix = ixes_stack.pop().unwrap();
        min_ix = ixes_stack.pop().unwrap();

        let piv_ix = part(slc, min_ix, max_ix);

        // either there can be:
        // 0 — isize cast is needed when piv_ix=0,
        // 1 — then piv_ix==side-index,
        // or more — execution branch,
        // elems at each side of piv_ix

        let on_left_ix = piv_ix as isize - 1;
        if on_left_ix > min_ix as isize {
            ixes_stack.push(min_ix);
            ixes_stack.push(on_left_ix as usize);
        }

        let on_right_ix = piv_ix + 1;
        if on_right_ix < max_ix {
            ixes_stack.push(on_right_ix);
            ixes_stack.push(max_ix);
        }
    }
}

/// partitions slice
fn part(slc: &mut [usize], min_ix: usize, max_ix: usize) -> usize {
    // chosing last elem as pivot is unwise
    // when slc sorted already (in any order)
    // results Ο(n²) time complexity
    let pivot = slc[max_ix];

    let mut swap;
    let mut writ_ix = min_ix;
    let mut read_ix = min_ix;

    while read_ix < max_ix {
        if slc[read_ix] < pivot {
            swap = slc[writ_ix];
            slc[writ_ix] = slc[read_ix];
            slc[read_ix] = swap;

            writ_ix = writ_ix + 1;
        }

        read_ix = read_ix + 1;
    }

    slc[max_ix] = slc[writ_ix];
    slc[writ_ix] = pivot;

    writ_ix
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn sort_basic_test() {
        let mut arr = [35, 3456, 11, 245, 890, 39, 9902, 13, 54];
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr);

        assert_eq!(criterion, arr);
    }

    #[test]
    fn part_basic_test() {
        let mut arr = [35, 3456, 11, 245, 890, 39, 9902, 13, 54];
        let arr_len = arr.len();
        let exp_part = [35, 11, 39, 13, 54, 3456, 9902, 245, 890];

        assert_eq!(4, part(&mut arr, 0, arr_len - 1));

        assert_eq!(exp_part, arr);
    }
}
