use std::result::Result;

pub struct Counters {
    interval: Vec<u16>,
}

impl Counters {
    pub fn build_with_max(max: u16) -> Counters {
        let len = (max + 1) as usize;
        let mut counters = Vec::with_capacity(len);
        unsafe {
            counters.set_len(len);
        }

        Counters { interval: counters }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u16] {
        self.interval.as_mut_slice()
    }
}

pub fn rusty_sort(nums: &mut [u16], max: u16) -> Result<(), &'static str> {
    let mut counters = Counters::build_with_max(max);
    let counters: &mut [u16] = counters.as_mut_slice();

    counters.fill(0);
    for n in nums.iter() {
        if let Some(c) = counters.get_mut(*n as usize) {
            *c += 1;
        } else {
            return Err("Invalid maximum specified.");
        }
    }

    let nums_len = nums.len();

    let mut output_index = 0;
    for (i, c) in counters.iter().enumerate() {
        let i = i as u16;
        let mut c = *c;

        while c > 0 {
            nums[output_index] = i;

            output_index += 1;
            c -= 1;
        }

        // optimization; not part of source (original) algorithm
        if output_index == nums_len {
            break;
        }
    }

    Ok(())
}

// θ(r+n) [θ(2r+2n)] resp. [θ(2n)+θ(r)+𝚶(r)]
pub fn course_lesson_bound_sort(nums: &mut [u16], max: u16) {
    let mut counters = Counters::build_with_max(max);
    let counters: &mut [u16] = counters.as_mut_slice();
    let counters_len = counters.len();

    // θ(r); r = max
    let mut index = 0;
    while index < counters_len {
        counters[index] = 0;
        index += 1;
    }
    // θ(n)
    for n in nums.iter() {
        counters[*n as usize] += 1;
    }

    let nums_len = nums.len();

    // θ(r+n)
    let mut output_index = 0;
    let mut n = 0;
    for c in counters {
        let mut c = *c;
        while c > 0 {
            nums[output_index] = n;

            output_index += 1;
            c -= 1;
        }

        // optimization; not part of source (original) algorithm, changes this part θ(n+r) to (θ(n)+𝚶(r))
        if output_index == nums_len {
            break;
        }

        n += 1;
    }
}

#[cfg(test)]
mod test {
    use super::course_lesson_bound_sort;

    #[test]
    fn ordering_test() {
        let mut input: [u16; 6] = [5, 4, 3, 2, 1, 0];

        course_lesson_bound_sort(&mut input, 5);
        assert_eq!([0, 1, 2, 3, 4, 5], input);
    }

    #[test]
    fn counting_test() {
        let mut input = vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 5, 5];
        let criterion = input.clone();

        course_lesson_bound_sort(&mut input, 5);
        assert_eq!(criterion, input);
    }
    
    #[test]
    fn load_test() {
        let mut input = vec![3, 4, 3, 2, 2, 3, 3, 1, 3, 4, 1, 1, 5, 4, 2];
        let mut criterion = input.clone();
        criterion.sort();

        course_lesson_bound_sort(&mut input, 5);
        assert_eq!(criterion, input);
    }

    #[test]
    fn counters_test() {
        let mut input = [100, 1];

        course_lesson_bound_sort(&mut input, 100);
        assert_eq!([1, 100], input);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 100 but the index is 100")]
    fn invalid_range_test() {
        let mut input = [100, 1];

        course_lesson_bound_sort(&mut input, 99);
    }
}

#[cfg(test)]
mod rusty_sort_test {
    use super::rusty_sort;

    #[test]
    fn ordering_test() {
        let mut input: [u16; 6] = [5, 4, 3, 2, 1, 0];

        let result = rusty_sort(&mut input, 5);
        assert_eq!(Ok(()), result);
        assert_eq!([0, 1, 2, 3, 4, 5], input);
    }

    #[test]
    fn counting_test() {
        let mut input = vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 5, 5];
        let criterion = input.clone();

        let result = rusty_sort(&mut input, 5);
        assert_eq!(Ok(()), result);
        assert_eq!(criterion, input);
    }
    
    #[test]
    fn load_test() {
        let mut input = vec![3, 4, 3, 2, 2, 3, 3, 1, 3, 4, 1, 1, 5, 4, 2];
        let mut criterion = input.clone();
        criterion.sort();

        let result = rusty_sort(&mut input, 5);
        assert_eq!(Ok(()), result);
        assert_eq!(criterion, input);
    }

    #[test]
    fn counters_test() {
        let mut input = [100, 1];

        let result = rusty_sort(&mut input, 100);
        assert_eq!(Ok(()), result);
        assert_eq!([1, 100], input);
    }

    #[test]
    fn invalid_range_test() {
        let mut input = [100, 1];

        let result = rusty_sort(&mut input, 99);
        assert_eq!(Err("Invalid maximum specified."), result);
    }
}
