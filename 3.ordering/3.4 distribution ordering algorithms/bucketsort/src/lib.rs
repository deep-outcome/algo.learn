#![allow(dead_code)]

trait BucketKey {
    fn bucket_key(&self) -> usize;
}

#[cfg(test)]
mod bucketsort_via_ptr {
    use super::{bucketsort_via_ptr, BucketKey};

    #[derive(Debug, PartialEq, Eq)]
    struct NonCopyStruct<'a> {
        name: &'a str,
        order: u32,
    }

    impl<'a> NonCopyStruct<'a> {
        fn new(name: &'a str, order: u32) -> NonCopyStruct {
            NonCopyStruct { name, order }
        }
    }

    impl BucketKey for NonCopyStruct<'_> {
        fn bucket_key(&self) -> usize {
            self.order as usize
        }
    }

    #[test]
    fn basic_test() {
        let mut test = [
            Box::new(NonCopyStruct::new("four", 4)),
            Box::new(NonCopyStruct::new("three", 3)),
            Box::new(NonCopyStruct::new("four_second", 4)),
            Box::new(NonCopyStruct::new("two", 2)),
            Box::new(NonCopyStruct::new("one", 1)),
            Box::new(NonCopyStruct::new("one_second", 1)),
        ];

        let mut criterion: Vec<Box<NonCopyStruct>> = test
            .iter()
            .map(|x| Box::new(NonCopyStruct::new(x.name, x.order)))
            .collect();

        criterion.sort_by_key(|x| x.bucket_key());

        bucketsort_via_ptr(&mut test, 4);
        assert_eq!(criterion, test);
    }

    #[test]
    #[should_panic(expected = "Key not in available buckets.")]
    fn invalid_range_test() {
        let mut test = [
            Box::new(NonCopyStruct::new("", 1)),
            Box::new(NonCopyStruct::new("", 0)),
        ];

        bucketsort_via_ptr(&mut test, 0);
    }

    #[test]
    fn want_to_be_short_escape_test() {
        let mut test = [
            Box::new(NonCopyStruct::new("", 4)),
            Box::new(NonCopyStruct::new("", 3)),
        ];

        let criterion = [
            Box::new(NonCopyStruct::new("", 3)),
            Box::new(NonCopyStruct::new("", 4)),
        ];

        bucketsort_via_ptr(&mut test, 1_000_000);
        assert_eq!(criterion, test);
    }
}

#[cfg(test)]
mod bucketsort_by_val {
    use super::{bucketsort_by_val, BucketKey};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct CopyStruct<'a> {
        name: &'a str,
        order: u32,
    }

    impl<'a> CopyStruct<'a> {
        fn new(name: &'a str, order: u32) -> CopyStruct {
            CopyStruct { name, order }
        }
    }

    impl BucketKey for CopyStruct<'_> {
        fn bucket_key(&self) -> usize {
            self.order as usize
        }
    }

    #[test]
    fn basic_test() {
        let test = [
            CopyStruct::new("four", 4),
            CopyStruct::new("three", 3),
            CopyStruct::new("four_second", 4),
            CopyStruct::new("two", 2),
            CopyStruct::new("one", 1),
            CopyStruct::new("one_second", 1),
        ];

        let mut criterion = test.to_vec();
        criterion.sort_by_key(|x| x.bucket_key());

        let test = bucketsort_by_val(&test, 4);
        assert_eq!(criterion, test);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn invalid_range_test() {
        let mut test = [CopyStruct::new("", 1), CopyStruct::new("", 0)];
        bucketsort_by_val(&mut test, 0);
    }

    #[test]
    fn want_to_be_short_escape_test() {
        let test = [CopyStruct::new("", 4), CopyStruct::new("", 3)];

        let criterion = [CopyStruct::new("", 3), CopyStruct::new("", 4)];

        let test = bucketsort_by_val(&test, 1_000_000);
        assert_eq!(criterion.to_vec(), test);
    }
}

