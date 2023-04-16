// - Shell sort works as insertion sort on sublists
// these sublists are arranged by gap (step) in
// array in question so they interleave each other

// - in each round of iterations for next gap each new list
// is merged-sorted exactly from 2 previous lists
// e.g. n=32
// gap=16; L₁=[0,16], …, L₁₆=[15,31]
// gap=8; L₁=L₁₍₁₆₎+L₉₍₁₆₎, …, L₈=L₈₍₁₆₎+L₁₆₍₁₆₎
// gap=4; L₁=L₁₍₈₎+L₅₍₈₎, …, L₄=L₄₍₈₎+L₈₍₈₎
// …

// - same rule implies step iterations count — log₂n
// e.g. n=32 ⇒ gap₁=16, gap₂=8, gap₃=4, gap₄=2, gap₅=1, 2^5=32

// - simple is also to reason about iterations over lists
// clearly there can be only up to len-step iterations in each top (step) iteration
// ex. 1, n=7,
// gap=4: 3, gap=2: 5, gap=1: 6
//
// ex. 2, n=8,
// gap=4: 4, gap=2: 6, gap=1: 7
//
// in other words while left_index+step points to number, there is iteration ⇒ len-step=iterations
// this can be expresed by formula
// log₂n
//   Σ⌊(1-1/2ⁱ)✕n⌋
//  i=1
// upper bound is denoted by repeated n division by 2 ⇒ log₂n
// lower bound expresses power of 2 needed to get step size for respective iteration
// floor is needed to truncate for odd lens
// formula extrapolation back to origin
// log₂n
// Σ⌊(n-n/2ⁱ)⌋ ⇐⇒ x₁+ … +xₘₐₓ = (n-n/2^1)+ … +(n-n/2^max)
// i=1
// formula leverages that step is always division of len by some power of 2
// i.e. gap₂=n/4=n/2/2
// iterations = n*(1-1/2^i)+ … +n*(1-1/2^max)

// - insertion worst case goes when n=2ⁱ and numbers ale layed out such
// that odd positions are ocuppied with # 1,2,3,…,n/2 and even positions
// with # n/2+1,n/2+2,…,n
// this because gap presorts fail in all iterations
// e.g. n=16, let observe L₁ situation
//         1   2   3   4    5   6    7   8    9   10  11   12  13  14   15   16
//       +---+---+---+----+---+----+---+----+---+----+---+----+---+----+---+----+
//       | 1 | 9 | 2 | 10 | 3 | 11 | 4 | 12 | 5 | 13 | 6 | 14 | 7 | 15 | 8 | 16 |
//       +---+---+---+----+---+----+---+----+---+----+---+----+---+----+---+----+
//         ↓       ↓        ↓        ↓        ↓        ↓        ↓        ↓
// gap=8  L₁       ↓        ↓        ↓       L₁        ↓        ↓        ↓
//         ↓       ↓        ↓        ↓        ↓        ↓        ↓        ↓
// gap=4  L₁       ↓       L₁        ↓       L₁        ↓       L₁        ↓
//         ↓       ↓        ↓        ↓        ↓        ↓        ↓        ↓
// gap=2  L₁      L₁       L₁       L₁       L₁       L₁       L₁       L₁
// gap=1  L₁ = original sequence, since same goes for other lists
//
// work left for last and only insertion sort can be then deduced, n=8
// +-----------+-------+----------------------------+
// | iteration | swaps |            layout          |
// +-----------+-------+----------------------------+
// |         1 |     0 | [1], [5], 2, 6, 3, 7, 4, 8 |
// |         2 |     1 | 1, [2], [5], 6, 3, 7, 4, 8 |
// |         3 |     0 | 1, 2, [5], [6], 3, 7, 4, 8 |
// |         4 |     2 | 1, 2, [3], 5, [6], 7, 4, 8 |
// |         5 |     0 | 1, 2, 3, 5, [6], [7], 4, 8 |
// |         6 |     3 | 1, 2, 3, [4], 5, 6, [7], 8 |
// |         7 |     0 | 1, 2, 3, 4, 5, 6, [7], [8] |
// +-----------+-------+----------------------------+
//
// so using integers sum formula s=n(f+l)/2 = n(n+1)/2 (positive integers)
// 1+2+ … +(n/2)-1 = (((n/2)-1)*(n/2)) / 2 ≅ n²/4/2 = n²/8 ⇒ Ο(n²)
// so altogether it is some log₂n + c*n + n² ⇒ Ο(n²)

// - for other gap sequences complexity is different, see for further reference:
// ❥ https://sedgewick.io/wp-content/themes/sedgewick/papers/1996Shellsort.pdf
// ❥ https://en.wikipedia.org/wiki/Shellsort#Gap_sequences


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
    
    let step_cycles = (len as f64).log2().ceil() as usize;    

    let mut right_index_increments = HashMap::<usize, usize>::with_capacity(step_cycles);
    let mut inner_cycles = HashMap::<usize, usize>::with_capacity(step_cycles);

    for i in 1..=step_cycles {
        right_index_increments.insert(i, 0);
        inner_cycles.insert(i, 0);
    }

    let mut step = step(len);

    let mut current_step_cycle = 1;
    while step > 0 {
        let mut r_index = step;

        // need to guard for shorter right side
        while r_index < len {
            // same as insertion sort — take 1ˢᵗ from unordered
            let right = slc[r_index];

            let mut move_to_inx = r_index;

            // and shift all bigger ones (but decrementing using step (gap))
            while right < slc[move_to_inx - step] {
                // COUNTER
                *inner_cycles.get_mut(&current_step_cycle).unwrap() += 1;
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
            *right_index_increments.get_mut(&current_step_cycle).unwrap() += 1;
            // COUNTER
        }

        // proceed with shorter step (gap)
        step = crate::shellsort::step(step);

        current_step_cycle = current_step_cycle + 1;
    }

    println!("len {:?}", len);    
    println!("step_cycles {:?}", step_cycles);
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
