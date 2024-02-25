/// idea behind this in-place sort labors with
/// divisibility rules — greater num cannot divide smaller num
/// so having: a,b,c where c>b ∧ c>a
/// then when: c*b+a=x ⇒ x%c=a ∧ ⌊x/c⌋=b
///
///
/// c must by greater than both (a,b) since:
/// 1) a=b
/// example
/// +---+---+
/// | 4 | 4 |
/// +---+---+
/// after 1ˢᵗ iteration
/// +----+---+
/// | 20 | 4 |
/// +----+---+
/// after right portion exhaustion
/// +----+----+
/// | 20 | 20 |
/// +----+----+
/// after current nums computation
/// +---+---+
/// | 5 | 5 |
/// +---+---+
///
/// 2) 0 should swap with max
/// example
/// +---+---+
/// | 4 | 0 |
/// +---+---+
/// after 1ˢᵗ iteration
/// +---+---+
/// | 4 | 0 |
/// +---+---+
/// after left portion exhaustion
/// +---+----+
/// | 4 | 16 |
/// +---+----+
/// after current nums computation
/// +---+---+
/// | 1 | 4 |
/// +---+---+
///
///
/// Ο(2) space complexity
/// Ο(2*n) ∊ Ο(n) time complexity, at 1ˢᵗ glanc c (aux operations) is not small
#[allow(dead_code)]
pub fn sort(arr: &mut [usize], left: usize, right: usize, ex_end: usize) {
    let mulcator = arr[right - 1].max(arr[ex_end - 1]) + 1;

    let mut lef_ix = left;
    let mut rig_ix = right;
    let mut wri_ix = left;

    while lef_ix < right && rig_ix < ex_end {
        // modulo operation "retrieves" original num
        // when it was already overwritten
        // by proceeding wri_ix
        //
        // lef_ix=0, rig_ix=2, wri_ix=0, mulcator=6
        // +---+---+---+---+
        // | 3 | 5 | 1 | 4 |
        // +---+---+---+---+
        //
        // after 1ˢᵗ iteration
        // +---+---+---+---+
        // | 9 | 5 | 1 | 4 |
        // +---+---+---+---+
        // lef_ix=0, rig_ix=3, wri_ix=1 | --> 9%6=3, 3<4
        //
        // after 2ⁿᵈ iteration
        // +---+----+---+---+
        // | 9 | 23 | 1 | 4 |
        // +---+----+---+---+
        let lef_num = arr[lef_ix] % mulcator;

        // ▪ it is not possible to read overwritten nums on right portion
        // ▪ important is that there is starting gap between wri_ix and rig_ix
        // ▪ as wri_ix grows with lef_ix increments, gap shortens but this can
        // result only in wri_ix equal rig_ix at maximum (when only left taken)
        // ▪ when wri_ix grows with rig_ix increments, gap maintains
        // ▪ thus there is no need for modulo "retriveval"
        let rig_num = arr[rig_ix];

        // = maintains stability
        if lef_num <= rig_num {
            arr[wri_ix] += lef_num * mulcator;
            wri_ix += 1;
            lef_ix += 1;
        } else {
            arr[wri_ix] += rig_num * mulcator;
            wri_ix += 1;
            rig_ix += 1;
        }
    }

    // exhaust left portion if needed
    while lef_ix < right {
        arr[wri_ix] += (arr[lef_ix] % mulcator) * mulcator;
        wri_ix += 1;
        lef_ix += 1;
    }

    // exhaust right portion if needed
    // as proved in core cyclus, when left portion is exhausted
    // wri_ix == rig_ix
    while rig_ix < ex_end {
        // again no modulo
        arr[rig_ix] *= mulcator;
        rig_ix += 1;
    }

    // "extract" current/actual nums
    wri_ix = left;
    while wri_ix < ex_end {
        arr[wri_ix] /= mulcator;
        wri_ix = wri_ix + 1;
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn basic_test() {
        let mut arr = [3, 4, 1, 2];
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, 2, 4);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn basic_test2() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, 3, 6);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn basic_test3() {
        let mut arr = [1, 3, 5, 7, 2, 4, 6, 8];
        let mut criterion = arr.clone();
        criterion.sort();

        sort(&mut arr, 0, 4, 8);
        assert_eq!(criterion, arr);
    }
}
