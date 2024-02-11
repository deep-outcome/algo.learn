#![allow(dead_code)]

pub const MAX_LEVELS: usize = 25;

pub struct FixMinBinHeap<T>
where
    T: PartialOrd + Clone + Default,
{
    data: Box<[T]>,
    len: usize,
}

use std::convert::From;
impl<T> From<&[T]> for FixMinBinHeap<T>
where
    T: PartialOrd + Clone + Default,
{
    /// Final capacity is aligned to maximal leaf capacity.
    fn from(slice: &[T]) -> FixMinBinHeap<T> {
        let len = slice.len();

        let limit = (len as f64).log2();
        let levels = limit.ceil() as usize;

        assert!(
            levels <= MAX_LEVELS,
            "Input length is greater than maximal heap item count support."
        );

        let mut heap = FixMinBinHeap::<T>::new(levels);

        let data = &mut heap.data;

        let mut wr_ix = 0;
        while wr_ix < len {
            data[wr_ix] = slice[wr_ix].clone();
            wr_ix += 1;
        }

        heap.len = len;

        heap.sort();
        heap
    }
}

impl<T> From<Vec<T>> for FixMinBinHeap<T>
where
    T: PartialOrd + Clone + Default,
{
    /// Note that heap satiation cannot be guaranteed.
    /// Heap will support `Vec<T>` capacity nodes.
    fn from(vec: Vec<T>) -> FixMinBinHeap<T> {
        let cap = vec.capacity();
        let len = vec.len();

        let mut vec = vec;
        if len != cap {
            for each in vec.spare_capacity_mut() {
                each.write(T::default());
            }

            unsafe { vec.set_len(cap) }
        }

        let mut heap = FixMinBinHeap {
            data: vec.into_boxed_slice(),
            len,
        };

        heap.sort();
        heap
    }
}

impl<T> FixMinBinHeap<T>
where
    T: PartialOrd + Clone + Default,
{
    // TC: Ο(n)
    fn sort(&mut self) {
        let len = self.len;

        if len == 0 {
            return;
        }

        let mut ix = (self.len / 2) - 1;
        loop {
            self.buble_down(ix);

            if ix == 0 {
                break;
            }

            ix -= 1;
        }
    }
}

/// Uses `core::Clone`. Wrap large types into `std::rc::Rc` or `std::sync::Arc`.
impl<T> FixMinBinHeap<T>
where
    T: PartialOrd + Clone + Default,
{
    pub fn new(levels: usize) -> Self {
        assert!(
            levels <= MAX_LEVELS,
            "Maximum supported levels is 25. 0 for root only."
        );

        let nodes = 2usize.pow((levels + 1) as u32) - 1;
        let data = vec![T::default(); nodes].into_boxed_slice();

        Self { data, len: 0 }
    }

    pub fn insert(&mut self, t: T) -> Result<(), ()> {
        let wrix = self.len;
        let data = &mut self.data;

        let cap = data.len();

        if wrix == cap {
            return Err(());
        }

        data[wrix] = t;
        self.len = wrix + 1;

        self.bubble_up(wrix);

        Ok(())
    }

    // `desix` = descendat index
    fn bubble_up(&mut self, mut desix: usize) {
        let data = &mut self.data;

        while desix > 0 {
            // predecessor index
            let predix = (desix - 1) / 2;

            let predecessor = data[predix].clone();
            let descendant = data[desix].clone();

            if descendant < predecessor {
                data[predix] = descendant;
                data[desix] = predecessor;
            } else {
                break;
            }

            desix = predix;
        }
    }

    pub fn peek_min(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        } else {
            Some(&self.data[0])
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        let len = self.len;

        if len == 0 {
            return None;
        }

        let new_len = len - 1;

        let data = &mut self.data;
        let min = data[0].clone();
        data[0] = data[new_len].clone();

        self.len = new_len;
        self.buble_down(0);

        Some(min)
    }

    // `pred_ix` = predecessor index
    fn buble_down(&mut self, mut pred_ix: usize) {
        let len = self.len;
        let data = &mut self.data;

        loop {
            // descendat index
            let mut des_ix = 2 * pred_ix + 1;

            if des_ix >= len {
                break;
            }

            let des2_ix = des_ix + 1;
            if des2_ix < len && data[des2_ix] < data[des_ix] {
                des_ix = des2_ix;
            }

            let predecessor = data[pred_ix].clone();
            if predecessor < data[des_ix] {
                break;
            }

            data[pred_ix] = data[des_ix].clone();
            data[des_ix] = predecessor;

            pred_ix = des_ix;
        }
    }
}

