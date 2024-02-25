use std::cell::Cell;

/// Radixsort algorithm logₙr based key generator
/// # Summary
/// `KeyProducer` can be used for producing keys that fit radixsort time complexity:
/// - ϴ(logₙr×n)
/// - ϴ(β×n)=ϴ(β×logₙr×n), where r ≤ nᵝ
///
pub struct KeyProducer {
    keys_limit: u64,

    // based on on keys_limit and tens_order
    keys_cnt: usize,

    // specifies key len in 10 magnitude (100=10^2 ⇒ 2 digits key)
    tens_order: u64,

    // holds precomputed 10s order (i.e. 10, 100, 1000, …)
    // just to not compute it each time
    digits_separator: u64,

    // provides statistical information
    // useful for linear complexity radixsort escape signalization,
    // or verifications
    true_keys_cnter: Cell<usize>,
}

impl KeyProducer {
    fn incr_keys_cnter(&self) {
        let true_keys_cnter = &self.true_keys_cnter;
        true_keys_cnter.set(true_keys_cnter.get() + 1);
    }
}

impl KeyProducer {
    pub fn keys_cnt(&self) -> usize {
        self.keys_cnt
    }

    pub fn true_keys_cnt(&self) -> usize {
        self.true_keys_cnter.get()
    }
}

impl KeyProducer {
    /// _Optimization for radixsort lexbucketsort_
    /// <br/>When computed `tens_order` is greater than `keys_limit`,
    /// _buckets_len_ can be futilely long.
    ///
    /// <br/>Ex ⸻ `keys_limit < tens_order`
    /// | `keys_limit=34`, `items_len=12` ⇒ `tens_order=100`
    /// <br/>⸺ then indexes **35–99** will never be used.
    ///
    /// <br/>On other hand `keys_limit` cannot uplift _buckets_len_
    /// <br/>Ex ⸻ `keys_limit > tens_order`
    /// | `keys_limit=1_999_000`, `items_len=9` ⇒ `tens_order=10`,
    /// <br/>⸺ then _buckets_len_ cannot uplift to `1_999_001` and it stays `10` (0–9).        
    pub fn max_key(&self) -> usize {
        std::cmp::min(self.tens_order - 1, self.keys_limit) as usize
    }

    pub fn cycl_count(&self) -> usize {
        (if self.tens_order == 10 {
            self.digits_separator.ilog10()
        } else {
            self.digits_separator.ilog(self.tens_order)
        }) as usize
    }

    /// Advances to next round of key separations.
    pub fn fin_round(&mut self) {
        *self.true_keys_cnter.get_mut() = 0;
        self.digits_separator = self.digits_separator * self.tens_order as u64;
    }

    /// `key_limit` — inclusive max expected num / supported key
    pub fn new(keys_limit: u64, items_len: usize) -> KeyProducer {
        assert_ne!(0, items_len, "Some length is expected.");
        assert_ne!(0, keys_limit, "0-key only is not supported.");

        let mut order_limit = 10; // i.e. 1–9 % 10 = 1–9

        let items_len = items_len as u64;
        while items_len / order_limit > 0 {
            // meaning 1–9 / 10 = 0, 10–99 / 10 = 1–9, 10–99 / 100 = 0, …
            order_limit *= 10;
        }

        let log_value = if order_limit == 10 {
            keys_limit.ilog10()
        } else {
            keys_limit.ilog(order_limit)
        };

        KeyProducer {
            keys_limit,
            // +1 is needed since logarithm expresses count of division using base provided until
            // #1 is resulted, for instance when
            // base = 10, argument = 100–999 ⇒ ilog = 2; 100–999 / 10 (once) / 10 (twice) = 1–9
            // obviously for numbers composed of 3 digits it is not possible by dividing
            // by 10 2 times to separate all digits
            // thus evrytime there is need for 1 more division on unit order
            keys_cnt: (log_value + 1) as usize,
            tens_order: order_limit,
            digits_separator: 1,
            true_keys_cnter: Cell::new(0),
        }
    }

    pub fn key(&self, mut num: u64) -> usize {
        let digits_separator = self.digits_separator;
        num = num / digits_separator;

        if num == 0 {
            if digits_separator == 1 {
                self.incr_keys_cnter();
            }
            // num cannot fit into order
            // ex: num = 9, digits_separator = 100, num_tranc = 0
            // note: see expactation on key, it is tuple of some [0–99]…[0–99] numbers
            // like ([(0)9], [25], [98], [45], [23]) for 925_984_523, so 9 is seen as
            // [(0)0],[(0)0],…,[(0)9]
            return 0;
        }

        self.incr_keys_cnter();
        return (num % self.tens_order) as usize;
    }
}

