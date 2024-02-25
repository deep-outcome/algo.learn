use super::{Amount, Serie};

use std::cell::RefCell;
use std::rc::Rc;

use std::marker::PhantomData;

impl<'a> IntoIterator for Serie<'a> {
    type Item = &'a Amount;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(0usize, self.first.clone(), self.len)
    }
}

pub struct Iter<'a> {
    phantom: PhantomData<&'a Amount>,
    iter_count: usize,
    next: Option<Rc<RefCell<Amount>>>,
    len: usize,
}

impl<'a> Iter<'a> {
    pub fn new(iter_count: usize, next: Option<Rc<RefCell<Amount>>>, len: usize) -> Iter<'a> {
        Iter {
            phantom: PhantomData,
            iter_count,
            next,
            len,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Amount;

    fn next(&'_ mut self) -> Option<&'a Amount> {
        let iter_count = self.iter_count;

        if iter_count == self.len {
            return None;
        }

        self.iter_count = iter_count + 1usize;

        let current;
        unsafe {
            current = self
                .next
                .clone()
                .unwrap()
                .as_ref()
                .as_ptr()
                .as_ref()
                .unwrap();
        }

        self.next = current.next.clone();

        Some(current)
    }
}