fn bucketsort_by_val<T>(items: &[T], max: usize) -> Vec<T>
where
    T: BucketKey + Copy,
{
    let items_len = items.len();
    if items_len < 1 {
        return items.to_vec();
    }

    let buckets_len = max + 1;
    let mut buckets = Vec::with_capacity(buckets_len);
    let buckets_populator = buckets.spare_capacity_mut();

    for i in 0..=max {
        // push causes allocation of Vec space by Rust internal memory rules
        // this results in reallocation (not just extension of memory block) when
        // block is full
        // let assume it groves cheaply just as if just extends its block
        // and that this happens in 1-at-time manner
        buckets_populator[i].write(Vec::with_capacity(0)); // 0 = act to play on memory demand θ(n+r)
    }

    unsafe {
        buckets.set_len(buckets_len);
    }

    for i in 0..items_len {
        let it = &items[i]; // using reference to avoid whole T copy
        buckets[it.bucket_key()].push(it); // push understood as described above
    }

    let mut res = Vec::with_capacity(items_len);
    let res_populator = res.spare_capacity_mut();

    // since buckets were chosen to hold reference, it is not possible to write
    // results into input slice
    let mut res_index = 0;
    for buc_index in 0..=max {
        let buc = &buckets[buc_index];
        let buc_len = buc.len();

        if buc_len > 0 {
            for val_index in 0..buc_len {
                res_populator[res_index].write(*buc[val_index]);
                res_index += 1;
            }

            if res_index == items_len {
                // escape shortcut when result is full; not part of original implementation
                // breaks θ(n+r) time complexity on results writing
                break;
            }
        }
    }

    unsafe {
        res.set_len(items_len);
    }

    res
}

use std::mem;
use std::ptr;

fn bucketsort_via_ptr<T>(items: &mut [Box<T>], max: usize)
where
    T: BucketKey,
{
    let items_len = items.len();
    if items_len < 1 {
        return;
    }

    let buckets_len = max + 1;
    let mut buckets = Vec::with_capacity(buckets_len);
    let buckets_populator = buckets.spare_capacity_mut();

    for i in 0..=max {
        // push causes allocation of Vec space by Rust internal memory rules
        // this results in reallocation (not just extension of memory block) when
        // block is full
        // let assume it groves cheaply just as if just extends its block
        // and that this happens 1-at-time manner
        buckets_populator[i].write(Vec::with_capacity(0)); // 0 = act to play on memory demand θ(n+r)
    }

    unsafe {
        buckets.set_len(buckets_len);
    }

    let mut items_ptr = items.as_ptr();

    for _ in 0..items_len {
        let boxed_t;
        unsafe {
            boxed_t = items_ptr.read();
        }
        let bucket_key = boxed_t.bucket_key();

        if bucket_key < buckets_len {
            buckets[bucket_key].push(boxed_t); // push understood as described above
            unsafe {
                items_ptr = items_ptr.add(1);
            }
        } else {
            // if memory is not freed of doublets, wrong memory is priority error (not panic)
            mem::forget(boxed_t);
            free_doubled_boxs(buckets);
            panic!("Key not in available buckets.")
        }
    }

    let mut items_ptr = items.as_mut_ptr();

    let mut writes_count = 0;
    for buc_index in 0..=max {
        let buc = &buckets[buc_index];
        let buc_len = buc.len();

        if buc_len > 0 {
            let mut buc_ptr = buc.as_ptr();

            for _ in 0..buc_len {
                unsafe {
                    ptr::write(items_ptr, buc_ptr.read());
                    buc_ptr = buc_ptr.add(1);
                    items_ptr = items_ptr.add(1);
                }

                writes_count += 1;
            }

            if writes_count == items_len {
                // escape shortcut when result is full; not part of original implementation
                // breaks θ(n+r) time complexity on results writing
                break;
            }
        }
    }

    free_doubled_boxs(buckets);

    fn free_doubled_boxs<T>(mut bucs: Vec<Vec<Box<T>>>) {
        while let Some(mut b) = bucs.pop() {
            while let Some(boxed) = b.pop() {
                mem::forget(boxed);
            }
        }
    }
}
