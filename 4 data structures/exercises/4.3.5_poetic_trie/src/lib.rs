//! Poetrie, poetic trie, is trie designated for finding rhymes for your verses.
//!
//! For given input, and populated tree, it will find word with lengthiest shared suffix for you.
// improvements:
//      - return n (10, max 10, …) words with x-length shared suffix
//      - allow to speficy expected min a max suffix match length
//      - custom letter equalizer
//      - use verbose method names
//      - case insensivity
// check 'imp:' also
use std::{collections::hash_map::HashMap, ops::Deref};

mod uc;
use uc::UC;

type Links = HashMap<char, Node>;

fn ext(l: &mut Links, buff: &mut Vec<char>, o: &mut Vec<String>) {
    for (k, n) in l.iter_mut() {
        buff.push(*k);

        if n.entry {
            let entry = buff.iter().rev().collect();
            o.push(entry);
        }

        if let Some(l) = n.links.as_mut() {
            ext(l, buff, o);
        }

        _ = buff.pop();
    }
}

/// `Entry` alias for using in key role.
pub type Key<'a> = Entry<'a>;

/// `&str` validated for usage with `Poetrie`.
#[derive(Clone, PartialEq, Debug)]
pub struct Entry<'a>(&'a str);

impl<'a> Entry<'a> {
    /// Constructor for `&str`.
    ///
    /// Return value is `None` for 0-length `str`.
    pub const fn new_from_str(entry: &'a str) -> Option<Self> {
        if entry.len() == 0 {
            None
        } else {
            Some(Entry(entry))
        }
    }
}

impl<'a> Deref for Entry<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Poetrie is poetic retrieval tree implementation for finding words with shared suffixes.
///
/// Inputs are validated only for 0 length thus is up to consumer code
/// to allow population with sensible values only.
///
/// All methods are case sensitive.
pub struct Poetrie {
    root: Node,
    // backtrace buff
    btr: UC<Vec<(char, *mut Node)>>,
    // entries count
    cnt: usize,
}

const NULL: char = '\0';
impl Poetrie {
    /// `Poetrie` constructor.    
    pub const fn new() -> Poetrie {
        Poetrie {
            root: Node::empty(),
            btr: UC::new(Vec::new()),
            cnt: 0,
        }
    }

    /// Inserts entry specified into tree.
    ///
    /// Return value is `true` if `entry` was inserted into tree, `false` if it was present already.
    pub fn ins(&mut self, entry: &Entry) -> bool {
        let mut node = &mut self.root;
        let mut chars = entry.chars();
        while let Some(c) = chars.next_back() {
            let links = node.links.get_or_insert_with(|| Links::new());
            node = links.entry(c).or_insert(Node::empty());
        }

        if node.entry {
            false
        } else {
            node.entry = true;
            self.cnt += 1;

            true
        }
    }

    /// Verifies whether `entry` provided is present in tree.
    ///
    /// Return value is `true` if `entry` is present in tree, `false` otherwise.
    pub fn en(&self, entry: &Entry) -> bool {
        let res = self.track(entry, false);

        TraRes::Ok == res
    }

    /// Finds entry with most shared suffix to key.
    ///
    /// If there are more entries with equal suffix length
    /// only one in unguaranteed order is returned.
    pub fn suf(&self, key: &Key) -> String {
        String::new()
    }

    /// Removes entry from tree.
    ///
    /// Return value is `true` if entry was removed, `false` if it was not present.
    pub fn rem(&mut self, entry: &Entry) -> bool {
        let tra_res = self.track(entry, true);
        let res = if let TraRes::Ok = tra_res {
            self.rem_actual(
                #[cfg(test)]
                &mut 0,
            );

            self.cnt -= 1;
            true
        } else {
            false
        };

        self.btr.get_mut().clear();
        res
    }