#[cfg(test)]
mod tests_of_units {

    use super::FixMinBinHeap;
    use std::ops::Deref;

    mod new {
        use super::super::FixMinBinHeap;

        #[test]
        fn leaf_count() {
            let heap = FixMinBinHeap::<u64>::new(2);

            assert_eq!(0, heap.len);
            assert_eq!(7, heap.data.len());
        }

        #[test]
        #[should_panic(expected = "Maximum supported levels is 25. 0 for root only.")]
        fn unsupported_level_count() {
            _ = FixMinBinHeap::<u64>::new(26);
        }
    }

    mod insertion {
        use super::super::FixMinBinHeap;
        use std::ops::Deref;

        #[test]
        fn filling() {
            let mut heap = FixMinBinHeap::<i16>::new(1);

            let nums: [i16; 2] = [9, 8];

            let mut count = 0;
            for n in nums {
                assert_eq!(Ok(()), heap.insert(n));
                count += 1;
                assert_eq!(count, heap.len);
            }

            let case: &[i16] = &[8, 9, 0];
            assert_eq!(case, heap.data.deref());
        }

        #[test]
        fn full_error() {
            let mut heap = FixMinBinHeap::<usize>::new(0);

            assert_eq!(Ok(()), heap.insert(0));
            assert_eq!(Err(()), heap.insert(0));
        }
    }

