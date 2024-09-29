#[cfg_attr(test, derive(PartialEq))]
struct Letter<'a> {
    #[cfg(test)]
    value: char,
    alphabet: Option<Alphabet<'a>>,
    entry: Option<Entry<'a>>,
}

#[cfg_attr(test, derive(PartialEq, Clone, Debug))]
pub struct Entry<'a> {
    value: &'a str,
    frequ: usize,
}

impl<'a> Letter<'a> {
    fn new() -> Self {
        Letter {
            #[cfg(test)]
            value: '🫀',
            alphabet: None,
            entry: None,
        }
    }
}

impl<'a> Entry<'a> {
    pub fn value(&self) -> &'a str {
        self.value
    }

    pub fn frequency(&self) -> usize {
        self.frequ
    }

    fn new(v: &'a str, freq: usize) -> Self {
        Entry { value: v, frequ: freq }
    }
}

type Alphabet<'a> = Box<[Letter<'a>]>;
type FrequencyDictionary<'a> = std::vec::Vec<Entry<'a>>;

const BASE_ALPHABET_LEN: usize = 26;
const ALPHABET_LEN: usize = BASE_ALPHABET_LEN * 2;

fn alphabet<'a>() -> Alphabet<'a> {
    let mut ab = Vec::with_capacity(ALPHABET_LEN);

    #[cfg(test)]
    let mut c = 'A' as u8;

    for sc in ab.spare_capacity_mut() {
        let mut _letter = sc.write(Letter::new());

        #[cfg(test)]
        {
            _letter.value = c as char;

            if c == 'Z' as u8 {
                c = 'a' as u8;
            } else {
                c = c + 1;
            }
        }
    }

    unsafe { ab.set_len(ALPHABET_LEN) };

    ab.into_boxed_slice()
}

fn ix(c: char) -> usize {
    let code_point = c as usize;

    match code_point {
        | c if c > 64 && c < 91 => c - 'A' as usize,
        | c if c > 96 && c < 123 => c - 'a' as usize + BASE_ALPHABET_LEN,
        | _ => panic!("Unsupported char. Cannot convert to index."),
    }
}

fn exc<'a, 'b>(ab: &'b mut Alphabet<'a>, fd: &mut FrequencyDictionary<'a>) {
    for letter in ab.iter_mut() {
        if let Some(e) = letter.entry.take() {
            fd.push(e);
        }

        if let Some(alphabet) = letter.alphabet.as_mut() {
            exc(alphabet, fd);
            letter.alphabet = None;
        }
    }
}

fn ins<'a>(mut ab: &mut Alphabet<'a>, entry: &'a str) {
    let entry_len = entry.len();

    if entry_len == 0 {
        return;
    }

    let last_letter_ix = entry_len - 1;

    let mut erator = entry.chars().enumerate();

    loop {
        let (it_ix, c) = erator.next().unwrap();

        let c_ix = ix(c);

        let letter = &mut ab[c_ix];

        if it_ix == last_letter_ix {
            let entry = letter.entry.get_or_insert_with(|| Entry::new(entry, 0));
            entry.frequ += 1;
            break;
        } else {
            ab = letter.alphabet.get_or_insert_with(|| crate::alphabet())
        }
    }
}

use std::cmp::Ordering;
fn cmp(l: &Entry, r: &Entry) -> Ordering {
    match l.frequ.cmp(&r.frequ) {
        | Ordering::Equal => match l.value.cmp(r.value) {
            | Ordering::Equal => panic!("All words should match one entry."),
            | x => x,
        },
        | Ordering::Less => Ordering::Greater,
        | Ordering::Greater => Ordering::Less,
    }
}

pub struct FrequencyCounter<'a> {
    root: Alphabet<'a>,
}

impl<'a> FrequencyCounter<'a> {
    pub fn new() -> Self {
        Self { root: crate::alphabet() }
    }

