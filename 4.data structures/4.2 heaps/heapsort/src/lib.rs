// TC: Ο(n log n)
pub fn heapsort<T>(data: &mut [T], inv_ord: bool)
where
    T: PartialOrd + Clone,
{
    let sort_form = if inv_ord {
        SortForm::Minimal
    } else {
        SortForm::Maximal
    };
    let cmp = cmp::<T>(sort_form);

    let data_len = data.len();
    let mut ix = (data_len / 2) - 1;

    // TC: Ο(n log n)
    loop {
        bubble_down(data, data_len, ix, cmp);

        if ix == 0 {
            break;
        }

        ix -= 1;
    }

    ix = data_len - 1;
    // TC: Ο(n log n)
    while ix > 0 {
        let root = data[0].clone();
        data[0] = data[ix].clone();
        data[ix] = root;

        bubble_down(data, ix, 0, cmp);

        ix -= 1;
    }
}

fn bubble_down<T>(data: &mut [T], heap_len: usize, mut pred_ix: usize, cmp: fn(&T, &T) -> bool)
where
    T: PartialOrd + Clone,
{
    loop {
        let mut des_ix = 2 * pred_ix + 1;

        if des_ix >= heap_len {
            break;
        }

        let des2_ix = des_ix + 1;
        if des2_ix < heap_len && cmp(&data[des2_ix], &data[des_ix]) {
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

enum SortForm {
    Minimal,
    Maximal,
}

fn cmp<T>(form: SortForm) -> for<'a, 'b> fn(&'a T, &'b T) -> bool
where
    T: PartialOrd,
{
    match form {
        SortForm::Maximal => PartialOrd::<T>::gt,
        SortForm::Minimal => PartialOrd::<T>::lt,
        _ => panic!("Disallowed form discriminant: {}", form as isize),
    }
}

#[cfg(test)]
mod tests_of_units {

    mod heapsort {

        use super::super::heapsort;

        #[test]
        fn minimal() {
            let mut data: [i8; 12] = [100, 30, -5, 12, 25, -3, -33, 0, 0, 3, 16, 22];

            heapsort(&mut data, false);

            let criterion = [-33, -5, -3, 0, 0, 3, 12, 16, 22, 25, 30, 100];

            assert_eq!(criterion, data);
        }

        #[test]
        fn maximal() {
            let mut data: [i8; 12] = [30, -5, 12, 25, -3, -33, 0, 0, 3, 16, 22, 100];

            heapsort(&mut data, true);

            let criterion = [100, 30, 25, 22, 16, 12, 3, 0, 0, -3, -5, -33];

            assert_eq!(criterion, data);
        }
    }

    mod bubble_down {

        use super::super::{bubble_down, cmp, SortForm};

        #[test]
        fn minimal() {
            let mut data: [i16; 15] = [7, 2, 5, 4, 2, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];

            let cmp = cmp::<i16>(SortForm::Minimal);

            bubble_down(&mut data, 9, 0, cmp);

            let test_data: [i16; 15] = [2, 2, 5, 4, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0];
            assert_eq!(test_data, data);

            #[rustfmt::skip]
            segment_test(&mut data,8,3,&[5, 7, 6, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],cmp);
            #[rustfmt::skip]
            segment_test(&mut data,5,4,&[8, 9, 8, 9, 7, 8, 6, 9, 7, 7, 0, 0, 0, 0, 0],cmp);
        }

        #[test]
        fn maximal() {
            let mut data: [i16; 15] = [5, 9, 10, 7, 7, 8, 6, 4, 3, 2, 1, 4, 3, 5, 0];

            let cmp = cmp::<i16>(SortForm::Maximal);

            bubble_down(&mut data, 13, 0, cmp);

            let test_data: [i16; 15] = [10, 9, 8, 7, 7, 5, 6, 4, 3, 2, 1, 4, 3, 5, 0];
            assert_eq!(test_data, data);

            #[rustfmt::skip]
            segment_test(&mut data,12,3,&[7, 7, 6, 4, 2, 5, 4, 3, 3, 1, 1, 4, 3, 5, 0],cmp);
            #[rustfmt::skip]
            segment_test(&mut data,9,3,&[5, 4, 4, 3, 2, 3, 1, 1, 3, 1, 1, 4, 3, 5, 0],cmp);
            #[rustfmt::skip]
            segment_test(&mut data,6,3,&[3, 2, 3, 1, 2, 3, 1, 1, 3, 1, 1, 4, 3, 5, 0],cmp);
            #[rustfmt::skip]
            segment_test(&mut data,3,2,&[2, 1, 1, 1, 2, 3, 1, 1, 3, 1, 1, 4, 3, 5, 0],cmp);
        }

        fn segment_test<T>(
            data: &mut [T],
            offset: isize,
            bubble_count: isize,
            test_data: &[T; 15],
            cmp: fn(&T, &T) -> bool,
        ) where
            T: PartialOrd + Clone + Default + std::fmt::Debug,
        {
            let data_ptr: *mut T = data.as_mut_ptr();
            let mut heap_len = (offset + 1) as usize;

            for i in 0..bubble_count {
                unsafe {
                    data_ptr.write(data_ptr.offset(offset - i).read());
                }

                bubble_down(data, heap_len, 0, cmp);
                heap_len -= 1;
            }

            assert_eq!(test_data, data);
        }
    }
}
