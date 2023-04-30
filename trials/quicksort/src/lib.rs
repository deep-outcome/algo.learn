#[allow(dead_code)]
#[allow(unconditional_recursion)]
fn sort(slc: &mut [usize]) {
    let len = slc.len();

    if len == 0 {
        return;
    }

    let part = part(slc, slc.len() - 1);
    
    sort(&mut slc[0..part]);
    sort(&mut slc[part + 1..len]);
}

/// partitions slice
fn part(slc: &mut [usize], max_ix: usize) -> usize {
    // chosing last elem as pivot is unwise
    // when slc sorted already (in any order)
    // results Ο(n²) time complexity
    let pivot = slc[max_ix];

    let mut swap;
    let mut writ_ix = 0;
    let mut read_ix = 0;

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

        assert_eq!(4, part(&mut arr, arr_len - 1));

        assert_eq!(exp_part, arr);
    }
}
