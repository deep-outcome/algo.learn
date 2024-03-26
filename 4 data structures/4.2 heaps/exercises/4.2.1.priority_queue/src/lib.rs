#![allow(dead_code)]

use std::cmp::{Ord, Ordering};

pub const MAX_LEVELS: usize = 25;

type Form = PriorityQueueForm;
type Item<T> = PriorityQueueItem<T>;
type Queue<T> = PriorityQueue<T>;

#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum PriorityQueueForm {
    Maximal,
    Minimal,
    Undefined,
}

/// Priority queue realized
/// via fixed binary heap
pub struct PriorityQueue<T>
where
    T: Clone + Default + Priority,
{
    data: Box<[Item<T>]>,
    len: usize,
    form: Form,
    rank: u64,
}

#[derive(PartialEq, Clone, Debug)]
struct PriorityQueueItem<T>
where
    T: Priority,
{
    item: T,
    rank: u64,
}

pub trait Priority {
    type Priority: Ord;
    fn prio(&self) -> Self::Priority;
}

impl Priority for u16 {
    type Priority = u16;
    fn prio(&self) -> u16 {
        let mut num = *self as i32;
        num = (num - u16::MAX as i32) * -1;

        num as u16
    }
}

impl<T> Priority for Item<T>
where
    T: Priority,
{
    type Priority = <T as Priority>::Priority;
    fn prio(&self) -> <T as Priority>::Priority {
        self.item.prio()
    }
}

