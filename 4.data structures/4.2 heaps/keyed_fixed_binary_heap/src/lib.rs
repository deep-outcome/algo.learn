#![allow(dead_code)]

pub const MAX_LEVELS: usize = 25;

#[derive(PartialEq, Debug)]
pub enum FixBinHeapForm {
    Maximal,
    Minimal,
}

pub struct FixBinHeap<T>
where
    T: Clone + Default + FixBinHeapKey,
{
    data: Box<[T]>,
    len: usize,
    form: FixBinHeapForm,
}

pub trait FixBinHeapKey {
    type Key: PartialOrd;
    fn key(&self) -> Self::Key;
}

impl FixBinHeapKey for u16 {
    type Key = u16;
    fn key(&self) -> u16 {
        let mut num = *self as i32;
        num = (num - u16::MAX as i32) * -1;

        num as u16
    }
}

impl FixBinHeapKey for usize {
    type Key = usize;
    fn key(&self) -> usize {
        let modul = self % 10;
        let map = self - modul + 9 - modul;
        map

        // so this maps
        // +-----------+---+-------------+---+-----------+
        // |  0 => 9   | … |   5 => 4    | … |   9 => 0  |
        // +-----------+---+-------------+---+-----------+
        // |    ⋮      | ⋮ |     ⋮       | ⋮ |     ⋮     |
        // +-----------+---+-------------+---+-----------+
        // | 99 => 90  | … |  95 => 94   | … |  99 => 90 |
        // +-----------+---+-------------+---+-----------+
        // |    ⋮      | ⋮ |     ⋮       | ⋮ |     ⋮     |
        // +-----------+---+-------------+---+-----------+
    }
}

