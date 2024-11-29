/// THIS SORT GOES WITH LEXICOGRAPHICAL SORTING
/// LARGE BUCKETS ARE DOWNSIDE OF THIS SORT

/// let assume floating point number of form bellow
/// n = m*2ᵉ
/// mantisa  = m      |     2²⁴ ≤ m ≤ 2²⁵-1
/// exponent = e      |  -128 ≤ e ≤ 127     , -(2⁷) ≤ e ≤ 2⁷-1
/// +----------------------------------------------------------------------------------------------------------+---------------+---------------------------+
/// |                                             mantissa                                                     | exponent-sign |          exponent         |
/// +----------------------------------------------------------------------------------------------------------+---------------+---------------------------+
/// |                                              24 bit                                                      |    1 bit      |            7 bit          |
/// +----------------------------------------------------------------------------------------------------------+---------------+---------------------------+
/// | layout   | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |    | 0 |      | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +----------------------------------------------------------------------------------------------------------+---------------+---------------------------+
/// | exponent | 23| 22| 21| 20| 19| 18| 17| 16| 15| 14| 13| 12| 11| 10| 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |    | 7 |      | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// +----------------------------------------------------------------------------------------------------------+---------------+---------------------------+
///
use super::consts::*;
use super::FPoint;

#[allow(dead_code)]
fn sort(fpoints: &mut [FPoint]) {
    let perms = 2usize.pow(24);
    let mut bucs = Vec::with_capacity(perms);

    let sp_cp_mut = &mut bucs.spare_capacity_mut()[..perms];

    // T: Θ(2²⁴)
    // S: Θ(2²⁴)
    for cp in sp_cp_mut {
        cp.write(Vec::<FPoint>::with_capacity(0));
    }

    unsafe {
        bucs.set_len(perms);
    }

    // T: Θ(n)
    // S: Θ(n)
    for &f in fpoints.iter() {
        let mant = f >> 8;

        // println!("mant {mant}, f {f}");

        bucs[mant as usize].push(f);
    }

    wr_output(&mut bucs, fpoints);

    // T: Θ(n)
    // S: Θ(n)
    for &f in fpoints.iter() {
        let mut exp = (f & EXP_MASK) as usize;
        if SIG_BIT_MASK & f != SIG_BIT_MASK {
            // exponent is defined using 2's complement
            exp += 128;
        }

        // println!("exp {exp}, f {f}");
        bucs[exp].push(f);
    }

    wr_output(&mut bucs, fpoints);
}

fn wr_output(bucs: &mut Vec<Vec<FPoint>>, fpoints: &mut [FPoint]) {
    let fpoints_len = fpoints.len();

    let mut wr_ix = 0;

    // T: Ο(2²⁴)
    for b in bucs {
        if b.len() == 0 {
            continue;
        }
        for &f in b.iter() {
            fpoints[wr_ix] = f;
            wr_ix += 1;

            if fpoints_len == wr_ix {
                break;
            }
        }

        b.clear();
    }
}

#[cfg(test)]
mod sort_tests {

    use super::super::auxies;
    use super::sort;

    #[test]
    fn extremes_test() {
        let max_fraction_of_min = 0b_1000_0000;
        let max_fraction_of_max = u32::MAX ^ 0b0111_1111;
        let max_of_min: u32 = 0b0111_1111;
        let total_max = u32::MAX ^ 0b1000_0000;

        // println!("{}", auxies::get(max_fraction_of_max));
        // println!("{}", auxies::get(max_fraction_of_min));
        // println!("{}", auxies::get(total_max));
        // println!("{}", auxies::get(max_of_min));

        let mut arr = [
            total_max,
            max_of_min,
            max_fraction_of_max,
            max_fraction_of_min,
        ];
        let criterion = [
            max_fraction_of_min,
            max_fraction_of_max,
            max_of_min,
            total_max,
        ];

        sort(&mut arr);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn load_test() {
        let min_mant_min_exp = 0 | 0b_1000_0000;
        let a = 0b_0101_0101_0101_0101_0101_0101___1010_1010;
        let b = 0b_1010_1010_1010_1010_1010_1010___1010_1010;

        let one_half = 0 | 0b_1110_0111;
        let one = 0 | 0b_1110_1000;
        let two = 0 | 0b_1110_1001;

        let min_mant_zer_exp = 0;
        let max_mant_zer_exp = u32::MAX ^ 0b_1111_1111;

        let c: u32 = 0b_0101_0101_0101_0101_0101_0101___0101_0101;
        let d: u32 = 0b_1010_1010_1010_1010_1010_1010___0101_0101;
        let min_mant_max_exp = 0 | 0b_0111_1111;

        let mut arr = [
            min_mant_max_exp,
            d,
            c,
            max_mant_zer_exp,
            min_mant_zer_exp,
            two,
            one,
            one_half,
            b,
            a,
            min_mant_min_exp,
        ];

        let mut criterion = arr.clone().map(|x| (auxies::get(x), x));
        criterion.sort_by(|a, b| a.0.total_cmp(&b.0));
        let criterion = criterion.map(|x| x.1);

        sort(&mut arr);
        assert_eq!(criterion, arr);
    }
}
