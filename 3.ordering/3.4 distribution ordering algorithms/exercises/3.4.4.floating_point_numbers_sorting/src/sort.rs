/// let assume floating point number of form bellow
/// n = m*2ᵉ
/// mantisa  = m      |     2²⁴ ≤ m ≤ 2²⁵-1
/// exponent = e      |  -128 ≤ e ≤ 127     , -(2⁷) ≤ e ≤ 2⁷-1
/// this sort will work for any similar f-point num assuming: bit power ascending from right, exponent being defined using 2's complement,
/// i.e. magnitude of particular bit pow is not decisive, only relation to other bit pows
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
// use super::auxies::*;
use std::rc::Rc;

pub type FPoint = u32;

#[derive(Clone)]
// S: Θ(usize+usize+24*u8+u32) => Ο(44 bytes)
struct FPointKey {
    polynom: Rc<Box<[u8; 24]>>,
    val: FPoint,
}

impl FPointKey {
    fn new(f: FPoint) -> Self {
        Self {
            val: f,
            polynom: gen_poly(f),
        }
    }
}

use core::fmt::{Debug, Error, Formatter};
impl Debug for FPointKey {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{:?}, {}", self.polynom, self.val))
    }
}

impl PartialEq for FPointKey {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Unimplemented");
    }
}

impl PartialOrd for FPointKey {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        panic!("Unimplemented");
    }

    fn lt(&self, other: &Self) -> bool {
        let l_p = &self.polynom;
        let r_p = &other.polynom;

        // T: Θ(24)
        for i in (0..24).rev() {
            if l_p[i] < r_p[i] {
                return true;
            }
        }

        return false;
    }

    fn le(&self, _other: &Self) -> bool {
        panic!("Unimplemented");
    }

    fn gt(&self, _other: &Self) -> bool {
        panic!("Unimplemented");
    }
    fn ge(&self, _other: &Self) -> bool {
        panic!("Unimplemented");
    }
}

/// sorting is based on math:
/// 1️⃣ power summation (see gen_poly)
/// e.g. 2¹*2² = 2³
/// 2️⃣ polynonom multiplication (see lt)
/// e.g. 12*16 = (2³+2²)*16 = 2³*2⁴ + 2²*2⁴ = 192
#[allow(dead_code)]
fn sort(fpoints: &mut [FPoint]) {
    let fpoints_len = fpoints.len();

    // S: Θ(n)
    let mut keys = Vec::<FPointKey>::with_capacity(fpoints_len);
    let k_spa_cap = keys.spare_capacity_mut();

    let mut wr_ix = 0;
    // T: Θ(n)
    for &f in fpoints.iter() {
        k_spa_cap[wr_ix].write(FPointKey::new(f));
        wr_ix += 1;
    }

    unsafe { keys.set_len(fpoints_len) }

    // println!("\n{:?}\n", keys);
    // println!("{:?}", fpoints);

    // insertion sort
    // T: Ο(n*log n)
    for r_ix in 1..fpoints_len {
        let r = &keys[r_ix].clone();

        let mut l_ix = r_ix - 1;
        loop {
            let l = &keys[l_ix];
            if r < l {
                keys[l_ix + 1] = l.clone();
                keys[l_ix] = r.clone();
            } else {
                break;
            }

            if l_ix == 0 {
                break;
            }

            l_ix -= 1;
        }
    }

    wr_ix = 0;
    for k in keys {
        fpoints[wr_ix] = k.val;
        wr_ix += 1;
    }
}

use super::consts::*;
fn gen_poly(f: FPoint) -> Rc<Box<[u8; 24]>> {
    let mut exp = (f & EXP_MASK) as u8;

    if SIG_BIT_MASK & f != SIG_BIT_MASK {
        // exponent is defined using 2's complement
        exp += 128;
    }

    let mut polynom = [0; 24];
    // print!("{}, {}, {}, ", get(f), get_exp(f), get_mant(f));

    let mant = f >> 8;

    // T: Θ(24)
    for bit_ix in 0..24 {
        let mant_mask = 1 << bit_ix;

        if mant_mask & mant == mant_mask {
            // there is possible to go with:
            // • exact polynom member value, i.e `pow = bit_ix + exp` ⇒ (u16)            
            // • ommit mantissa power completely since relation is provided by order, i.e. `pow = exp` ⇒ (u8)
            
            polynom[bit_ix as usize] = exp;
        }
    }

    // having input of n 100_000 magnitude
    // keys would consume at least ≈ 4.2 MB
    // let avoid stack overflow by default by
    // heap allocation
    Rc::new(Box::new(polynom))
}

