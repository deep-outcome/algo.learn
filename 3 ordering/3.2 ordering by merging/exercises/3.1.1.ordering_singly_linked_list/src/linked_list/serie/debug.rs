use super::{Amount, Serie};
use std::rc::Rc;

use std::fmt::{Debug, Formatter};

impl<'a> Debug for Serie<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        _ = f.write_str("debug print ");

        let mut index = 0;

        let mut detail = String::from("[");
        for next in self.iter() {
            detail = detail + &format!("{}{:?}", index, next) + ", ";
            index = index + 1;
        }

        let detail = detail.trim_end().trim_end_matches(',');
        _ = f.write_str(detail);
        _ = f.write_str("]");

        Ok(())
    }
}

impl Debug for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("(")?;
        f.write_str(&self.value.to_string())?;
        f.write_str(";-->")?;

        let next = match &self.next.clone() {
            None => String::default(),
            Some(next) => unsafe { next.as_ptr().as_ref().unwrap().value.to_string() },
        };

        f.write_str(&next)?;
        f.write_str(")")
    }
}

impl<'a> Serie<'a> {
    pub fn check_counts(&self, source: &[i32], print_info: bool) {
        let mut next_ref = &self.first;

        let mut first = true;

        for i in source {
            if print_info {
                println!();
                println!("i {} ", i);
            }

            let unwrapped = &next_ref.as_ref().unwrap();

            unsafe {
                next_ref = &unwrapped.as_ptr().as_ref().unwrap().next;

                if print_info {
                    println!(
                        "nval {} ",
                        next_ref.as_ref().unwrap().as_ptr().as_ref().unwrap().value
                    );
                }
            }

            if first {
                assert_eq!(2, Rc::strong_count(unwrapped));
                first = false;
            } else {
                assert_eq!(1, Rc::strong_count(unwrapped));
            }

            if print_info {
                println!("sc {} ", Rc::strong_count(unwrapped));
                println!("wc {} ", Rc::weak_count(unwrapped));
            }
        }
    }
}
