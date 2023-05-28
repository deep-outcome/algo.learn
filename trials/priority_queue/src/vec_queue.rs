#![allow(dead_code)]

use std::cmp::{Eq, PartialOrd};
use std::fmt::Debug;

#[derive(Debug)]
struct Item<T>
where
    T: Debug,
{
    val: T,
    prio: usize,
}

struct Queue<T>
where
    T: PartialOrd + Eq + Debug,
{
    items: Vec<Item<T>>,
}

impl<T> Queue<T>
where
    T: PartialOrd + Eq + Debug,
{
    pub fn new() -> Self {
        Queue {
            items: Vec::<Item<T>>::new(),
        }
    }

    pub fn peek_ix(&self) -> Option<usize> {
        let items = &self.items;
        if items.len() == 0 {
            return None;
        }

        let mut high_prio = usize::MAX;
        let mut high_ix = usize::MAX;

        for (ix, it) in items.iter().enumerate() {
            let prio = it.prio;

            if prio == usize::MAX && high_ix == usize::MAX {
                // this condition can be satisfied only
                // when 0-index `it` has `usize::MAX` priority
                high_ix = 0                 
            }
            // lower number denotes higher priority
            else if prio < high_prio {
                high_prio = prio;
                high_ix = ix;
            }
        }

        Some(high_ix)
    }

    pub fn deq(&mut self) -> Option<T> {
        let ix = self.peek_ix();

        if let Some(ix) = ix {
            Some(self.items.remove(ix).val)
        } else {
            None
        }
    }

    pub fn enq(&mut self, val: T, prio: usize) {
        self.items.push(Item { val, prio });
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn basic_test() {
        let mut que = Queue::new();

        let items = [0, 1, 3, 4, 6, 7];
        for (it, prio) in items
            .iter()
            .zip(items.iter().enumerate().rev().map(|(ix, _)| ix))
        {
            que.enq(*it, prio + 2);
        }

        que.enq(9, 1);
        que.enq(8, 1);

        assert_eq!(Some(9), que.deq());
        assert_eq!(Some(8), que.deq());

        for it in items.iter().rev() {
            assert_eq!(Some(*it), que.deq());
        }

        assert_eq!(None, que.deq());
    }

    #[test]
    fn enq_deq_basic_test() {
        let mut que = Queue::new();

        que.enq(6, usize::MIN);
        assert_eq!(Some(6), que.deq());
    }

    #[test]
    fn enq_deq_fifo_test() {
        let mut que = Queue::new();

        que.enq(6, 2);
        que.enq(9, 1);
        que.enq(6, 2);
        que.enq(8, 1);

        assert_eq!(Some(9), que.deq());
        assert_eq!(Some(8), que.deq());
    }

    #[test]
    fn deq_0_items() {
        let mut que = Queue::<usize>::new();

        let deq = que.deq();
        assert_eq!(None, deq);
    }

    #[test]
    fn deq_usize_max_prio_test() {
        let mut que = Queue::<usize>::new();
        que.enq(4, usize::MAX);
        que.enq(3, 0);

        assert_eq!(Some(3), que.deq());
        assert_eq!(Some(4), que.deq());
    }

    #[test]
    fn peek_fifo_test() {
        let mut que = Queue::new();

        que.enq(6, 2);
        que.enq(9, 1);
        que.enq(6, 2);
        que.enq(9, 1);

        assert_eq!(Some(1), que.peek_ix());
        que.deq();        

        assert_eq!(Some(2), que.peek_ix());
    }

    #[test]
    fn peek_0_items() {
        let que = Queue::<usize>::new();

        let peek = que.peek_ix();
        assert_eq!(None, peek);
    }

    #[test]
    fn peek_usize_max_prio_test() {
        let mut que = Queue::<usize>::new();
        que.enq(3, usize::MAX);
        que.enq(3, 0);

        assert_eq!(Some(1), que.peek_ix());
        _ = que.deq();
        assert_eq!(Some(0), que.peek_ix());
    }
}