#[cfg(test)]
mod units_mix {
    use super::*;

    #[test]
    fn getters_test() {
        let pducer = KeyProducer {
            keys_limit: 2,
            keys_cnt: 3,
            tens_order: 4,
            digits_separator: 5,
            true_keys_cnter: Cell::new(6),
        };

        assert_eq!(pducer.keys_cnt, pducer.keys_cnt());
        assert_eq!(pducer.true_keys_cnter.get(), pducer.true_keys_cnt());
    }

    #[test]
    fn fin_round_test() {
        let mut pducer = KeyProducer {
            keys_limit: 0,
            keys_cnt: 0,
            tens_order: 10,
            digits_separator: 1,
            true_keys_cnter: Cell::new(1),
        };

        pducer.fin_round();
        assert_eq!(0, pducer.true_keys_cnter.get());
        assert_eq!(10, pducer.digits_separator);
    }

    #[test]
    fn incr_keys_cnter_test() {
        let pducer = KeyProducer {
            keys_limit: 0,
            keys_cnt: 0,
            tens_order: 0,
            digits_separator: 0,
            true_keys_cnter: Cell::new(0),
        };

        pducer.incr_keys_cnter();
        assert_eq!(1, pducer.true_keys_cnter.get());
    }

    // ⟫⟫START ⸻ max_key()⟪⟪

    #[test]
    fn max_key_coerced_to_keys_limit() {
        let pducer = KeyProducer::new(99, 10);
        assert_eq!(99, pducer.max_key());
    }

    #[test]
    fn max_key_derived_from_10s_order() {
        let pducer = KeyProducer::new(101, 10);
        assert_eq!(99, pducer.max_key());
    }

    // ⟫⟫END ⸻ max_key()⟪⟪
    // ⟫⟫START ⸻ cycl_count()⟪⟪

    #[test]
    fn cycl_count_ilog10() {
        let pducer = KeyProducer {
            keys_limit: 0,
            keys_cnt: 0,
            tens_order: 10,
            digits_separator: 1000,
            true_keys_cnter: Cell::new(0),
        };

        assert_eq!(3, pducer.cycl_count());
    }

    #[test]
    fn cycl_count_ilog() {
        let pducer = KeyProducer {
            keys_limit: 0,
            keys_cnt: 0,
            tens_order: 1000,
            digits_separator: 1_000_000,
            true_keys_cnter: Cell::new(0),
        };

        assert_eq!(2, pducer.cycl_count());
    }

    #[test]
    fn cycl_count_ilog10_digits_separator_1() {
        let pducer = KeyProducer {
            keys_limit: 0,
            keys_cnt: 0,
            tens_order: 10,
            digits_separator: 1,
            true_keys_cnter: Cell::new(0),
        };

        assert_eq!(0, pducer.cycl_count());
    }

    #[test]
    fn cycl_count_ilog_digits_separator_1() {
        let pducer = KeyProducer {
            keys_limit: 0,
            keys_cnt: 0,
            tens_order: 1000,
            digits_separator: 1,
            true_keys_cnter: Cell::new(0),
        };

        assert_eq!(0, pducer.cycl_count());
    }

    // ⟫⟫END ⸻ cycl_count()⟪⟪
}

#[cfg(test)]
mod units_new {
    use super::*;

    #[test]
    fn new_ilog10_test() {
        let pducer = KeyProducer::new(10, 1);
        assert_eq!(2, pducer.keys_cnt);
    }

    #[test]
    fn new_ilog10_test2() {
        let pducer = KeyProducer::new(99, 9);
        assert_eq!(2, pducer.keys_cnt);
    }

    #[test]
    fn new_basic() {
        let pducer = KeyProducer::new(333, 2);
        assert_eq!(333, pducer.keys_limit);
        assert_eq!(1, pducer.digits_separator);
        assert_eq!(0, pducer.true_keys_cnter.get());
    }