    fn rem_actual(&mut self, #[cfg(test)] esc_code: &mut usize) {
        let mut trace = self.btr.iter();
        let en_duo = unsafe { trace.next_back().unwrap_unchecked() };
        let mut node = unsafe { en_duo.1.as_mut() }.unwrap();

        node.entry = false;
        if node.links() {
            #[cfg(test)]
            {
                *esc_code = 1;
            }

            return;
        }

        // subnode entry
        let mut sn_entry = en_duo.0;
        while let Some((c, n)) = trace.next_back() {
            node = unsafe { n.as_mut() }.unwrap();
            let links = node.links.as_mut().unwrap();
            _ = links.remove(&sn_entry);

            if links.len() > 0 {
                #[cfg(test)]
                {
                    *esc_code = 2;
                }

                return;
            }

            if node.entry {
                #[cfg(test)]
                {
                    *esc_code = 3;
                }

                break;
            }

            sn_entry = *c;
        }

        node.links = None;
        #[cfg(test)]
        {
            if *esc_code != 3 {
                *esc_code = 4;
            }
        }
    }

    // case-sensitive which is not senseful
    fn find(&self, key: &Key, #[cfg(test)] b_code: &mut usize) -> FindRes {
        let mut chars = key.chars();
        let mut c;

        // match
        let mut buff = Vec::with_capacity(1000);

        // operative node
        let mut op_node = &self.root;
        if let Some(l) = op_node.links.as_ref() {
            c = unsafe { chars.next_back().unwrap_unchecked() };
            if let Some(n) = l.get(&c) {
                op_node = n;
                buff.push(c)
            } else {
                return FindRes::NoJointSuffix;
            }
        } else {
            return FindRes::EmptyTree;
        }

        // closest branch information
        let mut branching = None;
        let mut bak_len = 0;

        // track key as much as possible first
        'track: loop {
            if let Some(next_c) = chars.next_back() {
                if op_node.entry {
                    bak_len = buff.len();
                }

                c = next_c;
            } else {
                #[cfg(test)]
                set_bcode(2, b_code);
                break 'track;
            };

            if let Some(l) = op_node.links.as_ref() {
                if l.len() > 1 {
                    branching = Some((l, (buff.len(), c)));
                }

                if let Some(n) = l.get(&c) {
                    buff.push(c);
                    op_node = n;

                    continue;
                }

                #[cfg(test)]
                set_bcode(4, b_code);
                break 'track;
            }

            #[cfg(test)]
            set_bcode(8, b_code);
            break 'track;
        }

        // CONTINUATION
        // Is possible:
        // - key is suffix to some entry
        // - key has partially shared suffix with some entry
        //
        // Not possible:
        // - key is entry and no suffix to other entry
        // - part of key suffix is other entry
        if !op_node.links.is_some() {
            if let Some((blinks, (blen, skip_c))) = branching {
                // subentry with longer shared suffix must be
                // prioritized over branch
                if bak_len > blen {
                    #[cfg(test)]
                    set_bcode(256, b_code);
                    return ok(&buff[..bak_len]);
                }

                #[cfg(test)]
                set_bcode(512, b_code);

                buff.truncate(blen);

                // imp: possibly randomize somehow node selection
                for (test_c, n) in blinks.iter() {
                    let test_c = *test_c;
                    if test_c == skip_c {
                        continue;
                    }

                    buff.push(test_c);
                    op_node = n;
                }
            } else {
                return if bak_len == 0 {
                    #[cfg(test)]
                    set_bcode(16, b_code);
                    FindRes::OnlyKeyMatches
                } else {
                    #[cfg(test)]
                    set_bcode(32, b_code);
                    return ok(&buff[..bak_len]);
                };
            }
        }

        // extend or connect branch now, if possible
        while let Some(l) = op_node.links.as_ref() {
            // imp: possibly randomize hashmap key selection
            let (c, n) = unsafe { l.iter().next().unwrap_unchecked() };
            buff.push(*c);
            op_node = n;
        }

        #[cfg(test)]
        set_bcode(128, b_code);
        return ok(&buff);

        fn ok(cs: &[char]) -> FindRes {
            return FindRes::Ok(cs.iter().rev().collect());
        }

