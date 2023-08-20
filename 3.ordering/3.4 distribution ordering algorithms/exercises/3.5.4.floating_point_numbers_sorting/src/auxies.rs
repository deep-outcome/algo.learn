#![allow(dead_code)]

use super::consts::*;
use super::sort::FPoint;

pub fn get_mant(f: FPoint) -> u32 {
    let mant = (f >> 8) << 1;
    mant
}

pub fn get_exp(f: FPoint) -> i32 {
    let exp = f & EXP_MASK;

    let mut exp = exp as i32;

    if f & SIG_BIT_MASK == SIG_BIT_MASK {
        exp -= 128;
    }

    exp
}

pub fn get(f: FPoint) -> f64 {
    let exp = get_exp(f);
    let mant = get_mant(f);

    2f64.powi(exp) * mant as f64
}

#[cfg(test)]
mod tests_of_units {
    use super::*;

    #[test]
    fn get_test1() {
        let max_fraction = u32::MAX ^ 0b0111_1111;

        let get = get(max_fraction);
        let criterion = (2f64.powi(25) - 2f64) * 2f64.powi(-128);

        assert_eq!(criterion, get);
    }

    #[test]
    fn get_test2() {
        let max: u32 = u32::MAX ^ 0b1000_0000;

        let get = get(max);
        let criterion = (2f64.powi(25) - 2f64) * 2f64.powi(127);

        assert_eq!(criterion, get);
    }

    #[test]
    fn get_mant_test() {
        let test: u32 = u32::MAX;

        let test = get_mant(test);
        assert_eq!(2u32.pow(25) - 2, test);
    }

    #[test]
    fn get_exp_test1() {
        let test: u32 = u32::MAX;

        let test = get_exp(test);
        assert_eq!(-1, test);
    }

    #[test]
    fn get_exp_test2() {
        let test: u32 = 0b_0111_1111;

        let test = get_exp(test);
        assert_eq!(127, test);
    }
}
