#![allow(non_snake_case)]

use std::vec::Vec;

pub fn union<T>(A: &[T], B: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let result = nucleus(A, B, MergeType::Union);

    let mut i = result.i;
    let mut j = result.j;

    let A_len = result.A_len;
    let B_len = result.B_len;

    let mut output = result.output;

    while i < A_len {
        output.push(A[i]);
        i += 1;
    }

    while j < B_len {
        output.push(B[j]);
        j += 1;
    }

    output
}

pub fn intersection<T>(A: &[T], B: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    nucleus(A, B, MergeType::Intersection).output
}

#[derive(PartialEq)]
enum MergeType {
    Intersection,
    Union,
}

struct MergeResult<T> {
    i: usize,
    j: usize,
    A_len: usize,
    B_len: usize,
    output: Vec<T>,
}

fn nucleus<T>(A: &[T], B: &[T], mt: MergeType) -> MergeResult<T>
where
    T: PartialOrd + Copy,
{
    let mut i = 0;
    let mut j = 0;

    let A_len = A.len();
    let B_len = B.len();

    let mut output = Vec::new();

    let doUnion = mt == MergeType::Union;

    // TC: Ο(m+n)
    // TC: Ω(m or n)
    while j < B_len && i < A_len {
        if A[i] < B[j] {
            if doUnion {
                output.push(A[i]);
            }

            i += 1;
            continue;
        }

        if A[i] > B[j] {
            if doUnion {
                output.push(B[j]);
            }

            j += 1;
            continue;
        }

        output.push(A[i]);
        i += 1;
        j += 1;
    }

    MergeResult {
        i,
        j,
        A_len,
        B_len,
        output,
    }
}

#[cfg(test)]
mod tests_of_units {

    mod intersection_tests {
        use super::super::intersection;

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

    mod union_tests {
        use super::super::union;

        #[test]
        fn basic_test() {
            let A = [1, 3, 5, 6, 7];
            let B = [2, 3, 4, 6];

            let test = union(&A, &B);
            assert_eq!(&[1, 2, 3, 4, 5, 6, 7], test.as_slice());
        }

        #[test]
        fn basic_test2() {
            let A = [2, 3, 4, 6];
            let B = [1, 3, 5, 6, 7];

            let test = union(&A, &B);
            assert_eq!(&[1, 2, 3, 4, 5, 6, 7], test.as_slice());
        }

        #[test]
        fn all_over_A() {
            let A = [1, 2, 3, 4];
            let B = [9];

            let test = union(&A, &B);
            assert_eq!(&[1, 2, 3, 4, 9], test.as_slice());
        }

        #[test]
        fn all_over_B() {
            let A = [9];
            let B = [1, 2, 3, 4];

            let test = union(&A, &B);
            assert_eq!(&[1, 2, 3, 4, 9], test.as_slice());
        }
    }
}