    /// Suports only A-Za-z `char`s.
    ///
    /// If condition was not upheld, method would unluckily panic.    
    pub fn count(&'a mut self, strs: &mut [&'a str]) -> FrequencyDictionary {
        let root = &mut self.root;

        for s in strs.iter() {
            ins(root, s);
        }

        let mut fd = FrequencyDictionary::<'a>::with_capacity(strs.len());
        exc(root, &mut fd);

        fd.sort_unstable_by(cmp);
        fd
    }
}

#[cfg(test)]
mod tests_of_units {

    use crate::Letter;
    use std::fmt::{Debug, Formatter};
    impl<'a> Debug for Letter<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let alphabet = some_none(self.alphabet.as_ref());
            let entry = some_none(self.entry.as_ref());

            return f.write_fmt(format_args!(
                "Letter {{\n  value: {:?}\n  alphabet: {}\n  entry: {}\n}}",
                self.value, alphabet, entry
            ));

            fn some_none<T>(val: Option<T>) -> &'static str {
                if val.is_some() {
                    "Some"
                } else {
                    "None"
                }
            }
        }
    }

    #[test]
    fn new() {
        let letter = Letter::new();

        assert_eq!('🫀', letter.value);
        assert!(letter.alphabet.is_none());
        assert!(letter.entry.is_none());
    }

    use crate::Entry;

    #[test]
    fn all_methods() {
        let s = "SpreadOfTheSpan";
        let f = 321;

        let entry = Entry::new(s, f);
        assert_eq!(s, entry.value);
        assert_eq!(f, entry.frequ);
        assert_eq!(s, entry.value());
        assert_eq!(f, entry.frequency());
    }

    use super::alphabet as alphabet_fn;

    #[test]
    fn alphabet() {
        let ab = alphabet_fn();
        assert_eq!(crate::ALPHABET_LEN, ab.len());

        let chain = ('A'..='Z').chain('a'..='z');

        for (ix, c) in chain.enumerate() {
            let letter = &ab[ix];

            assert_eq!(c, letter.value);
            assert!(letter.alphabet.is_none());
            assert!(letter.entry.is_none());
        }
    }

    mod ix {
        use crate::ix;
        use std::panic::catch_unwind;

        #[test]
        fn ixes() {
            assert_eq!(0, ix('A'));
            assert_eq!(25, ix('Z'));
            assert_eq!(26, ix('a'));
            assert_eq!(51, ix('z'));
        }

        #[test]
        fn unsupported_char() {
            let ucs = unsupported_chars();

            for c in ucs {
                let result = catch_unwind(|| ix(c));
                assert!(result.is_err());

                let err = result.err().unwrap();
                let downcast = err.downcast_ref::<&str>().unwrap();
                assert_eq!(&"Unsupported char. Cannot convert to index.", downcast);
            }
        }

        fn unsupported_chars() -> [char; 4] {
            #[rustfmt::skip] let ucs =
            [
                'A' as u8 -1, 'Z' as u8 +1, // 65–90
                'a' as u8 -1, 'z' as u8 +1, // 97–122
            ];

            ucs.map(|x| x as char)
        }
    }

    mod exc {
        use crate::{alphabet, exc, ix, Alphabet, Letter, Entry, FrequencyDictionary};

        #[test]
        fn basic_test() {
            let mut ab = alphabet();

            #[allow(non_snake_case)]
            let A = Entry::new("A", 3);
            let z = Entry::new("z", 3);

            ab[ix('A')].entry = Some(A.clone());
            ab[ix('z')].entry = Some(z.clone());

            let mut fd = FrequencyDictionary::with_capacity(2);
            exc(&mut ab, &mut fd);

            assert_eq!(2, fd.len());

            assert_eq!(A, fd[0]);
            assert_eq!(z, fd[1]);

            for l in ab.iter() {
                assert!(l.entry.is_none());
            }
        }

        #[test]
        fn nesting() {
            let mut root = alphabet();
            let root = &mut root;

            const ORDERED_LEN: usize = 8;

            #[rustfmt::skip]
            let ordered: [(&str, usize); ORDERED_LEN] = [
                ("A", 7), ("z", 8),
                ("B", 3), ("y", 4),
                ("y", 1), ("B", 9),
                ("z", 5), ("A", 555)
            ];

            prep(root, ordered[0], ordered[1]);
            prep(root, ordered[2], ordered[3]);
            prep(root, ordered[4], ordered[5]);
            prep(root, ordered[6], ordered[7]);

            let mut fd = FrequencyDictionary::with_capacity(ORDERED_LEN);
            exc(root, &mut fd);

            assert_eq!(ORDERED_LEN, fd.len());

            for (ix, ex) in ordered.iter().enumerate() {
                let entry = &fd[ix];
                assert_eq!(ex.0, entry.value);
                assert_eq!(ex.1, entry.frequ);
            }

            for l in root.iter() {
                assert!(l.entry.is_none());
                assert!(l.alphabet.is_none());
            }

            fn prep<'a, 'b>(
                ab: &'b mut Alphabet<'a>,
                ent: (&'a str, usize),
                sub_ent: (&'a str, usize),
            ) {
                let c = ent.0.chars().next().unwrap();
                let sub_c = sub_ent.0.chars().next().unwrap();

                let l = &mut ab[ix(c)];

                let mut l_alphabet = alphabet();
                let sub_l = &mut l_alphabet[ix(sub_c)];
                sub_l.entry = Some(Entry::new(sub_ent.0, sub_ent.1));

                l.alphabet = Some(l_alphabet);
                l.entry = Some(Entry::new(ent.0, ent.1));
            }
        }

        #[test]
        #[allow(non_snake_case)]
        fn in_depth_recursion() {
            let mut root = alphabet();
            let root = &mut root;

            const ORDERED_LEN: usize = 12;

            // depth level described by numeric suffix
            // level: 10^(level-1) = suffix < 10^level
            // ones for level 1, tens level 2, …
            #[rustfmt::skip]
            let ordered: [(&str, usize); ORDERED_LEN] = [
                ("A1", 16), ("A10", 35), ("z10", 99), ("B100", 11), ("q1000", 44),
                ("B1", 22), ("y10", 71),
                ("y1", 13), ("B10", 29), ("C100", 39), ("X100", 25), ("r1000", 11),
            ];

            let A = add_ents_to_ab(root, ordered[0]);
            _ = add_ents_to_le(A, ordered[1]);
            let Az = add_ents_to_le(A, ordered[2]);
            let AzB = add_ents_to_le(Az, ordered[3]);
            _ = add_ents_to_le(AzB, ordered[4]);

            let B = add_ents_to_ab(root, ordered[5]);
            _ = add_ents_to_le(B, ordered[6]);

            let y = add_ents_to_ab(root, ordered[7]);
            let yB = add_ents_to_le(y, ordered[8]);
            _ = add_ents_to_le(yB, ordered[9]);
            let yBX = add_ents_to_le(yB, ordered[10]);
            _ = add_ents_to_le(yBX, ordered[11]);

            let mut fd = FrequencyDictionary::with_capacity(ORDERED_LEN);
            exc(root, &mut fd);

            assert_eq!(ORDERED_LEN, fd.len());

            for (ix, ex) in ordered.iter().enumerate() {
                let entry = &fd[ix];
                assert_eq!(ex.0, entry.value);
                assert_eq!(ex.1, entry.frequ);
            }

            for l in root.iter() {
                assert!(l.entry.is_none());
                assert!(l.alphabet.is_none());
            }

            fn add_ents_to_le<'a, 'b>(
                le: &'b mut Letter<'a>,
                ent: (&'a str, usize),
            ) -> &'b mut Letter<'a> {
                let ab = le.alphabet.get_or_insert_with(|| alphabet());
                add_ents_to_ab(ab, ent)
            }

            fn add_ents_to_ab<'a, 'b>(
                ab: &'b mut Alphabet<'a>,
                ent: (&'a str, usize),
            ) -> &'b mut Letter<'a> {
                let c = ent.0.chars().next().unwrap();
                let l = &mut ab[ix(c)];

                l.entry = Some(Entry::new(ent.0, ent.1));
                l
            }
        }
    }

    mod ins {
        use crate::{ins, alphabet, ix};

        #[test]
        fn new_path() {
            let mut ab = alphabet();
            let entry = "impreciseness";

            ins(&mut ab, &entry);

            let chars: Vec<char> = entry.chars().collect();
            let len = chars.len();
            let last_ix = len - 1;

            let mut alphabet = &ab;
            for c_ix in 0..len {
                let c = chars[c_ix];
                let l = &alphabet[ix(c)];

                let non_terminal_it = c_ix != last_ix;

                let ab = l.alphabet.as_ref();
                assert_eq!(
                    non_terminal_it,
                    ab.is_some(),
                    "{c_ix}, {c}, {non_terminal_it}",
                );

                if non_terminal_it {
                    alphabet = ab.unwrap();
                } else {
                    let e = l.entry.as_ref();
                    assert!(e.is_some());
                    let e = e.unwrap();
                    assert_eq!(entry, e.value);
                    assert_eq!(1, e.frequ);

                    assert!(l.alphabet.is_none());
                }
            }
        }

        #[test]
        fn double_insert() {
            let mut ab = alphabet();
            let entry = "impreciseness";

            ins(&mut ab, entry);
            ins(&mut ab, entry);

            let chars: Vec<char> = entry.chars().collect();
            let len = chars.len();
            let last_ix = len - 1;

            let mut alphabet = &ab;
            for c_ix in 0..len {
                let c = chars[c_ix];
                let l = &alphabet[ix(c)];

                let non_terminal_it = c_ix != last_ix;

                let ab = l.alphabet.as_ref();
                assert_eq!(
                    non_terminal_it,
                    ab.is_some(),
                    "{c_ix}, {c}, {non_terminal_it}"
                );

                if non_terminal_it {
                    alphabet = ab.unwrap();
                } else {
                    let e = l.entry.as_ref();
                    assert!(e.is_some());
                    let e = e.unwrap();
                    assert_eq!(entry, e.value);
                    assert_eq!(2, e.frequ);
                }
            }
        }

        #[test]
        fn empty_str() {
            let emt = "";
            let mut ab = alphabet();

            ins(&mut ab, emt);

            for l in ab.iter() {
                assert!(l.entry.is_none());
                assert!(l.alphabet.is_none());
            }
        }
    }

    mod cmp {
        use crate::{cmp, Entry};
        use std::cmp::Ordering;

        #[test]
        fn equal_frequ() {
            let e1 = Entry { value: "A", frequ: 3 };

            let e2 = Entry { value: "B", frequ: 3 };

            assert_eq!(Ordering::Less, cmp(&e1, &e2));
            assert_eq!(Ordering::Greater, cmp(&e2, &e1));
        }

        #[test]
        fn diff_frequ() {
            let e1 = Entry { value: "A", frequ: 1 };

            let e2 = Entry { value: "B", frequ: 2 };

            assert_eq!(Ordering::Greater, cmp(&e1, &e2));
            assert_eq!(Ordering::Less, cmp(&e2, &e1));
        }

        #[test]
        #[should_panic(expected = "All words should match one entry.")]
        fn equal_frequ_equal_value() {
            let e1 = Entry { value: "A", frequ: 2 };

            let e2 = Entry { value: "A", frequ: 2 };

            cmp(&e1, &e2);
        }
    }

    mod frequency_counter {
        use crate::{FrequencyCounter, alphabet};

        #[test]
        fn new() {
            let counter = FrequencyCounter::new();
            assert_eq!(alphabet(), counter.root);
        }

        mod count {
            use crate::FrequencyCounter;

            #[test]
            #[allow(non_snake_case)]
            fn basic_test() {
                let ABC = "ABC";
                let zyx = "zyx";

                let mut strs = [zyx, zyx, ABC, ABC];

                let mut counter = FrequencyCounter::new();
                let fd = counter.count(&mut strs);

                assert_eq!(2, fd.len());

                for (ix, s) in [ABC, zyx].iter().enumerate() {
                    let entry = &fd[ix];
                    assert_eq!(2, entry.frequ);
                    assert_eq!(*s, entry.value);
                }
            }

            #[test]
            #[allow(non_snake_case)]
            #[rustfmt::skip]
            fn load() {
                
                let ABC = "ABC"; let ABD = "ABD";                  
                let INSTANT = "INSTANT"; let JUNCTURE = "JUNCTURE"; let MOMENT = "MOMENT"; 
                
                let JUI = "JUI"; let JUN = "JUN"; 
                let XYA = "XYA"; let XYQ = "XYQ"; let XYZ = "XYZ"; 
                
                let XYAB = "XYAB";                
                let XYABA = "XYABA"; let XYABC = "XYABC";
                
                let abc = "abc"; let abd = "abd";                
                let abcd = "abcd"; let abce = "abce";
                
                let qaa = "qaa"; 
                let qrs = "qrs"; let qrt = "qrt";                
                let qua = "qua"; let qwa = "qwa"; 
                
                let xya = "xya"; let xyz = "xyz"; 
                let zzaa = "zzaa"; let zzbb = "zzbb";                
                
                let percent = "percent"; let percentile = "percentile";
                
                let quail = "quail";
                let qualification = "qualification";
                let quality = "quality";
                let quantity = "quantity";
                
                let emt1 = ""; let emt2 = ""; let emt3 = "";
                
                const STRS_LEN: usize = 46;
                let mut strs: [&str; STRS_LEN] = [
                    XYZ, XYZ, XYZ, XYA, XYQ,
                    xyz, xya,
                    emt1,
                    zzbb, zzaa, zzaa, zzaa,
                    emt2,
                    ABC, ABD,
                    abce, abcd,
                    emt3, emt3, emt3,
                    abd, abc,
                    qwa, qua, qua, qaa, qaa,
                    XYABC, XYABA, XYAB,
                    qrt, qrs,
                    percentile, percent, percent, percent, percent,
                    quantity, quality, qualification, quail,
                    JUNCTURE, MOMENT, INSTANT,
                    JUI, JUN
                ];
                
                
                let proof = [
                    (percent, 4), (XYZ, 3), (zzaa, 3), (qaa, 2), (qua, 2),
                    (ABC, 1), (ABD, 1),
                    (INSTANT, 1),
                    (JUI, 1), (JUN, 1),
                    (JUNCTURE, 1), (MOMENT, 1),
                    (XYA, 1), (XYAB, 1), (XYABA, 1), (XYABC, 1),
                    (XYQ, 1),
                    (abc, 1), (abcd, 1), (abce, 1),                    
                    (abd, 1),
                    (percentile, 1),
                    (qrs, 1), (qrt, 1),
                    (quail, 1), (qualification, 1), (quality, 1), (quantity, 1),                    
                    (qwa, 1),
                    (xya, 1), (xyz, 1),
                    (zzbb, 1)
                ];
                
                let mut counter = FrequencyCounter::new();
                let fd = counter.count(&mut strs);
                
                assert_eq!(STRS_LEN, fd.capacity());
                assert_eq!(proof.len(), fd.len());
                
                for (ix, ex) in proof.iter().enumerate() {
                    let entry = &fd[ix];
                    assert_eq!(ex.0, entry.value);
                    assert_eq!(ex.1, entry.frequ);
                }
            }
        }
    }
}