impl<T> Queue<T>
where
    T: Priority + Clone + Default,
{
    pub fn new(levels: usize, f: Form) -> Self {
        assert!(
            levels <= MAX_LEVELS,
            "Maximum supported levels is 25. 0 for root only."
        );

        let nodes = 2usize.pow((levels + 1) as u32) - 1;
        let data = vec![
            Item {
                item: T::default(),
                rank: 0
            };
            nodes
        ]
        .into_boxed_slice();

        Self {
            data,
            len: 0,
            form: f,
            rank: 0,
        }
    }

    pub fn insert(&mut self, t: T) -> Result<(), ()> {
        let wrix = self.len;
        let data = &mut self.data;

        let cap = data.len();

        if wrix == cap {
            return Err(());
        }

        let rank = self.rank;
        let que_ite = &mut data[wrix];

        que_ite.item = t;
        que_ite.rank = rank;

        self.len = wrix + 1;
        self.rank = rank + 1;

        self.bubble_up(wrix);

        Ok(())
    }

    pub fn peek_root(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        } else {
            Some(&self.data[0].item)
        }
    }

    pub fn extract_root(&mut self) -> Option<T> {
        let len = self.len;

        if len == 0 {
            return None;
        }

        let new_len = len - 1;

        let data = &mut self.data;
        let root = data[0].item.clone();
        data[0] = data[new_len].clone();

        self.len = new_len;
        self.bubble_down(0);

        Some(root)
    }

    /// `disp` — disposition
    /// `cand` — candidate
    /// `eq_chk` — optional equality check
    /// `bubble_up` is called only from `insert` thus
    /// `cand` has always greater (globally greatest) rank
    /// therefore is okay to result just on value equality
    fn for_repl(f: Form, disp: &Item<T>, cand: &Item<T>, eq_chk: bool) -> bool {
        let mut can_ord = cand.prio().cmp(&disp.prio());

        if eq_chk && can_ord == Ordering::Equal {
            can_ord = cand.rank.cmp(&disp.rank);

            match can_ord {
                Ordering::Less => true,
                Ordering::Greater => false,
                _ => panic!("Code broken elsewhere."),
            }
        } else {
            if f == Form::Minimal {
                return can_ord == Ordering::Less;
            }

            if f == Form::Maximal {
                return can_ord == Ordering::Greater;
            }

            panic!("Unsupported queue form.");
        }
    }

    // `des_ix` = descendant index
    fn bubble_up(&mut self, mut des_ix: usize) {
        let data = &mut self.data;
        let form = self.form.clone();

        while des_ix > 0 {
            // predecessor index
            let pred_ix = (des_ix - 1) / 2;

            let predecessor = data[pred_ix].clone();
            let descendant = data[des_ix].clone();

            if Self::for_repl(form.clone(), &predecessor, &descendant, false) {
                data[pred_ix] = descendant;
                data[des_ix] = predecessor;

                des_ix = pred_ix;
            } else {
                break;
            }
        }
    }

    // `pred_ix` = predecessor index
    fn bubble_down(&mut self, mut pred_ix: usize) {
        let len = self.len;
        let form = self.form.clone();

        let data = &mut self.data;

        loop {
            // descendant index
            let mut des_ix = 2 * pred_ix + 1;

            if des_ix >= len {
                break;
            }

            let des2_ix = des_ix + 1;
            if des2_ix < len && Self::for_repl(form.clone(), &data[des_ix], &data[des2_ix], true) {
                des_ix = des2_ix;
            }

            let predecessor = data[pred_ix].clone();
            let descendant = data[des_ix].clone();
            if Self::for_repl(form.clone(), &predecessor, &descendant, true) {
                data[pred_ix] = descendant;
                data[des_ix] = predecessor;

                pred_ix = des_ix;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests_of_units {

    mod new {
        use crate::{Form, Queue};

        #[test]
        fn new() {
            let queue = Queue::<u16>::new(2, Form::Minimal);

            assert_eq!(0, queue.len);
            assert_eq!(0, queue.rank);
            assert!(queue
                .data
                .iter()
                .all(|x| x.item == u16::default() && x.rank == 0));
            assert_eq!(2usize.pow(3) - 1, queue.data.len());
            assert_eq!(Form::Minimal, queue.form);
        }

        #[test]
        #[should_panic(expected = "Maximum supported levels is 25. 0 for root only.")]
        fn unsupported_level_count() {
            _ = Queue::<u16>::new(26, Form::Minimal);
        }
    }

    mod insertion {
        use super::qi;
        use crate::{Form, Queue};
        use std::ops::Deref;

        #[test]
        fn prio() {
            #[rustfmt::skip]
            let test_cases = [
                (Form::Minimal, [8,9], [qi(9,1), qi(8,0), qi(0,0)],),
                (Form::Maximal, [9,8], [qi(8,1), qi(9,0), qi(0,0)],),
            ];

            for case in test_cases {
                let mut queue = Queue::<u16>::new(1, case.0);

                let nums: [u16; 2] = case.1;

                let mut count = 0;
                for n in nums {
                    assert_eq!(Ok(()), queue.insert(n));
                    count += 1;
                    assert_eq!(count, queue.len);
                }

                assert_eq!(&case.2, queue.data.deref());
            }
        }

        #[test]
        fn rank() {
            #[rustfmt::skip]
            let test_cases = [
                (Form::Minimal, [2,2], [qi(2,0), qi(2,1), qi(0,0)],),
                (Form::Maximal, [2,2], [qi(2,0), qi(2,1), qi(0,0)],),
            ];

            for case in test_cases {
                let mut queue = Queue::<u16>::new(1, case.0);

                let nums: [u16; 2] = case.1;

                let mut count = 0;
                for n in nums {
                    assert_eq!(Ok(()), queue.insert(n));
                    count += 1;
                    assert_eq!(count, queue.len);
                    assert_eq!(count as u64, queue.rank);
                }

                assert_eq!(&case.2, queue.data.deref());
            }
        }

        #[test]
        fn full_error() {
            let mut queue = Queue::<u16>::new(0, Form::Minimal);

            assert_eq!(Ok(()), queue.insert(0));
            assert_eq!(Err(()), queue.insert(0));
        }
    }

    mod peek_root {
        use crate::{Form, Item, Queue};

        #[test]
        fn none_root() {
            let queue = Queue::<u16>::new(0, Form::Minimal);
            assert_eq!(None, queue.peek_root());
        }

        #[test]
        fn some_root() {
            let queue: Queue<u16> = Queue {
                data: Box::new([Item { item: 90, rank: 0 }]),
                len: 1,
                form: Form::Minimal,
                rank: 1,
            };

            assert_eq!(Some(&queue.data[0].item), queue.peek_root());
        }
    }

    mod extraction {
        use super::{qi, que_ims};
        use crate::{Form, Item, Queue};

        #[test]
        fn prio() {
            const LEN: usize = 4;

            let test_cases = [
                (Form::Maximal, [8, 10, 9, 10], [8, 9, 10, 10]),
                (Form::Minimal, [10, 8, 9, 8], [10, 9, 8, 8]),
            ];

            for case in test_cases {
                let mut queue = Queue::<u16>::new(2, case.0);
                let data = &mut queue.data;

                let input_data = case.1;
                for ix in 0..LEN {
                    data[ix] = qi(input_data[ix], ix as u64);
                }

                queue.len = LEN;
                let mut queue_len = LEN;

                for num in case.2 {
                    assert_eq!(Some(num), queue.extract_root());
                    queue_len -= 1;
                    assert_eq!(queue_len, queue.len);
                }
            }
        }

        #[test]
        fn rank() {
            use std::ops::Deref;

            const LEN: usize = 3;

            let test_cases = [(Form::Maximal, [7; LEN]), (Form::Minimal, [8; LEN])];

            for case in test_cases {
                let mut queue = Queue::<u16>::new(2, case.0);

                {
                    let data = &mut queue.data;
                    let input_data = case.1;

                    for i in 0..LEN {
                        data[i] = qi(input_data[i], i as u64);
                    }

                    queue.len = LEN;
                }

                let data: *const Item<u16> = queue.data.deref().as_ptr();

                let test_data = que_ims(&case.1);

                let mut queue_len = LEN;
                for ix in 0..LEN {
                    let expe_rank = &test_data[ix].rank;
                    let root_rank = unsafe { data.offset(0).read() }.rank;
                    assert_eq!(*expe_rank, root_rank);

                    _ = queue.extract_root();

                    queue_len -= 1;
                    assert_eq!(queue_len, queue.len);
                }
            }
        }

        #[test]
        fn empty() {
            let mut queue = Queue::<u16>::new(0, Form::Minimal);
            assert_eq!(None, queue.extract_root());
        }
    }

    mod for_repl {
        use super::qi;
        use crate::{Form, PriorityQueue};

        #[test]
        fn eq_chk() {
            let test_cases = [
                (qi(11, 1), qi(11, 2), false, Form::Undefined),
                (qi(11, 2), qi(11, 1), true, Form::Undefined),
                (qi(11, 5), qi(10, 5), false, Form::Minimal),
                (qi(10, 5), qi(11, 5), true, Form::Minimal),
                (qi(11, 5), qi(10, 5), true, Form::Maximal),
                (qi(10, 5), qi(11, 5), false, Form::Maximal),
            ];

            for case in test_cases {
                assert_eq!(
                    case.2,
                    PriorityQueue::for_repl(case.3, &case.0, &case.1, true)
                );
            }
        }

        #[test]
        fn no_eq_chk() {
            let test_cases = [
                (qi(11, 5), qi(11, 4), false, Form::Minimal),
                (qi(11, 5), qi(11, 4), false, Form::Maximal),
                (qi(10, 4), qi(11, 5), true, Form::Minimal),
                (qi(11, 5), qi(10, 4), false, Form::Minimal),
                (qi(11, 4), qi(10, 5), true, Form::Maximal),
                (qi(10, 5), qi(11, 4), false, Form::Maximal),
            ];

            for case in test_cases {
                assert_eq!(
                    case.2,
                    PriorityQueue::for_repl(case.3, &case.0, &case.1, false)
                );
            }
        }

        #[test]
        #[should_panic(expected = "Unsupported queue form.")]
        fn unsupp_form() {
            _ = PriorityQueue::for_repl(Form::Undefined, &qi(0, 0), &qi(0, 0), false)
        }

        #[test]
        #[should_panic(expected = "Code broken elsewhere.")]
        fn equal_ranks() {
            _ = PriorityQueue::for_repl(Form::Undefined, &qi(11, 11), &qi(11, 11), true)
        }
    }

    mod bubble_up {
        use super::{qi, que_ims};
        use crate::{Form, Item, Queue};
        use std::ops::Deref;

        #[test]
        fn prio() {
            #[rustfmt::skip]
            let test_cases = [
                (Form::Minimal, [1, 0, 2, 3, 4, 5, 6, 7, 8, 9],
                [qi(9,9),qi(8,8),qi(5,5),qi(6,6),qi(7,7),qi(1,0),qi(4,4),qi(0,1),qi(3,3),qi(2,2),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]),
                (Form::Maximal, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                [qi(0,9),qi(1,8),qi(4,5),qi(3,6),qi(2,7),qi(8,1),qi(5,4),qi(9,0),qi(6,3),qi(7,2),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]),
            ];

            for case in test_cases {
                let mut queue = Queue::<u16>::new(3, case.0);
                let data = queue.data.as_mut_ptr();

                let nums: [u16; 10] = case.1;

                let mut wri_ix = 0;
                let mut rank = 0;

                for n in nums {
                    let qi = Item { item: n, rank };
                    rank += 1;

                    unsafe {
                        data.offset(wri_ix).write(qi);
                    }

                    queue.bubble_up(wri_ix as usize);
                    wri_ix += 1;
                }

                assert_eq!(&case.2, queue.data.deref());
            }
        }

        #[test]
        fn rank() {
            let test_cases = [(Form::Minimal, [2; 7]), (Form::Maximal, [3; 7])];

            for case in test_cases {
                let mut queue = Queue::<u16>::new(2, case.0);
                let data = queue.data.as_mut_ptr();

                let nums: [u16; 7] = case.1;

                let mut wri_ix = 0;
                let mut rank = 0;

                for n in nums {
                    let qi = Item { item: n, rank };
                    rank += 1;

                    unsafe {
                        data.offset(wri_ix).write(qi);
                    }

                    queue.bubble_up(wri_ix as usize);
                    wri_ix += 1;
                }

                assert_eq!(que_ims(&nums), queue.data.deref());
            }
        }
    }

    mod bubble_down {

        use super::{qi, que_ims};
        use crate::{Form, Item, Queue};
        use std::ops::Deref;

        #[test]
        fn maximal() {
            let queue_data: [u16; 15] = [9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0];
            let mut queue_data = que_ims(&queue_data);

            queue_data[0].rank = 9;

            for i in 10..15 {
                queue_data[i].rank = 0;
            }

            let mut queue: Queue<u16> = Queue {
                data: Box::new(queue_data),
                len: 9,
                form: Form::Maximal,
                rank: 10,
            };

            queue.bubble_down(0);

            #[rustfmt::skip]
                let test_data = [qi(1,1),qi(3,3),qi(2,2),qi(7,7),qi(4,4),qi(5,5),qi(6,6),qi(9,9),qi(8,8),
                                 qi(9,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)];

            assert_eq!(&test_data, queue.data.deref());

            #[rustfmt::skip]
            segment_test(&mut queue,8,3,
                         &[qi(4,4),qi(6,6),qi(5,5),qi(7,7),qi(9,9),qi(8,8),
                          qi(6,6),qi(9,9),qi(8,8),qi(9,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]);
            #[rustfmt::skip]
            segment_test(&mut queue,5,4,
                         &[qi(8,8),qi(9,9),
                         qi(8,8),qi(9,9),qi(9,9),qi(8,8),qi(6,6),qi(9,9),qi(8,8),qi(9,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]);
        }

        #[test]
        fn minimal() {
            let queue_data: [u16; 15] = [0, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0];
            let mut queue_data = que_ims(&queue_data);

            queue_data[0].rank = 9;

            for i in 10..15 {
                queue_data[i].rank = 0;
            }

            let mut queue: Queue<u16> = Queue {
                data: Box::new(queue_data),
                len: 9,
                form: Form::Minimal,
                rank: 10,
            };

            queue.bubble_down(0);
            #[rustfmt::skip]
            let test_data = [qi(8,1),qi(6,3),qi(7,2),qi(2,7),qi(5,4),qi(4,5),qi(3,6),qi(0,9),qi(1,8),
                             qi(0,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)];

            assert_eq!(&test_data, queue.data.deref());

            #[rustfmt::skip]
            segment_test(&mut queue,8,3,
                         &[qi(5,4),qi(3,6),qi(4,5),qi(2,7),qi(0,9),qi(1,8),
                          qi(3,6),qi(0,9),qi(1,8),qi(0,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]);

            #[rustfmt::skip]
            segment_test(&mut queue,5,4,
                         &[qi(1,8),qi(0,9),
                         qi(1,8),qi(0,9),qi(0,9),qi(1,8),qi(3,6),qi(0,9),qi(1,8),qi(0,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]);
        }

        #[test]
        fn rank() {
            let test_cases = [(Form::Minimal, [4; 15]), (Form::Maximal, [4; 15])];

            for case in test_cases {
                let mut queue_data = que_ims(&case.1);

                for i in 10..15 {
                    let qi = &mut queue_data[i];
                    qi.rank = 0;
                    qi.item = 0;
                }

                queue_data[0] = queue_data[9].clone();

                let mut queue: Queue<u16> = Queue {
                    data: Box::new(queue_data),
                    len: 9,
                    form: case.0,
                    rank: 10,
                };

                queue.bubble_down(0);

                #[rustfmt::skip]
                let test_data = [qi(4,1),qi(4,3),qi(4,2),qi(4,7),qi(4,4),qi(4,5),qi(4,6),qi(4,9),qi(4,8),
                                 qi(4,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)];

                assert_eq!(&test_data, queue.data.deref());

                #[rustfmt::skip]
                segment_test(&mut queue,8,3,
                             &[qi(4,4),qi(4,6),qi(4,5),qi(4,7),qi(4,9),qi(4,8),
                              qi(4,6),qi(4,9),qi(4,8),qi(4,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]);

                #[rustfmt::skip]
                segment_test(&mut queue,5,4,
                             &[qi(4,8),qi(4,9),
                              qi(4,8),qi(4,9),qi(4,9),qi(4,8),qi(4,6),qi(4,9),qi(4,8),qi(4,9),qi(0,0),qi(0,0),qi(0,0),qi(0,0),qi(0,0)]);
            }
        }

        // district-start helpers

        fn segment_test(
            queue: &mut Queue<u16>,
            offset: isize,
            bubble_count: isize,
            test_data: &[Item<u16>; 15],
        ) {
            let queue_data_ptr: *mut Item<u16> = queue.data.as_mut_ptr();

            for i in 0..bubble_count {
                unsafe {
                    queue_data_ptr.write(queue_data_ptr.offset(offset - i).read());
                }

                queue.len = queue.len - 1;
                queue.bubble_down(0);
            }

            assert_eq!(test_data, queue.data.deref());
        }

        // district-end helpers
    }

    mod priority {

        use crate::Priority;
        #[test]
        fn u16_prio_test() {
            assert_eq!(0, u16::MAX.prio());
            assert_eq!(1, u16::MAX - 1u16.prio());
            assert_eq!(u16::MAX, 0u16.prio());
            assert_eq!(u16::MAX - 1, 1u16.prio());
        }
    }

    // district-start helpers

    use crate::Item;

    fn que_ims<const N: usize>(arr: &[u16; N]) -> [Item<u16>; N] {
        let mut rank = -1i64;
        arr.map(|x| {
            rank += 1;
            Item {
                item: x,
                rank: rank as u64,
            }
        })
    }

    fn qi(item: u16, rank: u64) -> Item<u16> {
        Item { item, rank }
    }

    // district-end helpers
}