    #[test]
    fn bubble_up() {
        let mut heap = FixMinBinHeap::<i16>::new(3);
        let data = heap.data.as_mut_ptr();

        let nums: [i16; 10] = [9, 8, 7, 7, 6, 5, 4, 2, 2, 1];

        let mut wri_ix = 0;
        for n in nums {
            unsafe {
                data.offset(wri_ix).write(n);
            }

            heap.bubble_up(wri_ix as usize);
            wri_ix += 1;
        }

        let case: [i16; 15] = [1, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
        assert_eq!(case, heap.data.deref());
    }

    mod peek_min {
        use super::super::FixMinBinHeap;

        #[test]
        fn none_min() {
            let heap = FixMinBinHeap::<usize>::new(0);
            assert_eq!(None, heap.peek_min());
        }

        #[test]
        fn some_min() {
            let heap: FixMinBinHeap<usize> = FixMinBinHeap {
                data: Box::new([5]),
                len: 1,
            };

            assert_eq!(Some(&heap.data[0]), heap.peek_min());
        }
    }

    mod extraction {
        use super::super::FixMinBinHeap;

        #[test]
        fn exctracting() {
            let mut heap = FixMinBinHeap::<usize>::new(1);
            let data = &mut heap.data;

            let test_data = [8, 9, 10];
            data[0] = test_data[0];
            data[1] = test_data[2];
            data[2] = test_data[1];

            let mut heap_len = 3;
            heap.len = heap_len;

            for td in test_data {
                assert_eq!(Some(td), heap.extract_min());
                heap_len -= 1;
                assert_eq!(heap_len, heap.len);
            }
        }

        #[test]
        fn empty_none() {
            let mut heap = FixMinBinHeap::<usize>::new(0);
            assert_eq!(None, heap.extract_min());
        }
    }

    mod bubble_down {

        use super::FixMinBinHeap;
        use std::ops::Deref;

        #[test]
        fn bubble_down() {
            let heap_data: [i16; 15] = [7, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            let mut heap: FixMinBinHeap<i16> = FixMinBinHeap {
                data: Box::new(heap_data),
                len: 9,
            };

            heap.buble_down(0);

            let heap_data = &heap.data;

            let test_data: [i16; 15] = [2, 2, 5, 4, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            assert_eq!(test_data, heap_data.deref());

            segment_test(
                &mut heap,
                8,
                3,
                &[5, 7, 6, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],
            );
            segment_test(
                &mut heap,
                5,
                4,
                &[8, 9, 8, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],
            );
        }

        fn segment_test<T>(
            heap: &mut FixMinBinHeap<T>,
            offset: isize,
            bubble_count: isize,
            test_data: &[T; 15],
        ) where
            T: PartialOrd + Clone + Default + std::fmt::Debug,
        {
            let heap_data_ptr: *mut T = heap.data.as_mut_ptr();

            for i in 0..bubble_count {
                unsafe {
                    heap_data_ptr.write(heap_data_ptr.offset(offset - i).read());
                }

                heap.len = heap.len - 1;
                heap.buble_down(0);
            }

            let heap_data = &heap.data;

            assert_eq!(test_data, heap_data.deref());
        }
    }

    mod from_vec {
        use super::super::FixMinBinHeap;
        use std::ops::Deref;

        #[test]
        fn from_vec_len() {
            let vec = vec![9, 8, 7, 6];
            let len = vec.len();
            let ptr = vec.as_ptr();

            let heap = FixMinBinHeap::from(vec);
            assert_eq!(len, heap.len);
            assert_eq!([6, 8, 7, 9], heap.data.deref());
            assert_eq!(ptr, heap.data.deref().as_ptr());
        }

        #[test]
        fn from_vec_cap() {
            let nums = [9, 8, 7, 6];
            let mut vec = Vec::with_capacity(5);

            for n in nums {
                vec.push(n);
            }

            let len = vec.len();
            let ptr = vec.as_ptr();

            let heap = FixMinBinHeap::from(vec);
            assert_eq!(len, heap.len);
            assert_eq!([6, 8, 7, 9, 0], heap.data.deref());
            assert_eq!(ptr, heap.data.deref().as_ptr());
        }
    }

    mod from_slice_ref {
        use super::super::FixMinBinHeap;
        use super::super::MAX_LEVELS;
        use std::ops::Deref;

        #[test]
        fn basic_test() {
            let nums = [9, 8, 7, 6];
            let len = nums.len();

            let heap = FixMinBinHeap::from(&nums as &[i32]);
            assert_eq!(len, heap.len);
            assert_eq!([6, 8, 7, 9, 0, 0, 0], heap.data.deref());
        }

        #[derive(PartialEq, Eq, PartialOrd, Clone, Default)]
        struct ZeroSize();

        #[test]
        #[should_panic(expected = "Input length is greater than maximal heap item count support.")]
        fn limit_test() {
            let len = 2usize.pow(MAX_LEVELS as u32) + 1;
            let mut vec = Vec::<ZeroSize>::with_capacity(len);

            unsafe {
                vec.set_len(len);
            }

            _ = FixMinBinHeap::from(&vec as &[ZeroSize]);
        }
    }

    mod sort {
        use super::super::FixMinBinHeap;

        #[test]
        fn zero_len_test() {
            let mut heap = FixMinBinHeap {
                data: Box::new([4, 3, 2, 1]),
                len: 0,
            };

            heap.sort();

            assert_eq!([4, 3, 2, 1], *heap.data);
        }

        #[test]
        fn sorting() {
            let mut heap = FixMinBinHeap {
                data: Box::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
                len: 10,
            };

            heap.sort();

            assert_eq!([0, 1, 3, 2, 5, 4, 7, 9, 6, 8], *heap.data);
        }
    }
}
