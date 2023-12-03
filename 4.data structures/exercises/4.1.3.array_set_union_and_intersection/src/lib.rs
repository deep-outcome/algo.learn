#![allow(dead_code)]
#![allow(non_snake_case)]

use std::vec::Vec;

/// let assume set representation by ordered array
/// then there exists linear implementation
/// for intersection and union

fn intersection<T>(A: &[T], B: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut i = 0;
    let mut j = 0;

    let A_len = A.len();
    let B_len = B.len();

    let mut output = Vec::new();

    // TC: Ο(m+n)
    // TC: Ω(m or n)
    while j < B_len && i < A_len {
        if A[i] < B[j] {
            i += 1;
            continue;
        }

        if A[i] > B[j] {
            j += 1;
            continue;
        }

        output.push(A[i]);
        i += 1;
        j += 1;
    }

    output
}

#[cfg(test)]
mod tests_of_units {
    use super::intersection;

    #[test]
    fn basic_test() {
        let A = [1, 3, 5, 6, 7];
        let B = [2, 3, 4, 6];

        let test = intersection(&A, &B);
        assert_eq!(&[3, 6], test.as_slice());
    }

    #[test]
    fn basic_test2() {
        let A = [2, 3, 4, 6];
        let B = [1, 3, 5, 6, 7];

        let test = intersection(&A, &B);
        assert_eq!(&[3, 6], test.as_slice());
    }

    #[test]
    fn none_over_A() {
        let A = [1, 2, 3, 4];
        let B = [9];

        let test = intersection(&A, &B);
        assert_eq!(&[0; 0], test.as_slice());
    }

    #[test]
    fn none_over_B() {
        let A = [9];
        let B = [1, 2, 3, 4];

        let test = intersection(&A, &B);
        assert_eq!(&[0; 0], test.as_slice());
    }
}