#[cfg(test)]
mod get_poly_tests {
    use super::gen_poly;

    #[test]
    fn gen_poly_test1() {
        let f: u32 = 0b_1111_1111_1111_1111_1111_1111___0111_1111;

        let poly = gen_poly(f);

        let criterion = (0..24).map(|_| 127 + 128).collect::<Vec<u8>>();
        assert_eq!(criterion.leak(), &**poly);
    }

    #[test]
    fn gen_poly_test2() {
        let f: u32 = 0b_1111_1111_1111_1111_1111_1111___1111_1111;

        let poly = gen_poly(f);

        let criterion = (0..24).map(|_| 127).collect::<Vec<u8>>();
        assert_eq!(criterion.leak(), &**poly);
    }

    #[test]
    fn gen_poly_test3() {
        let f: u32 = 0b_1000_0000_0000_0000_0000_0001___0111_1111;

        let poly = gen_poly(f);

        let criterion = (1..=24)
            .map(|x| {
                if x != 1 && x != 24 {
                    return 0;
                }
                return 127 + 128;
            })
            .collect::<Vec<u8>>();
        assert_eq!(criterion.leak(), &**poly);
    }

    #[test]
    fn gen_poly_test4() {
        let f: u32 = 0b_1000_0000_0000_0000_0000_0001___1111_1111;

        let poly = gen_poly(f);

        let criterion = (1..=24)
            .map(|x| {
                if x != 1 && x != 24 {
                    return 0;
                }
                return 127;
            })
            .collect::<Vec<u8>>();
        assert_eq!(criterion.leak(), &**poly);
    }

    #[test]
    fn gen_poly_load_test1() {
        let f: u32 = 0b_1010_1010_1010_1010_1010_1010___1010_1010;

        let poly = gen_poly(f);

        let criterion = (1..=24)
            .map(|x| {
                if x % 2 == 0 {
                    return 42;
                }
                return 0;
            })
            .collect::<Vec<u8>>();
        assert_eq!(criterion.leak(), &**poly);
    }

    #[test]
    fn gen_poly_load_test2() {
        let f: u32 = 0b_0101_0101_0101_0101_0101_0101___0101_0101;

        let poly = gen_poly(f);

        let criterion = (1..=24)
            .map(|x| {
                if x % 2 == 0 {
                    return 0;
                }
                return 85 + 128;
            })
            .collect::<Vec<u8>>();
        assert_eq!(criterion.leak(), &**poly);
    }
}

#[cfg(test)]
mod lt_tests {
    use super::FPointKey;

    #[test]
    fn test1() {
        let fpk1 = FPointKey::new(0b_1111_1111_1111_1111_1111_1111___0111_1111);
        let fpk2 = FPointKey::new(0b_1111_1111_1111_1111_1111_1110___0111_1111);

        assert!(fpk2 < fpk1);
    }

    #[test]
    fn test2() {
        let fpk1 = FPointKey::new(0b_1111_1111_1111_1111_1111_1111___0111_1111);
        let fpk2 = FPointKey::new(0b_0111_1111_1111_1111_1111_1111___0111_1111);

        assert!(fpk2 < fpk1);
    }

    #[test]
    fn test3() {
        let fpk1 = FPointKey::new(0b_1111_1111_1111_1111_1111_1111___0111_1111);
        let fpk2 = FPointKey::new(0b_1111_1111_1111_1111_1111_1111___0111_1111);

        assert!(!(fpk2 < fpk1));
        assert!(!(fpk1 < fpk2));
    }
}

#[cfg(test)]
mod sort_tests {

    use super::sort;

    #[test]
    fn sort_basic_test() {
        let max_fraction = u32::MAX ^ 0b0111_1111;
        let max: u32 = u32::MAX ^ 0b1000_0000;

        let mut arr = [max, max_fraction];
        let criterion = [max_fraction, max];

        sort(&mut arr);
        assert_eq!(criterion, arr);
    }

    #[test]
    fn load_test() {
        let a: u32 = 0b_0101_0101_0101_0101_0101_0101___1010_1010;
        let b: u32 = 0b_1010_1010_1010_1010_1010_1010___1010_1010;

        let c: u32 = 0b_0101_0101_0101_0101_0101_0101___0101_0101;
        let d: u32 = 0b_1010_1010_1010_1010_1010_1010___0101_0101;

        let mut arr = [d, c, b, a];
        let criterion = [a, b, c, d];

        sort(&mut arr);
        assert_eq!(criterion, arr);
    }
}