        #[cfg(test)]
        fn set_bcode(c: usize, b_code: &mut usize) {
            let code = *b_code;
            *b_code = code | c;
        }
    }

    fn track(&self, entry: &Entry, trace: bool) -> TraRes {
        let mut node = &self.root;
        let tr = self.btr.get_mut();

        if trace {
            tr.push((NULL, node.to_mut_ptr()));
        }

        let mut chars = entry.chars();
        while let Some(c) = chars.next_back() {
            if let Some(l) = node.links.as_ref() {
                if let Some(n) = l.get(&c) {
                    if trace {
                        tr.push((c, n.to_mut_ptr()));
                    }

                    node = n;
                    continue;
                }
                return TraRes::UnknownForAbsentPathNode;
            }

            return TraRes::UnknownForAbsentPathLinks;
        }

        if node.entry {
            TraRes::Ok
        } else {
            TraRes::UnknownForNotEntry
        }
    }

    /// Return value is count of entries in tree.
    pub const fn ct(&self) -> usize {
        self.cnt
    }

    /// Extracts entries from tree and leaves tree intact.
    ///
    /// Extraction is alphabetically unordered.
    ///
    /// Return value is `None` for empty `Poetrie`.    
    pub fn ext(&mut self) -> Option<Vec<String>> {
        if self.cnt == 0 {
            return None;
        }

        // capacity is prebuffered to 1000
        let mut buff = Vec::with_capacity(1000);

        // capacity is prebuffered to 1000
        let mut res = Vec::with_capacity(1000);

        let rl = unsafe { self.root.links.as_mut().unwrap_unchecked() };
        ext(rl, &mut buff, &mut res);

        Some(res)
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
enum TraRes {
    Ok,
    UnknownForNotEntry,
    UnknownForAbsentPathLinks,
    UnknownForAbsentPathNode,
}

#[cfg_attr(test, derive(Debug), derive(PartialEq))]
enum FindRes {
    Ok(String),
    OnlyKeyMatches,
    EmptyTree,
    NoJointSuffix,
}

struct Node {
    links: Option<Links>,
    entry: bool,
}

impl Node {
    const fn links(&self) -> bool {
        self.links.is_some()
    }

    const fn empty() -> Self {
        Node {
            links: None,
            entry: false,
        }
    }

    const fn to_mut_ptr(&self) -> *mut Self {
        (self as *const Self).cast_mut()
    }
}

#[cfg(test)]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}

#[cfg(test)]
use std::fmt::{Debug, Formatter};

#[cfg(test)]
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let links = if self.links() { "Some" } else { "None" };

        f.write_fmt(format_args!(
            "Node {{\n  links: {:?}\n  entry: {:?}\n}}",
            links, self.entry
        ))
    }
}

#[cfg(test)]
mod tests_of_units {

    mod rev_entry {
        use crate::Entry;

        pub struct RevEntry(pub String);

        impl RevEntry {
            pub fn new(e: &str) -> Self {
                let rev = e.chars().rev().collect();
                RevEntry(rev)
            }

            pub fn entry(&self) -> Entry {
                Entry(self.0.as_str())
            }
        }

        use std::ops::Deref;
        impl Deref for RevEntry {
            type Target = String;
            fn deref(&self) -> &String {
                &self.0
            }
        }
    }

    mod ext {

        use crate::{Entry, Poetrie, ext};

        #[test]
        fn basic_test() {
            let mut poetrie = Poetrie::new();

            let a = &Entry("a");
            let z = &Entry("z");

            _ = poetrie.ins(a);
            _ = poetrie.ins(z);

            let mut buff = Vec::new();
            let mut test = Vec::new();

            let links = unsafe { poetrie.root.links.as_mut().unwrap_unchecked() };
            ext(links, &mut buff, &mut test);

            let proof = vec![String::from("a"), String::from("z")];
            assert_eq!(proof.len(), test.len());
            test.sort();

            assert_eq!(proof, test);

            assert_eq!(true, poetrie.en(a));
            assert_eq!(true, poetrie.en(z));
        }

        #[test]
        fn nesting() {
            let mut poetrie = Poetrie::new();

            let entries = vec![
                String::from("a"),
                String::from("az"),
                String::from("b"),
                String::from("by"),
                String::from("y"),
                String::from("yb"),
                String::from("z"),
                String::from("za"),
            ];

            for e in entries.iter() {
                _ = poetrie.ins(&Entry(e.as_str()));
            }

            let mut buff = Vec::new();
            let mut test = Vec::new();

            let links = unsafe { poetrie.root.links.as_mut().unwrap_unchecked() };
            ext(links, &mut buff, &mut test);

            assert_eq!(entries.len(), test.len());

            test.sort();
            assert_eq!(entries, test);
        }

