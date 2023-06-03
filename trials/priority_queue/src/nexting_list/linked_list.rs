#![allow(dead_code)]

#[derive(Debug)]
struct Nodule<T> {
    val: T,
    prio: usize,
    next: Option<Box<Nodule<T>>>,
}

#[derive(Debug)]
struct NextingList<T>
where
    T: Default,
{
    pseudo_head: Box<Nodule<T>>,
}

impl<T> NextingList<T>
where
    T: Default,
{
    pub fn new() -> NextingList<T> {
        NextingList {
            pseudo_head: Box::new(Nodule {
                val: T::default(),
                prio: usize::default(),
                next: None,
            }),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let head = &self.pseudo_head.next;
        if head.is_some() {
            Some(&head.as_ref().unwrap().val)
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = &mut self.pseudo_head.next;
        if let Some(mut h) = head.take() {
            *head = h.next.take();
            Some((*h).val)
        } else {
            None
        }
    }

    pub fn push(&mut self, val: T, prio: usize) {
        let mut nod = Box::new(Nodule {
            val,
            prio,
            next: None,
        });

        let mut curr_nod = &mut self.pseudo_head;

        loop {
            let next = &mut curr_nod.next;
            if next.is_none() {
                *next = Some(nod);
                break;
            }

            if next.as_ref().unwrap().prio >= prio {
                nod.next = next.take();
                *next = Some(nod);
                break;
            }

            curr_nod = next.as_mut().unwrap();
        }
    }
}

#[cfg(test)]
mod units_of_test {
    use super::*;

    #[test]
    fn basic_test() {
        let mut list = NextingList::<usize>::new();

        list.push(0, 1);
        list.push(1, 2);

        for i in [0, 1] {
            assert_eq!(i, list.pop().unwrap());
        }
    }

    #[test]
    fn tail_prioritizing_test() {
        let mut list = NextingList::<usize>::new();

        let vals = 0..=3;
        let prios = (10..=13).rev();

        for (v, p) in vals.clone().zip(prios.clone()) {
            list.push(v, p);
        }

        for v in vals.rev() {
            assert_eq!(v, list.pop().unwrap());
        }
    }

    #[test]
    fn insert_prioritizing_test() {
        let mut list = NextingList::<usize>::new();

        let base_vals = [(9, 0), (8, 1), (5, 4), (4, 5)];
        let insertions = [(7, 2), (6, 3)];

        for (v, p) in base_vals {
            list.push(v, p);
        }

        for (v, p) in insertions {
            list.push(v, p);
        }

        for i in (4..=9).rev() {
            assert_eq!(i, list.pop().unwrap());
        }
    }

    #[test]
    fn new_test() {
        let list = NextingList::<usize>::new();

        let ps_hd = &list.pseudo_head;
        assert_eq!(usize::default(), ps_hd.val);
        assert_eq!(usize::default(), ps_hd.prio);
        assert!(ps_hd.next.is_none());
    }

    #[test]
    fn peek_and_pop() {
        let mut list = NextingList::<usize>::new();

        assert!(list.peek().is_none());
        assert!(list.pop().is_none());

        let vals = 0..=9;
        let prios = (20..=29).rev();

        for (v, p) in vals.clone().zip(prios.clone()) {
            list.push(v, p);
        }

        for v in vals.rev() {
            let peek = list.peek();
            assert!(peek.is_some());
            assert_eq!(v, *peek.unwrap());

            let pop = list.pop();
            assert!(pop.is_some());
            assert_eq!(v, pop.unwrap());
        }

        assert!(list.peek().is_none());
        assert!(list.pop().is_none());
    }
}
