#![allow(dead_code)]

pub struct FixMinBinHeap<T> {
    data: Box<[Option<T>]>,
    len: usize,
}

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

        self.bubble_up();

        self.len = wrix + 1;
        Ok(())
    }

    fn bubble_up(&mut self) {
        let data = &mut self.data;
        let mut desix = self.len;

        while desix > 0 {
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

            heap.bubble_up();

            wri_ix += 1;
            heap.len = wri_ix as usize;
        }
        
        let case: [i16; 15] = [1, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
        let case = case
            .iter()
            .map(|x| {
                let x = *x;
                if x != 0 {
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<Vec<Option<i16>>>();

        assert_eq!(case, heap.data.deref());
    }
}