        #[test]
        fn in_depth_recursion() {
            let mut poetrie = Poetrie::new();

            let paths = vec![
                String::from("aa"),
                String::from("azbq"),
                String::from("by"),
                String::from("ybc"),
                String::from("ybcrqutmop"),
                String::from("ybcrqutmopfvb"),
                String::from("ybcrqutmoprfg"),
                String::from("ybxr"),
                String::from("zazazazazabyyb"),
            ];

            for p in paths.iter() {
                _ = poetrie.ins(&Entry(p.as_str()));
            }

            let mut buff = Vec::new();
            let mut test = Vec::new();

            let links = unsafe { poetrie.root.links.as_mut().unwrap_unchecked() };
            ext(links, &mut buff, &mut test);

            assert_eq!(paths.len(), test.len());

            test.sort();
            assert_eq!(paths, test);
        }
    }

    mod entry {
        use crate::Entry;

        #[test]
        fn new_from_str() {
            let entry = "entry";
            let test = Entry::new_from_str(entry);
            assert_eq!(true, test.is_some());
            assert_eq!(entry.as_ptr() as usize, test.unwrap().as_ptr() as usize);
        }

        #[test]
        fn new_from_str_zero_entry() {
            let entry = "";
            let test = Entry::new_from_str(entry);
            assert_eq!(None, test);
        }
    }

    mod poetrie {
        use crate::Poetrie;

        #[test]
        fn new() {
            let poetrie = Poetrie::new();

            let root = poetrie.root;
            assert_eq!(false, root.entry);
            assert_eq!(None, root.links);
            assert_eq!(0, poetrie.cnt);
        }

        mod ins {
            use crate::{Entry, Poetrie};

            #[test]
            fn basic_test() {
                let entry = Entry("touchstone");

                let mut poetrie = Poetrie::new();
                let res = poetrie.ins(&entry);
                assert_eq!(true, res);

                let links = &poetrie.root.links.as_ref();
                assert_eq!(true, links.is_some());
                let mut links = links.unwrap();

                let last_node_ix = entry.len() - 1;
                for (ix, c) in entry.chars().rev().enumerate() {
                    let node = &links.get(&c);

                    assert!(node.is_some());
                    let node = node.unwrap();

                    if ix == last_node_ix {
                        assert_eq!(false, node.links());
                        assert_eq!(true, node.entry);
                    } else {
                        assert_eq!(false, node.entry);
                        assert_eq!(true, node.links());
                        links = node.links.as_ref().unwrap();
                    }
                }

                assert_eq!(1, poetrie.cnt);
            }

            #[test]
            fn existing_path_insert() {
                let existing = &Entry("touchstone");
                let new = &Entry("touch");

                let mut poetrie = Poetrie::new();

                let res = poetrie.ins(existing);
                assert_eq!(true, res);
                assert_eq!(1, poetrie.cnt);

                let res = poetrie.ins(new);
                assert_eq!(true, res);
                assert_eq!(2, poetrie.cnt);

                assert_eq!(true, poetrie.en(existing));
                assert_eq!(true, poetrie.en(new));
            }

            #[test]
            fn singular_entry() {
                let e = Entry("a");

                let mut poetrie = Poetrie::new();
                let res = poetrie.ins(&e);
                assert_eq!(true, res);
                assert_eq!(1, poetrie.cnt);

                let links = poetrie.root.links;
                assert_eq!(true, links.is_some());
                let links = links.unwrap();
                let node = links.get(&'a');
                assert_eq!(true, node.is_some());
                assert_eq!(true, node.unwrap().entry);
            }

