pub mod mergesort {
    // Rust program for the above approach

    // gap calculation
    fn get_gap(fathom: usize) -> usize {
        if fathom == 1usize {
            return 0usize;
        }

        (fathom + 1usize) / 2usize
    }

    // function for swapping
    fn swap(nums: &mut [i32], x: usize, y: usize) {
        let temp = nums[x];
        nums[x] = nums[y];
        nums[y] = temp;
    }

    // merging subarrays using Shell's method
    // time complexity: O(n·log n)
    // space complexity: O(1)
    fn in_place_merge(nums: &mut [i32], first: usize, stopper: usize) {
        let mut gap = get_gap(stopper - first);

        while gap > 0usize {
            let mut close = first;
            let mut far = close + gap;

            while far < stopper {
                if nums[close] > nums[far] {
                    swap(nums, close, far);
                }

                close += 1usize;
                far += 1usize;
            }

            gap = get_gap(gap);
        }
    }

    // merge_sort makes log₂n recursive calls
    // each time calls in_place_merge(&mut [u32], usize, usize)
    // which takes n·log₂n steps:
    // time complexity sequence: (n·log₂n + 2·((ⁿ⁄₂)·log₂(ⁿ⁄₂)) + 4·((ⁿ⁄₄)·log₂(ⁿ⁄₄)) +.....+ 1)
    // time complexity: O(log n·(n·log n)) = O(n·(log n)²)
    // space complexity: O(1)
    pub fn merge_sort(nums: &mut [i32], start: usize, end: usize) {
        if start == end {
            return;
        }

        let left_half_end = (start + end) / 2;

        merge_sort(nums, start, left_half_end);
        merge_sort(nums, left_half_end + 1, end);
        in_place_merge(nums, start, end + 1);
    }

    fn print_nums(nums: &[i32]) {
        let mut res = String::from("");
        nums.iter().fold(&mut res, |acc, x| {
            acc.push_str(&x.to_string());
            acc.push(' ');
            acc
        });

        println!("{}", res.trim_end());
    }

    pub fn driver_code() {
        let mut nums = [12, 11, 13, 5, 6, 7];
        let len = nums.len();
        merge_sort(&mut nums, 0, len - 1usize);

        print_nums(&nums);
    }
}

#[cfg(test)]
mod tests {
    use super::mergesort;

    #[test]
    fn _1sized() {
        let mut nums = [-10];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 0);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    // 2-sized
    #[test]
    fn _2sized_ordered() {
        let mut nums = [11, 12];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 1);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _2sized_unordered() {
        let mut nums = [12, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 1);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    // 3-sized
    #[test]
    fn _3sized_ordered() {
        let mut nums = [11, 12, 13];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_unordered_permutation1() {
        let mut nums = [12, 11, 13];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_unordered_permutation2() {
        let mut nums = [12, 13, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_unordered_permutation3() {
        let mut nums = [13, 12, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_unordered_permutation4() {
        let mut nums = [13, 11, 12];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_unordered_permutation5() {
        let mut nums = [11, 13, 12];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_ordered_same_numbers() {
        let mut nums = [11, 11, 12];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_ordered_same_numbers_permutation1() {
        let mut nums = [12, 11, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _3sized_ordered_same_numbers_permutation2() {
        let mut nums = [11, 12, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 2);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    // 4-sized
    #[test]
    fn _4sized_ordered() {
        let mut nums = [11, 12, 13, 14];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation1() {
        let mut nums = [12, 11, 13, 14];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation2() {
        let mut nums = [12, 13, 11, 14];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation3() {
        let mut nums = [12, 13, 14, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation4() {
        let mut nums = [13, 12, 14, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation5() {
        let mut nums = [13, 14, 12, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation6() {
        let mut nums = [14, 13, 12, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation7() {
        let mut nums = [14, 12, 13, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation8() {
        let mut nums = [12, 14, 13, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_unordered_permutation9() {
        let mut nums = [12, 14, 13, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_ordered_same_numbers() {
        let mut nums = [11, 11, 12, 13];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_ordered_same_numbers_permutation1() {
        let mut nums = [11, 12, 11, 13];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_ordered_same_numbers_permutation2() {
        let mut nums = [11, 12, 13, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_ordered_same_numbers_permutation3() {
        let mut nums = [12, 11, 13, 11];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_ordered_same_numbers_permutation4() {
        let mut nums = [12, 11, 11, 13];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn _4sized_ordered_same_numbers_permutation5() {
        let mut nums = [13, 11, 11, 12];
        let mut criterion = nums.to_vec();

        mergesort::merge_sort(&mut nums, 0, 3);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn load1() {
        let mut nums = [-101, 13, 13, 11, 10, -33, -33, -33, 13, 11, 11, -101];
        let mut criterion = nums.to_vec();

        let len = nums.len();
        mergesort::merge_sort(&mut nums, 0, len - 1);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn load2() {
        let mut nums = [
            -101, 13, 13, 11, 10, -33, -33, -33, 13, 11, 11, -101, 10, 10, 8, 5, 110, 109, 108,
            110, -33,
        ];
        let mut criterion = nums.to_vec();

        let len = nums.len();
        mergesort::merge_sort(&mut nums, 0, len - 1);

        criterion.sort();
        assert_eq!(criterion, nums);
    }

    #[test]
    fn load3() {
        let mut nums = [
            -101, -102, 13, 13, 11, 10, -33, -33, -33, 13, 11, -102, -102, 11, 11, -101, 10, 10, 8,
            5, 110, 109, 108, 110, -33, -102,
        ];
        let mut criterion = nums.to_vec();

        let len = nums.len();
        mergesort::merge_sort(&mut nums, 0, len - 1);

        criterion.sort();
        assert_eq!(criterion, nums);
    }
}
