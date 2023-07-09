#[derive(PartialEq, Eq, Debug)]
pub enum Result {
    AlreadySorted,
    SwapSorted,
    Sorted,
}

pub fn merge<T>(arr: &mut [T], mut left: usize, block_size: usize, right_ex_end: usize) -> Result
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    let mut right = left + block_size;

    // stable
    if arr[right] >= arr[right - 1] {
        return Result::AlreadySorted;
    }

    let mut left_ex_end = right;

    //     if arr[left] > arr[right_ex_end - 1] {
    //         // not stable if simple swapped elsewhere
    //         //         return Result::SwapSorted;
    //     }

    while left < left_ex_end
    // handles shorter right block
    && right < right_ex_end
    {
        if arr[right] > arr[left] {
            left = left + 1;
            continue;
        }

        let swap = arr[right];

        let mut mov_ind = right;
        while mov_ind > left {
            arr[mov_ind] = arr[mov_ind - 1];
            mov_ind = mov_ind - 1;
        }

        arr[left] = swap;
        left = left + 1;
        right = right + 1;
        left_ex_end = left_ex_end + 1;
        // moving left end is necessary in
        // order to maintain ordered invariant
        // that means all elements shifted to
        // right block must be compared against
        // remaining elements in right block if
        // any are there
    }

    Result::Sorted
}

// worst case n^2
//     [7, 8, 9, 10, 11, 12];
//     [1, 2, 3, 4 , 5 , 6 ];

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut arr = [4, 5, 2, 3];

        assert_eq!(Result::Sorted, merge(&mut arr, 0, 2, 4));
        assert_eq!([2, 3, 4, 5], arr);
    }

    #[test]
    fn basic_test2() {
        let mut arr = [0, 0, 4, 5, 2, 3, 0, 0];

        assert_eq!(Result::Sorted, merge(&mut arr, 2, 2, 6));
        assert_eq!([0, 0, 2, 3, 4, 5, 0, 0], arr);
    }

    #[test]
    fn skip_in_place_test() {
        let mut arr = [1, 2, 3, 6, 4, 7, 8, 9];

        assert_eq!(Result::Sorted, merge(&mut arr, 0, 4, 8));
        assert_eq!([1, 2, 3, 4, 6, 7, 8, 9], arr);
    }

    #[test]
    fn already_sorted_test() {
        let mut arr = [4, 5, 5, 6];

        assert_eq!(Result::AlreadySorted, merge(&mut arr, 0, 2, 4));
        assert_eq!([4, 5, 5, 6], arr);
    }

    #[test]
    fn already_sorted_test2() {
        let mut arr = [5, 5, 6, 6];

        assert_eq!(Result::AlreadySorted, merge(&mut arr, 0, 2, 4));
        assert_eq!([5, 5, 6, 6], arr);
    }

    #[test]
    fn left_index_overflow() {
        let mut arr = [5, 5, 5, 4, 6, 6];

        assert_eq!(Result::Sorted, merge(&mut arr, 0, 3, 6));
        assert_eq!([4, 5, 5, 5, 6, 6], arr);
    }

    #[test]
    fn right_index_overflow() {
        let mut arr = [5, 5, 5, 4, 4];

        assert_eq!(Result::Sorted, merge(&mut arr, 0, 3, 5));
        assert_eq!([4, 4, 5, 5, 5], arr);
    }

    #[test]
    fn mov_ind_increment_test() {
        let mut arr = [9, 10, 11, 1, 9, 10];

        assert_eq!(Result::Sorted, merge(&mut arr, 0, 3, 6));
        assert_eq!([1, 9, 9, 10, 10, 11], arr);
    }
}
