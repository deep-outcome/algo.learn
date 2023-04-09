pub fn merge_x<T>(arr: &mut [T], left: usize, right: usize, block_size: usize) -> bool
where
    T: Copy + PartialOrd,
{
    let right = left + block_size;
    let pre_split = &mut arr[left..right + block_size];

    let mut split = pre_split.split_at_mut(block_size);
    merge(&mut split.0, &mut split.1)
}

pub fn merge<T>(left: &mut [T], right: &mut [T]) -> bool
where
    T: Copy + PartialOrd,
{
    let right_len = right.len();

    // already sorted shortcut
    // respectively slices are just in swapped order
    // simply swapping elsewhere would be unstable
    if left[0] > right[right_len - 1] {
        return false;
    }

    let left_len = left.len();
    let mut swap;

    //         for li in 0..left_len {
    //     //         `>` maintains stability
    //     //         while `>=` would not
    //             if left[li] > right[0] {
    //                 swap = left[li];
    //                 left[li] = right[0];
    //
    //     //             balancing index
    //                 let mut bi = 1;
    //                 while bi < right_len && swap > right[bi] {
    //                     right[bi - 1] = right[bi];
    //                     bi = bi + 1;
    //                 }
    //
    //                 right[bi - 1] = swap;
    //             }
    //         }
    //
    //         return true;

    // more epxressive control loop

    let mut li = 0;
    loop {
        // `<=` maintains stability
        // while `<` would not
        while left[li] <= right[0] {
            li = li + 1;

            if li == left_len {
                return true;
            }
        }

        swap = left[li];
        left[li] = right[0];

        // balancing index
        let mut bi = 1;
        // `>` maintains stability
        // while `>=` would not
        while bi < right_len && swap > right[bi] {
            right[bi - 1] = right[bi];
            bi = bi + 1;
        }

        right[bi - 1] = swap;
    }

    // worst case n^2
    //     [7, 8, 9, 10, 11, 12];
    //     [1, 2, 3, 4 , 5 , 6 ];

    // not good case
    //     [6, 8, 9, 10, 11, 12];
    //     [1, 2, 3, 4 , 5 , 6 ];

    //
    //     [6, 8, 9, 10, 11, 12];
    //     [1, 2, 6,  8, 9 , 11];

    // not good case
    //     [5, 8, 9, 10, 11, 12];
    //     [1, 2, 3, 4 , 5 , 6 ];
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn unique_numbers() {
        let mut a = [1, 5, 8, 90, 133, 180];
        let mut b = [2, 4, 9, 80, 144, 150];

        let mut all = a.to_vec();
        all.append(&mut b.to_vec());
        all.sort();

        let all: &[usize] = &all;

        let (a_criterion, b_criterion) = all.split_at(6);

        assert!(merge(&mut a, &mut b));

        assert_eq!(a_criterion, a);
        assert_eq!(b_criterion, b);
    }

    #[test]
    fn left_index_overflow() {
        let mut a = [1, 1];
        let mut b = [2];

        assert!(merge(&mut a, &mut b));

        let mut a = [1, 2, 2];
        assert!(merge(&mut a, &mut b));
    }

    #[test]
    fn stability_test() {
        let mut a = StabilityTester::from([1, 2, 3, 4]);
        let mut b = StabilityTester::from([0, 2, 3, 5]);

        // designes expected order
        a[0].ref_ord = 11;
        a[1].ref_ord = 20;
        a[2].ref_ord = 30;
        a[3].ref_ord = 40;

        b[0].ref_ord = 10;
        b[1].ref_ord = 21;
        b[2].ref_ord = 31;
        b[3].ref_ord = 41;

        let mut all = a.clone();
        all.append(&mut b.clone());
        all.sort();

        let all: &[StabilityTester] = &all;

        let (a_criterion, b_criterion) = all.split_at(4);

        assert!(merge(&mut a, &mut b));

        assert_eq!(a_criterion, a);
        assert_eq!(b_criterion, b);

        test_ref_ord(&a);
        test_ref_ord(&b);

        fn test_ref_ord(x: &[StabilityTester]) {
            let mut test = -1;
            for a in x {
                let ref_ord = a.ref_ord as i64;
                assert!(test < ref_ord, "test {}, orid_ind {}", test, ref_ord);
                test = ref_ord;
            }
        }
    }

    #[test]
    fn shortcut_escape_test1() {
        let mut a = [7, 8];
        let mut b = [6, 6];

        assert!(merge(&mut a, &mut b) == false);
        assert_eq!([6, 6], b);
        assert_eq!([7, 8], a);
    }

    #[test]
    fn shortcut_escape_test2() {
        let mut a = [7, 8];
        let mut b = [6, 7];

        assert!(merge(&mut a, &mut b) == true);
        assert_eq!([6, 7], a);
        assert_eq!([7, 8], b);
    }

    #[derive(Copy, Clone, Debug)]
    struct StabilityTester {
        val: usize,
        ref_ord: usize,
    }

    impl StabilityTester {
        pub fn new(val: usize, ref_ord: usize) -> StabilityTester {
            StabilityTester { val, ref_ord }
        }

        pub fn from<const SIZE: usize>(x: [usize; SIZE]) -> Vec<Self> {
            x.iter()
                .enumerate()
                .map(|x| StabilityTester::new(*x.1, x.0))
                .collect::<Vec<StabilityTester>>()
        }
    }

    impl Eq for StabilityTester {}

    impl PartialEq for StabilityTester {
        fn eq(&self, other: &Self) -> bool {
            self.val.eq(&other.val)
        }
    }

    use std::cmp::Ordering;
    impl PartialOrd for StabilityTester {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.val.partial_cmp(&other.val)
        }
    }

    impl Ord for StabilityTester {
        fn cmp(&self, other: &Self) -> Ordering {
            self.val.cmp(&other.val)
        }
    }
}
