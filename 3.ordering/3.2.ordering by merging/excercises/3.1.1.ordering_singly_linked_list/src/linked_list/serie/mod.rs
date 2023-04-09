use std::cell::RefCell;
use std::ops::Index;
use std::panic;
use std::rc::Rc;
use std::result::Result;

use std::marker::PhantomData;

mod debug;
mod helpers;
mod iter;

pub struct Serie<'a> {
    phantom: PhantomData<&'a mut i32>,
    len: usize,
    first: Option<Rc<RefCell<Amount>>>,
}

pub struct Amount {
    value: i32,
    next: Option<Rc<RefCell<Amount>>>,
}

impl<'a> Serie<'a> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn swap_with_next(&mut self, index1: usize) -> Result<(), String> {
        let index2 = index1 + 1usize;

        let changing_first = index1 == 0;

        let before_first_index = Serie::before_first(index1, self.len);
        let second = self.traverse_rc(index2)?;

        if before_first_index != index2 {
            let before_first = self.traverse_rc(before_first_index)?;
            let first = self.traverse_rc(index1)?;

            let mut swap = Serie::replace_next_with_next(&before_first, &first);
            swap = Serie::replace_next(&second, &swap);
            _ = Serie::replace_next(&first, &swap);
        }

        if changing_first {
            self.first.replace(second);
        }

        Ok(())
    }

    fn replace_next_with_next(
        dest: &Rc<RefCell<Amount>>,
        source: &Rc<RefCell<Amount>>,
    ) -> Rc<RefCell<Amount>> {
        let replacement;
        unsafe {
            replacement = source.as_ptr().as_ref().unwrap().next.clone();
        }
        (*dest.borrow_mut())
            .next
            .replace(replacement.unwrap())
            .unwrap()
    }

    fn replace_next(
        dest: &Rc<RefCell<Amount>>,
        source: &Rc<RefCell<Amount>>,
    ) -> Rc<RefCell<Amount>> {
        (*dest.borrow_mut()).next.replace(source.clone()).unwrap()
    }

    fn before_first(mut index1: usize, len: usize) -> usize {
        const AJDUSTER: usize = 1usize;

        if index1 == 0 {
            index1 = len;
        }

        index1 - AJDUSTER
    }
}

impl<'a> Index<usize> for Serie<'a> {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        match self.traverse_ref(index) {
            Ok(amount) => &amount.value,
            Err(s) => {
                panic!("{}", s);
            }
        }
    }
}

impl<'a> Into<Vec<i32>> for Serie<'a> {
    fn into(self) -> Vec<i32> {
        self.into_iter().map(|x| x.value).collect::<Vec<i32>>()
    }
}

impl<'a> From<&[i32]> for Serie<'a> {
    fn from(source: &[i32]) -> Serie<'a> {
        if source.len() == 0 {
            return Serie {
                phantom: PhantomData,
                len: 0,
                first: None,
            };
        }

        let mut iter = source.iter().rev();

        let last = Amount {
            value: *iter.next().unwrap(),
            next: None,
        };

        let mut next = Some(Rc::new(RefCell::new(last)));

        let mut last = unsafe { next.as_mut().unwrap().as_ptr().as_mut().unwrap() };

        for value in iter {
            let amount = Amount {
                value: *value,
                next: next,
            };

            next = Some(Rc::new(RefCell::new(amount)));
        }

        last.next = next.clone();

        let serie = Serie {
            phantom: PhantomData,
            len: source.len(),
            first: next,
        };

        serie.check_counts(source, false);

        serie
    }
}