            #[test]
            fn double_insert() {
                let entry = &Entry("appealing delicacy");

                let mut poetrie = Poetrie::new();
                let res = poetrie.ins(&entry);
                assert_eq!(true, res);
                assert_eq!(1, poetrie.cnt);

                let res = poetrie.ins(&entry);
                assert_eq!(false, res);
                assert_eq!(1, poetrie.cnt);

                let links = &poetrie.root.links.as_ref();
                assert_eq!(true, links.is_some());
                let mut links = links.unwrap();

                let last_ix = entry.len() - 1;
                for (ix, c) in entry.chars().rev().enumerate() {
                    let node = links.get(&c);
                    assert_eq!(true, node.is_some());
                    let node = node.unwrap();

                    if ix == last_ix {
                        assert_eq!(false, node.links());
                        assert_eq!(true, node.entry)
                    } else {
                        assert_eq!(true, node.links());
                        links = node.links.as_ref().unwrap();
                    }
                }
            }
        }

        mod en {

            use crate::{Entry, Poetrie};

            #[test]
            fn member() {
                let e = &Entry("Keyword");
                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(e);

                let res = poetrie.en(e);
                assert_eq!(true, res);
            }

            #[test]
            fn not_member() {
                let e = &Entry("Keyword");
                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(e);

                for e in ["Key", "Opener"] {
                    let e = Entry(e);
                    let res = poetrie.en(&e);
                    assert_eq!(false, res);
                }
            }
        }

        mod rem {
            use crate::{Entry, Poetrie};

            #[test]
            fn known_unknown() {
                let known = &Entry("safe-hideaway");
                let unknown = &Entry("grave-monition");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(known);

                assert_eq!(false, poetrie.rem(unknown));
                assert_eq!(0, poetrie.btr.len());
                assert_eq!(1, poetrie.cnt);

                assert_eq!(true, poetrie.rem(known));
                assert_eq!(0, poetrie.btr.len());
                assert_eq!(0, poetrie.cnt);
                assert_eq!(false, poetrie.en(known));
            }
        }

        // node in path to entry being deleted cannot
        // be deleted if and only if participates in
        // path to another entry where path len varies 0…m
        mod rem_actual {

            use super::super::rev_entry::RevEntry;
            use crate::Poetrie;

            #[test]
            fn basic_test() {
                let entry = RevEntry::new("abcxyz");
                let entry = &entry.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                _ = poetrie.track(entry, true);

                _ = poetrie.rem_actual(&mut 0);
                assert_eq!(false, poetrie.en(entry));
            }

            #[test]
            fn inner_entry() {
                let mut poetrie = Poetrie::new();

                let outer = RevEntry::new("Keyword");
                let outer = &outer.entry();
                _ = poetrie.ins(&outer);

                let inner = RevEntry::new("Key");
                let inner = &inner.entry();
                _ = poetrie.ins(inner);

                let mut esc_code = 0;
                _ = poetrie.track(inner, true);

                _ = poetrie.rem_actual(&mut esc_code);
                assert_eq!(1, esc_code);

                assert_eq!(false, poetrie.en(inner));
                assert_eq!(true, poetrie.en(outer));
            }

            #[test]
            fn links_removal() {
                let entry = RevEntry::new("Keyword");
                let entry = &entry.entry();
                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);

                let mut esc_code = 0;
                _ = poetrie.track(entry, true);
                _ = poetrie.rem_actual(&mut esc_code);
                assert_eq!(4, esc_code);

                assert_eq!(false, poetrie.en(entry));
                assert_eq!(None, poetrie.root.links);
            }

            #[test]
            fn node_composing_path() {
                let dissimilar = RevEntry::new("Dissimilar");
                let dissimilar = &dissimilar.entry();
                let keyword = RevEntry::new("Keyword");
                let keyword = &keyword.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(dissimilar);
                _ = poetrie.ins(keyword);

                let mut esc_code = 0;
                _ = poetrie.track(keyword, true);
                _ = poetrie.rem_actual(&mut esc_code);
                assert_eq!(2, esc_code);

                assert_eq!(false, poetrie.en(keyword));
                assert_eq!(true, poetrie.en(dissimilar));
            }

            #[test]
            fn entry_under_entry() {
                let above = RevEntry::new("keyworder");
                let above = &above.entry();
                let under = RevEntry::new("keyworders");
                let under = &under.entry();
                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(above);
                _ = poetrie.ins(under);

                let mut esc_code = 0;
                _ = poetrie.track(under, true);
                _ = poetrie.rem_actual(&mut esc_code);
                assert_eq!(3, esc_code);

                assert_eq!(false, poetrie.en(under));
                assert_eq!(true, poetrie.en(above));

                _ = poetrie.track(above, true);
                let btr = &poetrie.btr;
                let last = btr[btr.len() - 1];
                assert_eq!('r', last.0);
                let node = unsafe { last.1.as_ref() }.unwrap();
                assert_eq!(false, node.links());
            }
        }