impl<T> FixBinHeap<T>
where
    T: FixBinHeapKey + Clone + Default,
{
    /// Final capacity is aligned to maximal leaf capacity.
    fn from_slice(slice: &[T], form: FixBinHeapForm) -> FixBinHeap<T> {
        let len = slice.len();

        let limit = (len as f64).log2();
        let levels = limit.ceil() as usize;

        assert!(
            levels <= MAX_LEVELS,
            "Input length is greater than maximal heap item count support."
        );

        let mut heap = FixBinHeap::<T>::new(levels, form);

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

    /// Note that heap satiation cannot be guaranteed.
    /// Heap will support `Vec<T>` capacity nodes.
    fn from_vec(vec: Vec<T>, form: FixBinHeapForm) -> FixBinHeap<T> {
        let cap = vec.capacity();
        let len = vec.len();

        let mut vec = vec;
        if len != cap {
            for each in vec.spare_capacity_mut() {
                each.write(T::default());
            }

            unsafe { vec.set_len(cap) }
        }

        let mut heap = FixBinHeap {
            data: vec.into_boxed_slice(),
            len,
            form,
        };

        heap.sort();
        heap
    }

    // TC: Ο(n)
    fn sort(&mut self) {
        let len = self.len;

        if len == 0 {
            return;
        }

        let mut ix = (self.len / 2) - 1;
        loop {
            self.bubble_down(ix);

            if ix == 0 {
                break;
            }

            ix -= 1;
        }
    }
}

/// Uses `core::Clone`. Wrap large types into `std::rc::Rc` or `std::sync::Arc`.
impl<T> FixBinHeap<T>
where
    T: FixBinHeapKey + Clone + Default,
{
    pub fn new(levels: usize, form: FixBinHeapForm) -> Self {
        assert!(
            levels <= MAX_LEVELS,
            "Maximum supported levels is 25. 0 for root only."
        );

        let nodes = 2usize.pow((levels + 1) as u32) - 1;
        let data = vec![T::default(); nodes].into_boxed_slice();

        Self { data, len: 0, form }
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

    fn cmp(
        &self,
    ) -> for<'a, 'b> fn(&'a <T as FixBinHeapKey>::Key, &'b <T as FixBinHeapKey>::Key) -> bool {
        if self.form == FixBinHeapForm::Minimal {
            return PartialOrd::<<T as FixBinHeapKey>::Key>::lt;
        }

        if self.form == FixBinHeapForm::Maximal {
            return PartialOrd::<<T as FixBinHeapKey>::Key>::gt;
        }

        panic!("Unsupported heap form.");
    }

    // `desix` = descendat index
    fn bubble_up(&mut self, mut desix: usize) {
        let cmp = self.cmp();

        let data = &mut self.data;

        while desix > 0 {
            // predecessor index
            let predix = (desix - 1) / 2;

            let predecessor = data[predix].clone();
            let descendant = data[desix].clone();

            //if descendant < predecessor {
            if cmp(&descendant.key(), &predecessor.key()) {
                data[predix] = descendant;
                data[desix] = predecessor;
            } else {
                break;
            }

            desix = predix;
        }
    }

    pub fn peek_root(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        } else {
            Some(&self.data[0])
        }
    }

    pub fn extract_root(&mut self) -> Option<T> {
        let len = self.len;

        if len == 0 {
            return None;
        }

        let new_len = len - 1;

        let data = &mut self.data;
        let root = data[0].clone();
        data[0] = data[new_len].clone();

        self.len = new_len;
        self.bubble_down(0);

        Some(root)
    }

    // `pred_ix` = predecessor index
    fn bubble_down(&mut self, mut pred_ix: usize) {
        let len = self.len;
        let cmp = self.cmp();

        let data = &mut self.data;

        loop {
            // descendat index
            let mut des_ix = 2 * pred_ix + 1;

            if des_ix >= len {
                break;
            }

            let des2_ix = des_ix + 1;
            if des2_ix < len && cmp(&data[des2_ix].key(), &data[des_ix].key()) {
                des_ix = des2_ix;
            }

            let predecessor = data[pred_ix].clone();
            if cmp(&predecessor.key(), &data[des_ix].key()) {
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

    use super::{FixBinHeap, FixBinHeapForm};
    use std::ops::Deref;

    mod new {
        use crate::FixBinHeapForm;

        use super::super::FixBinHeap;

        #[test]
        fn leaf_count() {
            let heap = FixBinHeap::<u16>::new(2, FixBinHeapForm::Minimal);

            assert_eq!(0, heap.len);
            assert_eq!(7, heap.data.len());
            assert_eq!(FixBinHeapForm::Minimal, heap.form);
        }

        #[test]
        #[should_panic(expected = "Maximum supported levels is 25. 0 for root only.")]
        fn unsupported_level_count() {
            _ = FixBinHeap::<u16>::new(26, FixBinHeapForm::Minimal);
        }
    }

    mod insertion {
        use super::super::{FixBinHeap, FixBinHeapForm};
        use std::ops::Deref;

        #[test]
        fn filling() {
            let test_cases = [
                (FixBinHeapForm::Minimal, [8, 9], [9, 8, 0]),
                (FixBinHeapForm::Maximal, [9, 8], [8, 9, 0]),
            ];

            for case in test_cases {
                let mut heap = FixBinHeap::<u16>::new(1, case.0);

                let nums: [u16; 2] = case.1;

                let mut count = 0;
                for n in nums {
                    assert_eq!(Ok(()), heap.insert(n));
                    count += 1;
                    assert_eq!(count, heap.len);
                }

                assert_eq!(case.2, heap.data.deref());
            }
        }

        #[test]
        fn full_error() {
            let mut heap = FixBinHeap::<u16>::new(0, FixBinHeapForm::Minimal);

            assert_eq!(Ok(()), heap.insert(0));
            assert_eq!(Err(()), heap.insert(0));
        }
    }

    #[test]
    fn bubble_up() {
        #[rustfmt::skip]
        let test_cases = [
            (FixBinHeapForm::Minimal, [1, 2, 2, 4, 5, 6, 7, 7, 8, 9], [9, 8, 6, 7, 7, 2, 5, 1, 4, 2, 0, 0, 0, 0, 0]),
            (FixBinHeapForm::Maximal, [9, 8, 7, 7, 6, 5, 4, 2, 2, 1], [1, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0]),
        ];

        for case in test_cases {
            let mut heap = FixBinHeap::<u16>::new(3, case.0);
            let data = heap.data.as_mut_ptr();

            let nums: [u16; 10] = case.1;

            let mut wri_ix = 0;
            for n in nums {
                unsafe {
                    data.offset(wri_ix).write(n);
                }

                heap.bubble_up(wri_ix as usize);
                wri_ix += 1;
            }

            assert_eq!(case.2, heap.data.deref());
        }
    }

    mod peek_root {
        use super::super::{FixBinHeap, FixBinHeapForm};

        #[test]
        fn none_root() {
            let heap = FixBinHeap::<u16>::new(0, FixBinHeapForm::Minimal);
            assert_eq!(None, heap.peek_root());
        }

        #[test]
        fn some_root() {
            let heap: FixBinHeap<u16> = FixBinHeap {
                data: Box::new([5]),
                len: 1,
                form: FixBinHeapForm::Minimal,
            };

            assert_eq!(Some(&heap.data[0]), heap.peek_root());
        }
    }

    mod extraction {
        use super::super::{FixBinHeap, FixBinHeapForm};

        #[test]
        fn exctracting() {
            let test_cases = [
                (FixBinHeapForm::Maximal, [8, 10, 9], [8, 9, 10]),
                (FixBinHeapForm::Minimal, [10, 8, 9], [10, 9, 8]),
            ];

            for case in test_cases {
                let mut heap = FixBinHeap::<u16>::new(1, case.0);
                let data = &mut heap.data;

                let test_data = case.1;
                let mut ix = 0;
                while ix < 3 {
                    data[ix] = test_data[ix];
                    ix += 1;
                }

                let mut heap_len = 3;
                heap.len = heap_len;

                for td in case.2 {
                    assert_eq!(Some(td), heap.extract_root());
                    heap_len -= 1;
                    assert_eq!(heap_len, heap.len);
                }
            }
        }

        #[test]
        fn empty_none() {
            let mut heap = FixBinHeap::<u16>::new(0, FixBinHeapForm::Minimal);
            assert_eq!(None, heap.extract_root());
        }
    }

    mod bubble_down {

        use super::super::{FixBinHeap, FixBinHeapForm, FixBinHeapKey};
        use std::ops::Deref;

        #[test]
        fn minimal() {
            let heap_data: [u16; 15] = [7, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            let mut heap: FixBinHeap<u16> = FixBinHeap {
                data: Box::new(heap_data),
                len: 9,
                form: FixBinHeapForm::Maximal,
            };

            {
                heap.bubble_down(0);

                let heap_data = &heap.data;

                let test_data: [u16; 15] = [2, 2, 5, 4, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
                assert_eq!(test_data, heap_data.deref());
            }

            #[rustfmt::skip]
            segment_test(&mut heap,8,3,&[5, 7, 6, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],);
            #[rustfmt::skip]
            segment_test(&mut heap,5,4,&[8, 9, 8, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],);
        }

        #[test]
        fn maximal() {
            let heap_data: [u16; 15] = [5, 9, 10, 7, 7, 8, 6, 4, 3, 2, 1, 4, 3, 5, 0];
            let mut heap: FixBinHeap<u16> = FixBinHeap {
                data: Box::new(heap_data),
                len: 13,
                form: FixBinHeapForm::Minimal,
            };

            heap.bubble_down(0);
            let heap_data = &heap.data;

            let test_data: [u16; 15] = [10, 9, 8, 7, 7, 5, 6, 4, 3, 2, 1, 4, 3, 5, 0];
            assert_eq!(test_data, heap_data.deref());

            #[rustfmt::skip]
            segment_test(&mut heap,12,3,&[7, 7, 6, 4, 2, 5, 4, 3, 3, 1, 1, 4, 3, 5, 0],);
            #[rustfmt::skip]
            segment_test(&mut heap,9,3,&[5, 4, 4, 3, 2, 3, 1, 1, 3, 1, 1, 4, 3, 5, 0],);
            #[rustfmt::skip]
            segment_test(&mut heap,6,3,&[3, 2, 3, 1, 2, 3, 1, 1, 3, 1, 1, 4, 3, 5, 0],);
            #[rustfmt::skip]
            segment_test(&mut heap,3,2,&[2, 1, 1, 1, 2, 3, 1, 1, 3, 1, 1, 4, 3, 5, 0],);
        }

        fn segment_test<T>(
            heap: &mut FixBinHeap<T>,
            offset: isize,
            bubble_count: isize,
            test_data: &[T; 15],
        ) where
            T: FixBinHeapKey + Clone + Default + std::fmt::Debug + PartialEq,
            <T as FixBinHeapKey>::Key: PartialOrd,
        {
            let heap_data_ptr: *mut T = heap.data.as_mut_ptr();

            for i in 0..bubble_count {
                unsafe {
                    heap_data_ptr.write(heap_data_ptr.offset(offset - i).read());
                }

                heap.len = heap.len - 1;
                heap.bubble_down(0);
            }

            let heap_data = &heap.data;

            assert_eq!(test_data, heap_data.deref());
        }
    }

    mod from_vec {
        use super::super::{FixBinHeap, FixBinHeapForm};
        use std::ops::Deref;

        #[test]
        fn from_vec_len() {
            let vec = vec![9, 0, 19, 10usize];
            let len = vec.len();
            let ptr = vec.as_ptr();

            let heap = FixBinHeap::from_vec(vec, FixBinHeapForm::Maximal);
            assert_eq!(len, heap.len);
            assert_eq!([10, 0, 19, 9], heap.data.deref());
            assert_eq!(ptr, heap.data.deref().as_ptr());
        }

        #[test]
        fn from_vec_cap() {
            let nums = [10, 11, 8, 9usize];
            let mut vec = Vec::with_capacity(5);

            for n in nums {
                vec.push(n);
            }

            let len = vec.len();
            let ptr = vec.as_ptr();

            let heap = FixBinHeap::from_vec(vec, FixBinHeapForm::Minimal);
            assert_eq!(len, heap.len);
            assert_eq!([9, 11, 8, 10, 0], heap.data.deref());
            assert_eq!(ptr, heap.data.deref().as_ptr());
        }
    }

    mod from_slice_ref {
        use super::super::MAX_LEVELS;
        use super::super::{FixBinHeap, FixBinHeapForm, FixBinHeapKey};
        use std::ops::Deref;

        #[test]
        fn basic_test() {
            let nums: [u16; 4] = [9, 8, 7, 6];
            let len = nums.len();

            let heap = FixBinHeap::from_slice(&nums as &[u16], FixBinHeapForm::Maximal);
            assert_eq!(len, heap.len);
            assert_eq!([6, 8, 7, 9, 0, 0, 0], heap.data.deref());
        }

        #[derive(PartialEq, Eq, PartialOrd, Clone, Default)]
        struct ZeroSize();

        impl FixBinHeapKey for ZeroSize {
            type Key = usize;
            fn key(&self) -> Self::Key {
                panic!();
            }
        }

        #[test]
        #[should_panic(expected = "Input length is greater than maximal heap item count support.")]
        fn limit_test() {
            let len = 2usize.pow(MAX_LEVELS as u32) + 1;
            let mut vec = Vec::<ZeroSize>::with_capacity(len);

            unsafe {
                vec.set_len(len);
            }

            _ = FixBinHeap::from_slice(&vec as &[ZeroSize], FixBinHeapForm::Minimal);
        }
    }

    mod sort {
        use super::super::{FixBinHeap, FixBinHeapForm};

        #[test]
        fn zero_len_test() {
            let mut heap = FixBinHeap {
                data: Box::new([1usize, 10]),
                len: 0,
                form: FixBinHeapForm::Minimal,
            };

            heap.sort();

            assert_eq!([1, 10], *heap.data);
        }

        #[test]
        fn sorting() {
            let mut heap = FixBinHeap {
                data: Box::new([10, 11, 12, 0, 1usize, 2, 5, 6, 7]),
                len: 9,
                form: FixBinHeapForm::Minimal,
            };

            heap.sort();

            assert_eq!([7, 6, 5, 0, 1, 2, 12, 11, 10], *heap.data);
        }
    }

    mod fix_bin_heap_key {

        use super::super::FixBinHeapKey;
        #[test]
        fn u16_key_test() {
            assert_eq!(0, u16::MAX.key());
            assert_eq!(1, u16::MAX - 1u16.key());
            assert_eq!(u16::MAX, 0u16.key());
            assert_eq!(u16::MAX - 1, 1u16.key());
        }

        #[test]
        fn usize_key_test() {
            assert_eq!(0, 9usize.key());
            assert_eq!(5, 4usize.key());
            assert_eq!(9, 0usize.key());

            assert_eq!(100, 109usize.key());
            assert_eq!(105, 104usize.key());
            assert_eq!(109, 100usize.key());

            assert_eq!(1990, 1999usize.key());
            assert_eq!(1995, 1994usize.key());
            assert_eq!(1999, 1990usize.key());
        }
    }
}
