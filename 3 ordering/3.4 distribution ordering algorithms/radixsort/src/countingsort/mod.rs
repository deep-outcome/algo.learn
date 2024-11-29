#[allow(dead_code)]
fn radixsort(nums: &mut [u32], mut cycls: usize) {
    let nums_len = nums.len();
    assert!(cycls > 0, "Cannot sort by zero digit position.");

    if nums_len < 2 {
        return;
    }

    let odd_cycls = cycls & 1 == 1;

    let mut order = 1;
    let mut counters = [0; 10];

    let mut aux = Vec::with_capacity(nums_len);
    unsafe { aux.set_len(nums_len) }

    let mut output: &mut [u32] = &mut aux;
    let mut input = nums;
    let mut swap;

    loop {
        for n in input.iter() {
            // if n == 0, then write is possible immediately
            let num_tranc = *n / order;
            // num_tranc == 0, then write is possible immediately
            let sorter = (num_tranc % 10) as usize;
            counters[sorter] = counters[sorter] + 1;
        }

        // cumulative sum specifies resulting index
        for i in 1..=9 {
            counters[i] = counters[i] + counters[i - 1]
        }

        // beware of reversal order
        // otherwise numbers stability is broken
        // earlier number gets higher index
        for n in input.iter().rev() {
            let n = *n;
            let num_tranc = n / order;
            let sorter = (num_tranc % 10) as usize;

            let index = counters[sorter] - 1;
            output[index] = n;

            counters[sorter] = index;
        }

        cycls = cycls - 1;
        if cycls == 0 {
            // let use knowledge to spare extra
            // copying from aux vec into original
            if odd_cycls {
                for i in 0..nums_len {
                    input[i] = output[i]
                }
            }

            return;
        }

        // reset counters
        for i in 0..=9 {
            counters[i] = 0;
        }

        // uplift order
        order = order * 10;

        // spare writing one to another
        // just swap them and continue
        swap = input;
        input = output;
        output = swap;
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn general_test() {
        let mut nums = [9, 8, 7, 6, 5, 4, 2, 1, 0];
        let mut criterion = nums.clone();
        criterion.sort();
        radixsort(&mut nums, 1);

        assert_eq!(criterion, nums);
    }

    #[test]
    fn general_test2() {
        let mut nums = [12, 21];
        radixsort(&mut nums, 1);

        assert_eq!([21, 12], nums);
    }

    #[test]
    fn general_test3() {
        let mut nums = [721, 912];
        radixsort(&mut nums, 2);

        assert_eq!([912, 721], nums);
    }

    #[test]
    fn stability_test() {
        let mut nums = [91, 81, 71];
        radixsort(&mut nums, 1);

        assert_eq!([91, 81, 71], nums);
    }

    #[test]
    fn cumulatives_decrement_test() {
        let mut nums = [91, 81, 71, 0, 0, 0];
        radixsort(&mut nums, 2);

        assert_eq!([0, 0, 0, 71, 81, 91], nums);
    }

    #[test]
    fn counters_reset_test() {
        let mut nums = [100, 90, 81, 72, 63, 54, 45, 36, 27, 18, 9];
        let mut criterion = nums.clone();
        criterion.sort();

        radixsort(&mut nums, 3);

        assert_eq!(criterion, nums);
    }
}