        mod find {
            use crate::{Entry, FindRes, Poetrie, tests_of_units::rev_entry::RevEntry};

            #[test]
            fn basic_test() {
                let proof = String::from("halieutics");
                let entry = &Entry(proof.as_str());

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                _ = poetrie.ins(&Entry("codecs"));

                let key = &Entry("lyrics");
                let find = poetrie.find(key, &mut 0);

                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn exactly_last_match_1a() {
                let entry = &Entry("s");
                let key = &Entry("lyrics");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(40, b_code);
                assert_eq!(FindRes::Ok(String::from("s")), find);
            }

            #[test]
            fn exactly_last_match_1b() {
                let entry = &Entry("s");
                let key = &Entry("lyrics");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(34, b_code);
                assert_eq!(FindRes::Ok(String::from("s")), find);
            }

            #[test]
            fn exactly_last_match_2a() {
                let proof = String::from("lyrics");
                let entry = &Entry(proof.as_str());
                let key = &Entry("s");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(130, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn exactly_last_match_2b() {
                let proof = String::from("lyrics");
                let entry = &Entry(proof.as_str());
                let key = &Entry("s");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(130, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn exactly_last_match_3() {
                let key_entry = &Entry("s");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(key_entry);

                let mut b_code = 0;
                let find = poetrie.find(key_entry, &mut b_code);

                assert_eq!(18, b_code);
                assert_eq!(FindRes::OnlyKeyMatches, find);
            }

            #[test]
            fn no_data() {
                let key = &Entry("lyrics");

                let poetrie = Poetrie::new();
                let find = poetrie.find(key, &mut 0);

                assert_eq!(FindRes::EmptyTree, find);
            }

            #[test]
            fn no_suffix_match() {
                let entry = &Entry("epicalyx");
                let key = &Entry("lyrics");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);

                let find = poetrie.find(key, &mut 0);

                assert_eq!(FindRes::NoJointSuffix, find);
            }

            #[test]
            fn key_matches_itself_only() {
                let itself = &Entry("lyrics");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(itself);

                let mut b_code = 0;
                let find = poetrie.find(itself, &mut b_code);

                assert_eq!(18, b_code);
                assert_eq!(FindRes::OnlyKeyMatches, find);
            }

            #[test]
            fn key_is_suffix_to_entry_1() {
                let subentry = RevEntry::new("document");
                let entry = RevEntry::new("documentalist");
                let proof = entry.0.clone();

                let key = RevEntry::new("documental");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry.entry());
                _ = poetrie.ins(&entry.entry());

                let mut b_code = 0;
                let find = poetrie.find(&key.entry(), &mut b_code);

                assert_eq!(130, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn key_is_suffix_to_entry_2() {
                let subentry = RevEntry::new("document");
                let entry = RevEntry::new("documentalist");
                let proof = entry.0.clone();

                let key = RevEntry::new("documental");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry.entry());
                _ = poetrie.ins(&entry.entry());
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(130, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn only_subentry_is_possible1() {
                let subentry = RevEntry::new("document");
                let entry = RevEntry::new("documental");
                let proof = entry.0.clone();

                let key = RevEntry::new("documentalist");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry.entry());
                _ = poetrie.ins(&entry.entry());
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(34, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn only_subentry_is_possible2() {
                let proof = String::from("m");
                let subentry = Entry(proof.as_str());

                let key = &Entry("anagram");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry);
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(34, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn only_subsuffix_is_possible1() {
                let subentry = RevEntry::new("document");
                let entry = RevEntry::new("documental");
                let proof = entry.0.clone();

                let key = RevEntry::new("documentalist");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry.entry());
                _ = poetrie.ins(&entry.entry());

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(40, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn only_subsuffix_is_possible2() {
                let proof = String::from("m");
                let entry = Entry(proof.as_str());

                let key = &Entry("conundrum");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&entry);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(40, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn must_not_recourse_to_root_branching1() {
                let proof = String::from("hilum");
                let subentry = Entry(proof.as_str());
                let entry = Entry("claybank");

                let key = &Entry("haulm");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry);
                _ = poetrie.ins(&entry);
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(642, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn must_not_recourse_to_root_branching2() {
                let proof = String::from("hilum");
                let subentry = Entry(proof.as_str());
                let entry = Entry("claybank");

                let key = &Entry("haulm");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&subentry);
                _ = poetrie.ins(&entry);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(132, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn key_partially_shared_suffix_1a() {
                let proof = String::from("lyrics");
                let entry = &Entry(proof.as_str());

                let key = &Entry("athletics");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(132, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn key_partially_shared_suffix_1b() {
                let proof = String::from("lyrics");
                let entry = &Entry(proof.as_str());

                let key = &Entry("carboniferous");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(132, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn key_partially_shared_suffix_2a() {
                let proof = String::from("lyrics");
                let entry = &Entry(proof.as_str());

                let key = &Entry("athletics");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(642, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn key_partially_shared_suffix_2b() {
                let proof = String::from("lyrics");
                let entry = &Entry(proof.as_str());

                let key = &Entry("carboniferous");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(642, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn prefer_suffix_entry_when_longer_share_1() {
                // branching entry
                let bra_ent = RevEntry::new("documentarian");
                let suf_ent = RevEntry::new("documental");
                let proof = suf_ent.0.clone();

                let key = RevEntry::new("documentalist");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&suf_ent.entry());
                _ = poetrie.ins(&bra_ent.entry());

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(264, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn prefer_suffix_entry_when_longer_share_2() {
                // branching entry
                let bra_ent = RevEntry::new("documentarian");
                let suf_ent = RevEntry::new("documental");
                let proof = suf_ent.0.clone();

                let key = RevEntry::new("documentalist");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&suf_ent.entry());
                _ = poetrie.ins(&bra_ent.entry());
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(258, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            // redundant, kept for parity
            // same as key_partially_shared_suffix_1a
            fn prefer_branching_entry_when_at_least_same_share_1() {
                let bra_ent = RevEntry::new("documented");
                let proof = bra_ent.0.clone();

                // suffix entry
                let suf_ent = RevEntry::new("document");

                let key = RevEntry::new("documentalist");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&suf_ent.entry());
                _ = poetrie.ins(&bra_ent.entry());

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(132, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            #[test]
            fn prefer_branching_entry_when_at_least_same_share_2() {
                let bra_ent = RevEntry::new("documented");
                let proof = bra_ent.0.clone();

                // suffix entry
                let suf_ent = RevEntry::new("document");

                let key = RevEntry::new("documentalist");
                let key = &key.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&suf_ent.entry());
                _ = poetrie.ins(&bra_ent.entry());
                _ = poetrie.ins(key);

                let mut b_code = 0;
                let find = poetrie.find(key, &mut b_code);

                assert_eq!(642, b_code);
                assert_eq!(FindRes::Ok(proof), find);
            }

            // inject all cases
            #[test]
            fn load() {
                let mut poetrie = Poetrie::new();

                let entries = ["aesthetics", "statics", "mechanics", "athletics", "physics"];
                for e in entries {
                    _ = poetrie.ins(&Entry(e));
                }

                let mut b_code = 0;
                let key = Entry("musics");
                let proof = String::from("physics");

                assert_eq!(FindRes::Ok(proof), poetrie.find(&key, &mut b_code));
                assert_eq!(132, b_code);

                b_code = 0;
                let key = Entry("athletics");
                let proof = String::from("aesthetics");

                assert_eq!(
                    FindRes::Ok(proof),
                    poetrie.find(&key, &mut b_code),
                    "{}",
                    b_code
                );
                assert_eq!(642, b_code);

                b_code = 0;
                let key = Entry("aesthetics");
                let proof = String::from("athletics");

                assert_eq!(FindRes::Ok(proof), poetrie.find(&key, &mut b_code));
                assert_eq!(642, b_code);
            }
        }

        mod track {

            use super::super::rev_entry::RevEntry;
            use crate::{NULL, Poetrie, TraRes};

            #[test]
            fn tracing() {
                let mut poetrie = Poetrie::new();

                let keyword = "keyword";
                let entries = ["k", "key", keyword].map(|x| RevEntry::new(x));

                for e in entries.iter() {
                    _ = poetrie.ins(&e.entry());
                }

                _ = poetrie.track(&entries[2].entry(), true);

                let trace = poetrie.btr;
                let proof = format!("{}{}", NULL, keyword);
                for (ix, c) in proof.chars().enumerate() {
                    let d = trace[ix];
                    assert_eq!(c, d.0, "{ix}");
                }

                for e in entries {
                    let (c, node) = trace[e.len()];
                    let node = unsafe { node.as_ref() }.unwrap();
                    assert_eq!(true, node.entry, "c: {c}, e: {}", *e);
                }
            }

            #[test]
            fn ok() {
                let entry = RevEntry::new("información meteorológica");
                let entry = &entry.entry();

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(entry);
                let res = poetrie.track(entry, false);

                assert_eq!(TraRes::Ok, res);
            }

            #[test]
            fn unknown_not_path() {
                let entry = RevEntry::new("wordbook");
                let bad_entry = RevEntry::new("wordbooks");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&entry.entry());
                let res = poetrie.track(&bad_entry.entry(), false);
                assert_eq!(TraRes::UnknownForAbsentPathLinks, res);
            }

            #[test]
            fn unknown_not_path2() {
                let entry = RevEntry::new("wordbookz");
                let bad_entry = RevEntry::new("wordbooks");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&entry.entry());
                let res = poetrie.track(&bad_entry.entry(), false);
                assert_eq!(TraRes::UnknownForAbsentPathNode, res);
            }

            #[test]
            fn unknown_not_entry() {
                let entry = RevEntry::new("wordbooks");
                let bad_entry = RevEntry::new("wordbook");

                let mut poetrie = Poetrie::new();
                _ = poetrie.ins(&entry.entry());

                let res = poetrie.track(&bad_entry.entry(), false);
                assert_eq!(TraRes::UnknownForNotEntry, res);
            }
        }

        #[test]
        fn ct() {
            let test = 3;
            let mut poetrie = Poetrie::new();
            assert_eq!(0, poetrie.ct());
            poetrie.cnt = test;
            assert_eq!(test, poetrie.ct());
        }

        mod ext {
            use crate::{Entry, Poetrie};

            #[test]
            fn basic_test() {
                let proof = vec![
                    String::from("aa"),
                    String::from("azbq"),
                    String::from("by"),
                    String::from("ybc"),
                    String::from("ybxr"),
                    String::from("ybxrqutmop"),
                    String::from("ybxrqutmopfvb"),
                    String::from("ybxrqutmoprfg"),
                    String::from("zazazazazabyyb"),
                ];

                let entries = proof.iter().map(|x| Entry(x.as_str()));

                let mut poetrie = Poetrie::new();
                for e in entries.clone() {
                    _ = poetrie.ins(&e);
                }

                let ext = poetrie.ext();
                assert_eq!(true, ext.is_some());
                let mut ext = ext.unwrap();

                assert_eq!(proof.len(), ext.len());

                ext.sort();
                assert_eq!(proof, ext);

                assert_eq!(true, ext.capacity() >= 1000);

                for e in entries.clone() {
                    assert_eq!(true, poetrie.en(&e));
                }
            }

            #[test]
            fn empty_tree() {
                let mut poetrie = Poetrie::new();
                let ext = poetrie.ext();

                assert_eq!(None, ext);
            }
        }
    }

    mod node {

        use crate::{Links, Node};

        #[test]
        fn links() {
            let mut node = Node::empty();

            assert_eq!(false, node.links());
            node.links = Some(Links::new());
            assert!(node.links());
        }

        #[test]
        fn empty() {
            let node = Node::empty();

            assert_eq!(None, node.links);
            assert_eq!(false, node.entry);
        }

        #[test]
        fn to_mut_ptr() {
            let n = Node::empty();
            let n_add = &n as *const Node as usize;
            assert_eq!(n_add, n.to_mut_ptr() as usize);
        }
    }
}

// cargo fmt && cargo test --release
