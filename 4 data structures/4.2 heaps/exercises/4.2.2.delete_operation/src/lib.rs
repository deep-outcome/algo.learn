pub const MAX_LEVELS: usize = 25;

#[derive(PartialEq, Debug, Clone)]
pub enum FixBinHeapForm {
    Maximal,
    Minimal,
    Unspecified,
}

pub struct FixBinHeap<T>
where
    T: Ord + Clone + Default,
{
    data: Box<[T]>,
    len: usize,
    form: FixBinHeapForm,
}

impl<T> FixBinHeap<T>
where
    T: Ord + Clone + Default,
{
    /// Final capacity is aligned to maximal leaf capacity.
    pub fn from_slice(slice: &[T], form: FixBinHeapForm) -> FixBinHeap<T> {
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
    pub fn from_vec(vec: Vec<T>, form: FixBinHeapForm) -> FixBinHeap<T> {
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
    T: Ord + Clone + Default,
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

    fn cmp(&self) -> for<'a, 'b> fn(&'a T, &'b T) -> bool {
        if self.form == FixBinHeapForm::Minimal {
            return PartialOrd::<T>::lt;
        }

        if self.form == FixBinHeapForm::Maximal {
            return PartialOrd::<T>::gt;
        }

        panic!("Unsupported heap form.");
    }

    // `desix` = descendant index
    fn bubble_up(&mut self, mut des_ix: usize) {
        let cmp = self.cmp();

        let data = &mut self.data;

        while des_ix > 0 {
            // predecessor index
            let pred_ix = Self::predix(des_ix);

            let predecessor = data[pred_ix].clone();
            let descendant = data[des_ix].clone();

            if cmp(&descendant, &predecessor) {
                data[pred_ix] = descendant;
                data[des_ix] = predecessor;
            } else {
                break;
            }

            des_ix = pred_ix;
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

        let root = self.data[0].clone();
        _ = self.del(0);

        Some(root)
    }

    // `pred_ix` = predecessor index
    fn bubble_down(&mut self, mut pred_ix: usize) {
        let len = self.len;
        let cmp = self.cmp();

        let data = &mut self.data;

        loop {
            // descendant index
            let mut des_ix = Self::l_desix(pred_ix);

            if des_ix >= len {
                break;
            }

            let des2_ix = des_ix + 1;
            if des2_ix < len && cmp(&data[des2_ix], &data[des_ix]) {
                des_ix = des2_ix;
            }

            let predecessor = data[pred_ix].clone();
            let descendant = data[des_ix].clone();
            if cmp(&predecessor, &descendant) {
                break;
            }

            data[pred_ix] = descendant;
            data[des_ix] = predecessor;

            pred_ix = des_ix;
        }
    }

    #[cfg(feature = "complex-del")]
    pub fn del(&mut self, ix: usize) -> Result<(), ()> {
        use std::cmp::Ordering;

        let len = self.len;

        if ix >= len {
            return Err(());
        }

        let last_ix = self.len - 1;
        self.len = last_ix;

        if last_ix == 0 {
            return Ok(());
        }

        let data = &mut self.data;
        {
            data[ix] = data[last_ix].clone();
        }

        if ix > 0 {
            let pred_ix = Self::predix(ix);

            let ord = data[pred_ix].cmp(&data[ix]);
            if ord == Ordering::Equal {
                return Ok(());
            }

            let form = self.form.clone();
            if (ord == Ordering::Greater && form == FixBinHeapForm::Minimal)
                || (ord == Ordering::Less && form == FixBinHeapForm::Maximal)
            {
                self.bubble_up(ix);
                return Ok(());
            }
        }

        self.bubble_down(ix);
        return Ok(());
    }

    #[cfg(feature = "simplified-del")]
    pub fn del(&mut self, ix: usize) -> Result<(), ()> {
        let len = self.len;

        if ix >= len {
            return Err(());
        }

        let data = &mut self.data;
        {
            let last_ix = self.len - 1;
            self.len = last_ix;
            data[ix] = data[last_ix].clone();
        }
        self.bubble_up(ix);
        self.bubble_down(ix);
        return Ok(());
    }

    /// left descendant index computation
    fn l_desix(pred_ix: usize) -> usize {
        2 * pred_ix + 1
    }

    /// predecesor index computation
    fn predix(des_ix: usize) -> usize {
        (des_ix - 1) / 2
    }
}

// cargo test --features simplified-del
// cargo test --features complex-del

#[cfg(test)]
mod tests_of_units {

    use super::{FixBinHeap, FixBinHeapForm};
    use std::ops::Deref;

    mod new {
        use crate::FixBinHeapForm;

        use super::super::FixBinHeap;

        #[test]
        fn leaf_count() {
            let heap = FixBinHeap::<u64>::new(2, FixBinHeapForm::Minimal);

            assert_eq!(0, heap.len);
            assert_eq!(7, heap.data.len());
            assert_eq!(FixBinHeapForm::Minimal, heap.form);
        }

        #[test]
        #[should_panic(expected = "Maximum supported levels is 25. 0 for root only.")]
        fn unsupported_level_count() {
            _ = FixBinHeap::<u64>::new(26, FixBinHeapForm::Minimal);
        }
    }

    mod insertion {
        use super::super::{FixBinHeap, FixBinHeapForm};
        use std::ops::Deref;

        #[test]
        fn filling() {
            let test_cases = [
                (FixBinHeapForm::Maximal, [8, 9], [9, 8, 0]),
                (FixBinHeapForm::Minimal, [9, 8], [8, 9, 0]),
            ];

            for case in &test_cases {
                let mut heap = FixBinHeap::<i16>::new(1, case.0.clone());

                let nums: [i16; 2] = case.1;

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
            let mut heap = FixBinHeap::<usize>::new(0, FixBinHeapForm::Minimal);

            assert_eq!(Ok(()), heap.insert(0));
            assert_eq!(Err(()), heap.insert(0));
        }
    }

    #[test]
    fn bubble_up() {
        #[rustfmt::skip]
        let test_cases = [
            (FixBinHeapForm::Maximal, [1, 2, 2, 4, 5, 6, 7, 7, 8, 9], [9, 8, 6, 7, 7, 2, 5, 1, 4, 2, 0, 0, 0, 0, 0]),
            (FixBinHeapForm::Minimal, [9, 8, 7, 7, 6, 5, 4, 2, 2, 1], [1, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0]),
        ];

        for case in &test_cases {
            let mut heap = FixBinHeap::<i16>::new(3, case.0.clone());
            let data = heap.data.as_mut_ptr();

            let nums: [i16; 10] = case.1;

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
            let heap = FixBinHeap::<usize>::new(0, FixBinHeapForm::Minimal);
            assert_eq!(None, heap.peek_root());
        }

        #[test]
        fn some_root() {
            let heap: FixBinHeap<usize> = FixBinHeap {
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
                (FixBinHeapForm::Minimal, [8, 10, 9, 10], [8, 9, 10, 10]),
                (FixBinHeapForm::Maximal, [10, 8, 9, 8], [10, 9, 8, 8]),
            ];

            for case in &test_cases {
                let mut heap = FixBinHeap::<usize>::new(2, case.0.clone());
                let data = &mut heap.data;

                let mut heap_len = 4;

                let test_data = case.1;
                let mut ix = 0;
                while ix < heap_len {
                    data[ix] = test_data[ix];
                    ix += 1;
                }

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
            let mut heap = FixBinHeap::<usize>::new(0, FixBinHeapForm::Minimal);
            assert_eq!(None, heap.extract_root());
        }
    }

    mod bubble_down {

        use super::super::{FixBinHeap, FixBinHeapForm};
        use std::ops::Deref;

        #[test]
        fn minimal() {
            let heap_data: [i16; 15] = [7, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            let mut heap: FixBinHeap<i16> = FixBinHeap {
                data: Box::new(heap_data),
                len: 9,
                form: FixBinHeapForm::Minimal,
            };

            {
                heap.bubble_down(0);

                let heap_data = &heap.data;

                let test_data: [i16; 15] = [2, 2, 5, 4, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
                assert_eq!(test_data, heap_data.deref());
            }

            #[rustfmt::skip]
            segment_test(&mut heap,8,3,&[5, 7, 6, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],);
            #[rustfmt::skip]
            segment_test(&mut heap,5,4,&[8, 9, 8, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],);
        }

        #[test]
        fn maximal() {
            let heap_data: [i16; 15] = [5, 9, 10, 7, 7, 8, 6, 4, 3, 2, 1, 4, 3, 5, 0];
            let mut heap: FixBinHeap<i16> = FixBinHeap {
                data: Box::new(heap_data),
                len: 13,
                form: FixBinHeapForm::Maximal,
            };

            heap.bubble_down(0);
            let heap_data = &heap.data;

            let test_data: [i16; 15] = [10, 9, 8, 7, 7, 5, 6, 4, 3, 2, 1, 4, 3, 5, 0];
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
            T: Ord + Clone + Default + std::fmt::Debug,
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
            let vec = vec![9, 8, 7, 6];
            let len = vec.len();
            let ptr = vec.as_ptr();

            let heap = FixBinHeap::from_vec(vec, FixBinHeapForm::Minimal);
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

            let heap = FixBinHeap::from_vec(vec, FixBinHeapForm::Minimal);
            assert_eq!(len, heap.len);
            assert_eq!([6, 8, 7, 9, 0], heap.data.deref());
            assert_eq!(ptr, heap.data.deref().as_ptr());
        }
    }

    mod from_slice_ref {
        use super::super::MAX_LEVELS;
        use super::super::{FixBinHeap, FixBinHeapForm};
        use std::ops::Deref;

        #[test]
        fn basic_test() {
            let nums = [9, 8, 7, 6];
            let len = nums.len();

            let heap = FixBinHeap::from_slice(&nums as &[i32], FixBinHeapForm::Minimal);
            assert_eq!(len, heap.len);
            assert_eq!([6, 8, 7, 9, 0, 0, 0], heap.data.deref());
        }

        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
        struct ZeroSize();

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
                data: Box::new([4, 3, 2, 1]),
                len: 0,
                form: FixBinHeapForm::Minimal,
            };

            heap.sort();

            assert_eq!([4, 3, 2, 1], *heap.data);
        }

        #[test]
        fn sorting() {
            let mut heap = FixBinHeap {
                data: Box::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
                len: 10,
                form: FixBinHeapForm::Minimal,
            };

            heap.sort();

            assert_eq!([0, 1, 3, 2, 5, 4, 7, 9, 6, 8], *heap.data);
        }
    }

    mod del {

        use super::{FixBinHeap, FixBinHeapForm};
        use std::ops::Deref;

        #[test]
        // equal-to-predecesor relation
        fn no_bubbling() {
            #[rustfmt::skip]
            let test_cases = [
                (
                    FixBinHeapForm::Minimal,
                 [1,  10, 1,  20, 30,  1, 1,  40, 50,  60, 70,  1, 0,  0, 0],
                 [1,   1, 1,  20, 30,  1, 1,  40, 50,  60, 70,  1, 0,  0, 0],
                ),
                (
                    FixBinHeapForm::Maximal,
                 [70,  60, 1,  50, 40,  1, 1,  30, 20,  10, 0,  70, 0,  0, 0],
                 [70,  70, 1,  50, 40,  1, 1,  30, 20,  10, 0,  70, 0,  0, 0],
                ),
            ];

            for case in &test_cases {
                let mut heap = FixBinHeap {
                    data: Box::new(case.1),
                    len: 12,
                    form: case.0.clone(),
                };

                assert_eq!(Ok(()), heap.del(1));
                assert_eq!(case.2, heap.data.deref())
            }
        }

        #[test]
        // invalid-to-predecesor relation
        fn bubble_up() {
            #[rustfmt::skip]
            let test_cases = [
                (
                    FixBinHeapForm::Minimal,
                 [1,  10, 1,  20, 30,  1, 1,  40, 50,  60, 70,  1, 0,  0, 0],
                 [1,   1, 1,  10, 30,  1, 1,  40, 20,  60, 70,  1, 0,  0, 0],
                ),
                (
                    FixBinHeapForm::Maximal,
                 [70,  60, 1,  50, 40,  1, 1,  30, 20,  10, 0,  70, 0,  0, 0],
                 [70,  70, 1,  60, 40,  1, 1,  30, 50,  10, 0,  70, 0,  0, 0],
                ),
            ];

            for case in &test_cases {
                let mut heap = FixBinHeap {
                    data: Box::new(case.1),
                    len: 12,
                    form: case.0.clone(),
                };

                assert_eq!(Ok(()), heap.del(8));
                assert_eq!(case.2, heap.data.deref())
            }
        }

        #[test]
        // purely-valid-to-predecesor relation
        // descendants can be all: =, <, >
        fn bubble_down() {
            #[rustfmt::skip]
            let test_cases = [
                (
                    FixBinHeapForm::Minimal,
                 [1,  10, 1,  20, 30,  1, 1,  40, 50,  60, 70,  70, 0,  0, 0],
                 [1,  20, 1,  40, 30,  1, 1,  70, 50,  60, 70,  70, 0,  0, 0],
                ),
                (
                    FixBinHeapForm::Maximal,
                 [70,  60, 1,  50, 40,  1, 1,  30, 20,  10, 0,  1, 0,  0, 0],
                 [70,  50, 1,  30, 40,  1, 1,   1, 20,  10, 0,  1, 0,  0, 0],
                ),
            ];

            for case in &test_cases {
                let mut heap = FixBinHeap {
                    data: Box::new(case.1),
                    len: 12,
                    form: case.0.clone(),
                };

                assert_eq!(Ok(()), heap.del(1));
                assert_eq!(case.2, heap.data.deref())
            }
        }

        #[test]
        fn general_properties() {
            let test_cases = [(0, Ok(())), (1, Err(())), (2, Err(()))];

            for c in &test_cases {
                let mut heap = FixBinHeap {
                    data: Box::new([0; 1]),
                    len: 1,
                    form: FixBinHeapForm::Maximal,
                };

                assert_eq!(c.1, heap.del(c.0));

                let exp_len = if c.0 > 0 { 1 } else { 0 };
                assert_eq!(exp_len, heap.len);
            }
        }
    }
}
