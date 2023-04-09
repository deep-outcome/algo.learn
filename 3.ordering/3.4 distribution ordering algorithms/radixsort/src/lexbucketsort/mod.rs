pub mod key;

use key::KeyProducer;

struct SortResult {
    full_cycl_cnt: usize,
    key_len: usize,
}

#[allow(dead_code)]
impl SortResult {
    pub fn full_cycl_cnt(&self) -> usize {
        self.full_cycl_cnt
    }

    pub fn key_len(&self) -> usize {
        self.key_len
    }
}

#[allow(dead_code)]
fn radixsort(nums: &mut [u64], max: u64, nums_fit_max: bool) -> Option<SortResult> {
    if nums.len() < 2 {
        return None;
    }

    let mut k_pducer = KeyProducer::new(max, nums.len());

    let cycl_limit = if nums_fit_max {
        k_pducer.keys_cnt()
    } else {
        usize::MAX
    };

    bucketsort(nums, &mut k_pducer, cycl_limit);
    println!("{} {}", k_pducer.cycl_count(), k_pducer.keys_cnt());

    Some(SortResult {
        full_cycl_cnt: k_pducer.cycl_count(),
        key_len: k_pducer.keys_cnt(),
    })
}

// greater idea about radixsort is in sorting very large numbers
// while number of buckets is terse, i.e. r ≤ any_num
// lexbucketsort is ϴ(k×(n+r))
// radixsort is ϴ(logₙr×n)
// for num = nᵝ, r ≤ nᵝ ⇒ log r ≤ β log n ⇒ logₙr ≤ β logₙn ⇒ logₙr ≤ β ⇒ ϴ(β×n)=ϴ(β×logₙr×n)
// so polynomical large nums can be sorted at linear time complexity
//
// lesser one is about spare buckets initiations
// when n is much lesser than r
// e.g. n=10, r=1_000_000
// ϴ(n+r) ≫ ϴ(logₙr×n)
// this ideal applies as long as any_num ≤ r
pub fn bucketsort(items: &mut [u64], k_pducer: &mut KeyProducer, cycl_lim: usize) {
    let items_len = items.len();
    let buckets_len = k_pducer.max_key() + 1;

    let mut buckets = Vec::with_capacity(buckets_len);
    let buckets_populator = buckets.spare_capacity_mut();

    for i in 0..buckets_len {
        buckets_populator[i].write(Vec::with_capacity(0));
    }

    unsafe {
        buckets.set_len(buckets_len);
    }

    for _ in 0..cycl_lim {
        for i in 0..items_len {
            let val = items[i];
            let key = k_pducer.key(val);

            buckets[key].push(val);
        }

        // usable mostly for linear complexity challenge since there is no other
        // option to know cycl count beforehand, check with KeyProducer.keys_cnt
        // for details
        // also short quits logarithmic challenge when key_limit is incorrectly specified
        // (greater than actual nums)
        if k_pducer.true_keys_cnt() == 0 {
            break;
        }

        let mut writes_index = 0;
        for buc in buckets.iter_mut() {
            let buc_len = buc.len();

            if buc_len > 0 {
                for i in 0..buc_len {
                    items[writes_index] = buc[i];
                    writes_index = writes_index + 1;
                }

                // in last cycle may seems redundant
                // but Vec drop call basically does
                // same with its items
                buc.clear();

                if writes_index == items_len {
                    break;
                }
            }
        }

        k_pducer.fin_round();
    }
}

#[cfg(test)]
mod test {
    use super::{radixsort, SortResult};

    #[test]
    fn basic_test() {
        let mut nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let mut expectation = nums.clone();
        expectation.sort();

        let res = radixsort(&mut nums, 9, true);
        assert_eq!(expectation, nums);

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(1, res.full_cycl_cnt);
        assert_eq!(1, res.key_len);
    }

    #[test]
    fn bucs_len_test() {
        let mut nums = [8, 7, 6, 5 ,4 ,3, 2, 1, 0];
        let mut expectation = nums.clone();
        expectation.sort();

        _ = radixsort(&mut nums, 8, true);
        assert_eq!(expectation, nums);
    }
    
     #[test]
    fn bucs_clear_test() {
        let mut nums = [88, 77, 66, 55 ,44 ,33, 22, 11, 0];
        let mut expectation = nums.clone();
        expectation.sort();

        _ = radixsort(&mut nums, 88, true);
        assert_eq!(expectation, nums);
    }
    
    #[test]
    fn stability_test() {
        let mut nums = [81, 71, 61, 51];        

        _ = radixsort(&mut nums, 1, true);
        assert_eq!([81, 71, 61, 51], nums);
    }
    
    #[test]
    // this should sort by 1-digit key
    fn radixsort_correctly_sorts_different_digit_count_nums() {
        let mut nums = [1_888_111, 999_888, 999, 111, 11, 1];
        let mut expectation = nums.clone();
        expectation.sort();

        let res = radixsort(&mut nums, 1_888_111, true);
        assert_eq!(expectation, nums);

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(7, res.full_cycl_cnt);
        assert_eq!(7, res.key_len);
    }

    #[test]
    // this should sort by 2-digit key
    fn radixsort_correctly_sorts_different_digit_count_nums2() {
        let mut nums = [
            2_999_111, 1_888_111, 999_888, 888_999, 999, 111, 21, 11, 0, 1,
        ];
        let mut expectation = nums.clone();
        expectation.sort();

        let res = radixsort(&mut nums, 2_999_111, true);
        assert_eq!(expectation, nums);

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(4, res.full_cycl_cnt);
        assert_eq!(4, res.key_len);
    }

    #[test]
    #[allow(non_snake_case)]
    fn radixsort_incorrect_max__greater() {
        let mut nums = [23, 100, 71];

        let res = radixsort(&mut nums, 1000, true);
        assert_eq!([23, 71, 100], nums);

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(3, res.full_cycl_cnt);
        assert_eq!(4, res.key_len);
    }

    #[test]
    #[allow(non_snake_case)]
    fn radixsort_incorrect_max__lesser() {
        let mut nums = [23, 100, 71];

        let res = radixsort(&mut nums, 9, true);
        // it means sorting only on last digit
        assert_eq!([100, 71, 23], nums);

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(1, res.full_cycl_cnt);
        assert_eq!(1, res.key_len);
    }

    #[test]
    fn radixsort_linear_max() {
        let mut nums = [999_999_999, 1_000_003, 1_000_002, 1_000_001];
        let mut expectation = nums.clone();
        expectation.sort();

        let res = radixsort(&mut nums, 9999, false);
        assert_eq!(expectation, nums);

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(9, res.full_cycl_cnt);
        assert_eq!(4, res.key_len);
    }

    #[test]
    fn radixsort_too_short_nums() {
        let mut nums = vec![0; 1];

        assert!(radixsort(&mut nums).is_none());
        nums.pop();
        assert!(radixsort(&mut nums).is_none());

        fn radixsort(nums: &mut [u64]) -> Option<SortResult> {
            super::radixsort(nums, 0, false)
        }
    }
}
