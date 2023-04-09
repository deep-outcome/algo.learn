use super::iter::Iter;
use super::{Amount, Serie};

use std::cell::RefCell;
use std::rc::Rc;

impl<'a> Serie<'a> {
    pub fn traverse_ref(&self, virtual_index: usize) -> Result<&Amount, String> {
        let rc = self.traverse_rc(virtual_index)?;

        unsafe { Ok(rc.as_ptr().as_ref().unwrap()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(0usize, self.first.clone(), self.len)
    }

    pub fn traverse_rc(&self, virtual_index: usize) -> Result<Rc<RefCell<Amount>>, String> {
        if virtual_index >= self.len {
            return Err(String::from("Index greater or equal to serie length!"));
        }

        //  print!("rc virtual_index {} ", virtual_index);
        //  println!("rc {:?}", self);

        let mut current = self.first.clone();

        let mut traversing_counter = 0;

        loop {
            if traversing_counter == virtual_index {
                //                 unsafe {
                //                     println!(
                //                         "rc {:?}",
                //                         current.clone().unwrap().as_ptr().as_ref().unwrap()
                //                     );
                //                 }
                return Ok(current.unwrap().clone());
            }

            traversing_counter = traversing_counter + 1usize;

            if current.is_none() {
                return Err(format!(
                    "Serie ended before next reached! Links passed {}.",
                    traversing_counter
                ));
            }

            unsafe {
                current = current.unwrap().as_ptr().as_ref().unwrap().next.clone();
            }
        }
    }
}
