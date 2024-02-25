#![allow(dead_code)]

struct Hole {
    a: usize,
    b: usize,
}

/// while `BTreeSet` (or other "simple" set type) would perfectly fit
/// as input, let use slice for simplicity
/// also maintaing set type for collection would be against
/// idea of cheap gap search
fn find_hole(set: &[usize]) -> Option<Hole> {
    // let support 0–100 for simplicity
    let mut pseudo_bucs = [0usize; 101];

    for i in set {
        pseudo_bucs[*i] = 1;
    }

    // since having set of min 2 elems, min gap is 1
    // so `0` init val is okay
    let mut gap = 0;
    // pivot
    let mut pt = 0;

    let mut last;
    let mut ix = 0;

    loop {
        if pseudo_bucs[ix] == 1 {
            last = ix;
            break;
        }

        ix += 1;
    }

    let pseudo_bucs_len = pseudo_bucs.len();

    ix += 1;
    while ix < pseudo_bucs_len {
        if pseudo_bucs[ix] == 1 {
            let diff = ix - last;
            last = ix;

            if diff > gap {
                gap = diff;
                pt = ix;
            }
        }

        ix += 1;
    }

    Some(Hole { a: pt - gap, b: pt })
}

#[cfg(test)]
mod tests_of_units {
    use super::find_hole;

    #[test]
    fn basic_test() {
        let set = [4, 2, 1];
        let hole = find_hole(&set);

        assert!(hole.is_some());
        let hole = hole.unwrap();
        assert_eq!(2, hole.a);
        assert_eq!(4, hole.b);
    }

    #[test]
    fn no_default_start_at_zero() {
        let set = [7, 8];
        let hole = find_hole(&set);

        assert!(hole.is_some());
        let hole = hole.unwrap();
        assert_eq!(7, hole.a);
        assert_eq!(8, hole.b);
    }

    #[test]
    fn first_gap_taken() {
        let set = [7, 5, 4, 2, 1];
        let hole = find_hole(&set);

        assert!(hole.is_some());
        let hole = hole.unwrap();
        assert_eq!(2, hole.a);
        assert_eq!(4, hole.b);
    }

    #[test]
    fn min_gap_taken() {
        let set = [2, 1];
        let hole = find_hole(&set);

        assert!(hole.is_some());
        let hole = hole.unwrap();
        assert_eq!(1, hole.a);
        assert_eq!(2, hole.b);
    }

    #[test]
    fn max_gap() {
        let set = [100, 0];
        let hole = find_hole(&set);

        assert!(hole.is_some());
        let hole = hole.unwrap();
        assert_eq!(0, hole.a);
        assert_eq!(100, hole.b);
    }

    #[test]
    fn incrementing_diff() {
        let set = [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66];
        let hole = find_hole(&set);

        assert!(hole.is_some());
        let hole = hole.unwrap();
        assert_eq!(55, hole.a);
        assert_eq!(66, hole.b);
    }

    #[test]
    fn correct_ranging() {
        let arrs: [[usize; 2]; 2] = [[100, 99], [1, 0]];

        for a in arrs {
            let hole = find_hole(&a);

            assert!(hole.is_some());
            let hole = hole.unwrap();
            assert_eq!(a[1], hole.a);
            assert_eq!(a[0], hole.b);
        }
    }
}