    // orders are promoted to next 10s order
    // i.e. 1–9 (1) ⇒ 10, 10–99 (10) ⇒ 100, …
    // thus: items_len = 1 ⇒ tens_order = 10
    //
    // keys_cnt = ⎣order-based log(max_key)⎦ + 1
    #[test]
    fn new_1_000_000_len_1() {
        let pducer = KeyProducer::new(1_000_000, 1);
        assert_eq!(7, pducer.keys_cnt);
        assert_eq!(10, pducer.tens_order);
    }

    #[test]
    fn new_1_000_000_len_9() {
        let pducer = KeyProducer::new(1_000_000, 9);
        assert_eq!(7, pducer.keys_cnt);
        assert_eq!(10, pducer.tens_order);
    }

    #[test]
    fn new_1_000_000_len_10() {
        let pducer = KeyProducer::new(1_000_000, 10);
        assert_eq!(4, pducer.keys_cnt);
        assert_eq!(100, pducer.tens_order);
    }

    #[test]
    fn new_1_000_000_len_99() {
        let pducer = KeyProducer::new(1_000_000, 99);
        assert_eq!(4, pducer.keys_cnt);
        assert_eq!(100, pducer.tens_order);
    }

    #[test]
    fn new_1_000_000_len_100() {
        let pducer = KeyProducer::new(1_000_000, 100);
        assert_eq!(3, pducer.keys_cnt);
        assert_eq!(1000, pducer.tens_order);
    }

    #[test]
    fn new_1_000_000_len_999() {
        let pducer = KeyProducer::new(1_000_000, 999);
        assert_eq!(3, pducer.keys_cnt);
        assert_eq!(1000, pducer.tens_order);
    }

    // not all numbers fits exactly into base
    // decimal part must be truncated
    #[test]
    fn new_rounding_of_key_log_9_999_len_10() {
        let pducer = KeyProducer::new(9_999, 10); // 10s "uplifted" to base-100
        assert_eq!(2, pducer.keys_cnt); // so its 1.truncation + 1
    }

    #[test]
    fn new_rounding_of_key_log_10_001_len_10() {
        let pducer = KeyProducer::new(10_001, 10); // 10s "uplifted" to base-100
        assert_eq!(3, pducer.keys_cnt); // so its 2.truncation + 1
    }

    #[test]
    fn new_rounding_of_key_log_9_999_len_9() {
        let pducer = KeyProducer::new(9_999, 9); // 1s "uplifted" to base-10
        assert_eq!(4, pducer.keys_cnt); // so its 3.truncation + 1
    }

    #[test]
    fn new_rounding_of_key_log_10_001_len_9() {
        let pducer = KeyProducer::new(10_001, 9); // 1s "uplifted" to base-10
        assert_eq!(5, pducer.keys_cnt); // so its 4.truncation + 1
    }

    #[test]
    #[should_panic(expected = "Some length is expected.")]
    fn new_0_items_len() {
        KeyProducer::new(1, 0);
    }

    #[test]
    #[should_panic(expected = "0-key only is not supported.")]
    fn new_0_keys_limit() {
        KeyProducer::new(0, 1);
    }
}

#[cfg(test)]
mod units_key {
    use super::*;

    #[test]
    fn key_proper_shortening1() {
        // len = 1 translates into 10^1 order
        let mut pducer = KeyProducer::new(1_000, 1);

        let num = 9_876;

        let expacations = [6, 7, 8, 9];

        for cycl in 1..=4 {
            let key = pducer.key(num);
            assert_eq!(expacations[cycl - 1], key);
            pducer.fin_round();
        }
    }

    #[test]
    fn key_proper_shortening2() {
        // len = 100 translates into 10^3 order
        let mut pducer = KeyProducer::new(1_000_000, 100);
        let num = 987_654_321;

        let expacations = [321, 654, 987];

        for cycl in 1..=3 {
            let key = pducer.key(num);
            assert_eq!(expacations[cycl - 1], key);
            pducer.fin_round();
        }
    }

    #[test]
    fn key_incr_keys_cnter_test() {
        let mut pducer = KeyProducer::new(9, 100);
        let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        for nu in nums {
            let key = pducer.key(nu);
            assert_eq!(nu as usize, key);
        }

        assert_eq!(10, pducer.true_keys_cnter.get());
        pducer.fin_round();

        for nu in nums {
            let key = pducer.key(nu);
            assert_eq!(0, key);
        }

        assert_eq!(0, pducer.true_keys_cnter.get());
    }
}
