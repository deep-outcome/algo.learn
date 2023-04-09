// - Shell sort works as insertion sort on sublists
// - these sublists are arranged by gap (step) in
// array in question so they interleave each other

// - each step (next round of iterations for next gap)
// merges-sorts 2 of previous lists
// e.g. n=32, L₁…L₁₆, each of len=2, gap=16
// then when:
// gap=8; L₁=L₁₍₁₆₎+L₉₍₁₆₎, L₈=L₈₍₁₆₎+L₁₆₍₁₆₎
// gap=4; L₁=L₁₍₈₎+L₅₍₈₎, L₄=L₄₍₈₎+L₈₍₈₎
// …
// - same rule implies outer iterations count — log₂n
// e.g. n=32 ⇒ gap₁=16, gap₂=8, gap₃=4, gap₄=2, gap₅=1, 2^5=32

fn step(val: usize) -> usize {
    if val == 1 {
        0
    } else {
        (val as f64 / 2f64).ceil() as usize
    }
}

use std::collections::HashMap;

#[allow(dead_code)]
pub fn shellsort(slc: &mut [usize]) {
    let len = slc.len();

    if len < 2 {
        return;
    }

    let mut top_cycles = 0usize;

    let precomputed_top_cycles = (len as f64).log2().ceil() as usize;

    let mut right_index_increments = HashMap::<usize, usize>::with_capacity(precomputed_top_cycles);
    let mut inner_cycles = HashMap::<usize, usize>::with_capacity(precomputed_top_cycles);

    for i in 1..=precomputed_top_cycles {
        right_index_increments.insert(i, 0);
        inner_cycles.insert(i, 0);
    }

    let mut step = step(len);

    while step > 0 {
        // COUNTER
        top_cycles = top_cycles + 1;
        // COUNTER

        let mut r_index = step;

        // need to guard for shorter right side
        while r_index < len {
            // same as insertion sort — take 1ˢᵗ from unordered
            let right = slc[r_index];

            let mut move_to_inx = r_index;

            // and shift all bigger ones — with exception to decrement using step (gap)
            while right < slc[move_to_inx - step] {
                // COUNTER
                *inner_cycles.get_mut(&top_cycles).unwrap() += 1;
                // COUNTER

                let move_from_inx = move_to_inx - step;
                slc[move_to_inx] = slc[move_from_inx];

                move_to_inx = move_from_inx;
                if move_to_inx < step {
                    // either there are no more previous ones
                    // and `right` belongs to Lₓ list start
                    // or `right` is no longer smaller (while condition)
                    break;
                }
            }

            slc[move_to_inx] = right;
            // proceed with next pair
            r_index = r_index + 1;

            // COUNTER
            *right_index_increments.get_mut(&top_cycles).unwrap() += 1;
            // COUNTER
        }

        // proceed with shorter step (gap)
        step = crate::shellsort::step(step);
    }
    
    println!("len {:?}", len);
    
    println!("top_cycles {:?}", top_cycles);
    println!("precomputed_top_cycles {:?}", precomputed_top_cycles);
    let mut right_index_increments = right_index_increments.iter().collect::<Vec<_>>();
    right_index_increments.sort_by_key(|x| *x.0);    
    println!("right_index_increments {:?}", right_index_increments);
    
    let mut inner_cycles = inner_cycles.iter().collect::<Vec<_>>();
    inner_cycles.sort_by_key(|x| *x.0);
    println!("inner_cycles {:?}", inner_cycles);
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn basic_test() {
        let mut arr = [8, 7, 6, 5, 4, 3, 2, 1];
        let mut criterion = arr.clone();
        criterion.sort();

        shellsort(&mut arr);

        assert_eq!(criterion, arr);
    }

    #[test]
    fn odd_len() {
        let mut arr = [8, 7, 6, 5, 4, 3, 2];
        let mut criterion = arr.clone();
        criterion.sort();

        shellsort(&mut arr);

        assert_eq!(criterion, arr);
    }

    #[test]
    fn complex_test() {
        let mut arr = [
            825, 745, 236, 425, 234, 7843, 2002, 855, 735, 2116, 4555, 934, 7143, 102, 553, 733,
            2126, 4445, 954, 43, 2,
        ];
        let mut criterion = arr.clone();
        criterion.sort();

        shellsort(&mut arr);

        assert_eq!(criterion, arr);
    }
}
