#![allow(dead_code)]

pub struct FixMinBinHeap<T> {
    data: Box<[Option<T>]>,
    len: usize,
}

/// Uses `core::Clone`. Wrap large types into `std::rc::Rc` or `std::sync::Arc`.
impl<T> FixMinBinHeap<T>
where
    T: PartialOrd + Clone,
{
    pub fn new(levels: u8) -> Self {
        assert!(
            levels < 26,
            "Maximum supported levels is 25. 0 for root only."
        );

        let nodes = 2usize.pow((levels + 1) as u32) - 1;
        let data = vec![None; nodes].into_boxed_slice();

        Self { data, len: 0 }
    }

    pub fn insert(&mut self, t: T) -> Result<(), ()> {
        let wrix = self.len;
        let data = &mut self.data;

        let cap = data.len();

        if wrix == cap {
            return Err(());
        }

        data[wrix] = Some(t);
        self.len = wrix + 1;

        self.bubble_up(wrix);

        Ok(())
    }

    /// `desix` = descendat index
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

    pub fn min(&self) -> Option<&T> {
        self.data[0].as_ref()
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

        min
    }

    /// `pred_ix` = predecessor index
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

            let case: &[Option<i16>] = &[Some(8), Some(9), None];
            assert_eq!(case, heap.data.deref());
        }

        #[test]
        fn full_error() {
            let mut heap = FixMinBinHeap::<usize>::new(0);

            assert_eq!(Ok(()), heap.insert(0));
            assert_eq!(Err(()), heap.insert(0));
        }
    }

    fn map_to_option(num: i16) -> Option<i16> {
        if num == 0 {
            return None;
        }
        Some(num)
    }

    #[test]
    fn bubble_up() {
        let mut heap = FixMinBinHeap::<i16>::new(3);
        let data = heap.data.as_mut_ptr();

        let nums: [i16; 10] = [9, 8, 7, 7, 6, 5, 4, 2, 2, 1];

        let mut wri_ix = 0;
        for n in nums {
            unsafe {
                data.offset(wri_ix).write(Some(n));
            }

            heap.bubble_up(wri_ix as usize);
            wri_ix += 1;
        }

        let case: [i16; 15] = [1, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
        let case = case.map(map_to_option);

        assert_eq!(case, heap.data.deref());
    }

    mod min {
        use super::super::FixMinBinHeap;

        #[test]
        fn none_min() {
            let heap = FixMinBinHeap::<usize>::new(0);
            assert_eq!(None, heap.min());
        }

        #[test]
        fn some_min() {
            let heap: FixMinBinHeap<usize> = FixMinBinHeap {
                data: Box::new([Some(5)]),
                len: 1,
            };

            assert_eq!(heap.data[0].as_ref(), heap.min());
        }
    }

    mod extraction {
        use super::super::FixMinBinHeap;

        #[test]
        fn exctracting() {
            let mut heap = FixMinBinHeap::<usize>::new(1);
            let data = &mut heap.data;

            let test_data = [Some(8), Some(9), Some(10)];
            data[0] = test_data[0];
            data[1] = test_data[2];
            data[2] = test_data[1];

            let mut heap_len = 3;
            heap.len = heap_len;

            for td in test_data {
                assert_eq!(td, heap.extract_min());
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

    #[test]
    fn bubble_down() {
        let heap_data: [i16; 15] = [7, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
        let heap_data = heap_data.map(map_to_option);
        let mut heap: FixMinBinHeap<i16> = FixMinBinHeap {
            data: Box::new(heap_data),
            len: 9,
        };

        let heap_data_ptr: *mut Option<i16> = heap.data.as_mut_ptr();

        {
            heap.buble_down(0);

            let heap_data = &heap.data;

            let test_data: [i16; 15] = [2, 2, 5, 4, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            let test_data = test_data.map(map_to_option);
            assert_eq!(test_data, heap_data.deref());
        }

        let offset = 8;
        for i in 0..3 {
            unsafe {
                heap_data_ptr.write(heap_data_ptr.offset(offset - i).read());
            }

            heap.len = heap.len - 1;
            heap.buble_down(0);
        }

        {
            let heap_data = &heap.data;

            let test_data: [i16; 15] = [5, 7, 6, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            let test_data = test_data.map(map_to_option);
            assert_eq!(test_data, heap_data.deref());
        }

        let offset = 5;
        for i in 0..4 {
            unsafe {
                heap_data_ptr.write(heap_data_ptr.offset(offset - i).read());
            }

            heap.len = heap.len - 1;
            heap.buble_down(0);
        }

        {
            let heap_data = &heap.data;

            let test_data: [i16; 15] = [8, 9, 8, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            let test_data = test_data.map(map_to_option);
            assert_eq!(test_data, heap_data.deref());
        }
    }
}
