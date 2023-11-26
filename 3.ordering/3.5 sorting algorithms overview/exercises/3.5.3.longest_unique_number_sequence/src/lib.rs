#![allow(dead_code)]

use std::collections::BTreeSet;
use std::fmt::Display;

#[derive(Debug)]
struct UniquePortion {
    start_ix: usize,
    len: usize,
}
/// `seq` is sequence, i.e. ordered list of items.
fn find_longest<T>(seq: &[T]) -> Option<UniquePortion>
where
    T: Ord + Display,
{
    let seq_len = seq.len();

    if seq_len == 0 {
        return None;
    }

    if seq_len == 1 {
        return Some(UniquePortion {
            start_ix: 0,
            len: 1,
        });
    }

    Some(find_longest_focus(seq))
}

fn find_longest_focus<T>(seq: &[T]) -> UniquePortion
where
    T: Ord + Display,
{
    let mut set = BTreeSet::new();

    // portion determinants
    let mut p_ix = 0;
    let mut p_len = 0;

    // readings determinants
    let mut r_ix = 0;
    let mut r_len = 0;

    let seq_len = seq.len();

    // by extending enumeration with duplicity raising pseudoend
    // all logic can be simply placed just into loop body
    while r_ix <= seq_len {
        let r_ix_corr = r_ix - (r_ix / seq_len);
        let t = &seq[r_ix_corr];

        if set.insert(t) {
            r_len += 1;
        } else {
            if r_len > p_len {
                p_ix = r_ix - r_len;
                p_len = r_len;
            }

            // shortcut: there is no chance to beat current lenght
            // so quit immediately
            if seq_len - r_ix <= p_len {
                break;
            }

            set.clear();
            set.insert(t);
            r_len = 1;
        }

        r_ix += 1;
    }

    UniquePortion {
        start_ix: p_ix,
        len: p_len,
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::find_longest;

    // find_longest()
    #[test]
    fn short_slice() {
        let slice = [0; 0];
        let res = find_longest(&slice);
        assert!(res.is_none());

        let slice = [0; 1];
        let res = find_longest(&slice);
        assert!(res.is_some());

        let res = res.unwrap();

        assert_eq!(0, res.start_ix);
        assert_eq!(1, res.len);
    }

    use super::find_longest_focus;

    // find_longest_focus()
    #[test]
    fn takes_first_test() {
        let seq = [0, 1, 2, 2, 3, 4, 4, 5, 6];
        let res = find_longest_focus(&seq);

        assert_eq!(0, res.start_ix);
        assert_eq!(3, res.len);
    }

    #[test]
    // ensures that:
    //  ‣ possibly longest portion is not cut short
    //  ‣ longest portion at tail is registered at all
    fn shortage_test1() {
        let seq = [0, 1, 2, 2, 3, 4, 4, 5, 6, 7];
        let res = find_longest_focus(&seq);

        assert_eq!(6, res.start_ix);
        assert_eq!(4, res.len);
    }

    #[test]
    // if there is no option for longer portion,
    // nothing is broken
    fn shortage_test2() {
        let seqs: [&[usize]; 2] = [&[0, 1, 2, 2, 3, 4], &[0, 1, 2, 2, 3]];

        for s in seqs {
            let res = find_longest_focus(&s);

            assert_eq!(0, res.start_ix);
            assert_eq!(3, res.len);
        }
    }

    #[test]
    // workload test
    fn longest_seek() {
        let seq = [0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 9, 10, 10];
        let res = find_longest_focus(&seq);

        assert_eq!(7, res.start_ix);
        assert_eq!(4, res.len);
    }
}
