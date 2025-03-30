//! Poetrie, poetic trie, is trie designated for finding rhymes for your verses.
//!
//! For given input it will find word with lengthiest common suffix for you.
// improvements:
//      - return all words with x-length common suffix
//      - allow to speficy expected min a max suffix match length
//      - custom letter equalizer
use std::{collections::hash_map::HashMap, ops::Deref};

mod uc;
use uc::UC;

type Links = HashMap<char, Node>;

fn ext(l: &mut Links, buff: &mut String, o: &mut Vec<String>) {
    for (k, n) in l.iter_mut() {
        buff.push(*k);

        if n.entry {
            let entry = buff.clone();
            o.push(entry);
        }

        if let Some(l) = n.links.as_mut() {
            ext(l, buff, o);
        }

        _ = buff.pop();
    }
}

/// `& str` validated for usage with `Poetrie`.
#[derive(Clone, PartialEq, Debug)]
pub struct Entry<'a>(&'a str);

impl<'a> Entry<'a> {
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

/// Potrie, poetic retrieval tree implementation for finding common word suffixes.
pub struct Potrie {
    root: Node,
    // backtrace buff
    btr: UC<Vec<(char, *mut Node)>>,
    // entries count
    cnt: usize,
}

const NULL: char = '\0';
impl Potrie {
    /// Ctor.
    pub const fn new() -> Potrie {
        Potrie {
            root: Node::empty(),
            btr: UC::new(Vec::new()),
            cnt: 0,
        }
    }

    /// Inserts entry specified into tree.
    ///
    /// Return value is `true` if `entry` was inserted into tree, `false` if was present already.
    pub fn ins(&mut self, entry: &Entry) -> bool {
        let mut node = &mut self.root;
        for c in entry.chars() {
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
        let en_duo = trace.next_back().unwrap();
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

    fn track(&self, entry: &Entry, trace: bool) -> TraRes {
        let mut node = &self.root;
        let tr = self.btr.get_mut();

        if trace {
            tr.push((NULL, node.to_mut_ptr()));
        }

        for c in entry.chars() {
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
        let mut buff = String::with_capacity(1000);

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

#[cfg_attr(test, derive(PartialEq, Clone))]
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

    mod ext {

        use crate::{Entry, Potrie, ext};

        #[test]
        fn basic_test() {
            let mut trie = Potrie::new();

            let a = &Entry("a");
            let z = &Entry("z");

            _ = trie.ins(a);
            _ = trie.ins(z);

            let mut buff = String::new();
            let mut test = Vec::new();

            let links = unsafe { trie.root.links.as_mut().unwrap_unchecked() };
            ext(links, &mut buff, &mut test);

            let proof = vec![String::from("a"), String::from("z")];
            assert_eq!(proof.len(), test.len());
            test.sort();

            assert_eq!(proof, test);

            assert_eq!(true, trie.en(a));
            assert_eq!(true, trie.en(z));
        }

        #[test]
        fn nesting() {
            let mut trie = Potrie::new();

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
                _ = trie.ins(&Entry(e.as_str()));
            }

            let mut buff = String::new();
            let mut test = Vec::new();

            let links = unsafe { trie.root.links.as_mut().unwrap_unchecked() };
            ext(links, &mut buff, &mut test);

            assert_eq!(entries.len(), test.len());

            test.sort();
            assert_eq!(entries, test);
        }

        #[test]
        fn in_depth_recursion() {
            let mut trie = Potrie::new();

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
                _ = trie.ins(&Entry(p.as_str()));
            }

            let mut buff = String::new();
            let mut test = Vec::new();

            let links = unsafe { trie.root.links.as_mut().unwrap_unchecked() };
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

    mod trie {
        use crate::Potrie;

        #[test]
        fn new() {
            let trie = Potrie::new();

            let root = trie.root;
            assert_eq!(false, root.entry);            
            assert_eq!(None, root.links);
            assert_eq!(0, trie.cnt);
        }

        mod ins {
            use crate::{Entry, Potrie};

            #[test]
            fn basic_test() {
                let entry = Entry("touchstone");

                let mut trie = Potrie::new();
                let res = trie.ins(&entry);
                assert_eq!(true, res);

                let links = &trie.root.links.as_ref();
                assert_eq!(true, links.is_some());
                let mut links = links.unwrap();

                let last_node_ix = entry.len() - 1;
                for (ix, c) in entry.chars().enumerate() {
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

                assert_eq!(1, trie.cnt);
            }

            #[test]
            fn existing_path_insert() {
                let existing = &Entry("touchstone");
                let new = &Entry("touch");

                let mut trie = Potrie::new();

                let res = trie.ins(existing);
                assert_eq!(true, res);
                assert_eq!(1, trie.cnt);

                let res = trie.ins(new);
                assert_eq!(true, res);
                assert_eq!(2, trie.cnt);

                assert_eq!(true, trie.en(existing));
                assert_eq!(true, trie.en(new));
            }

            #[test]
            fn singular_entry() {
                let e = Entry("a");

                let mut trie = Potrie::new();
                let res = trie.ins(&e);
                assert_eq!(true, res);
                assert_eq!(1, trie.cnt);

                let links = trie.root.links;
                assert_eq!(true, links.is_some());
                let links = links.unwrap();
                let node = links.get(&'a');
                assert_eq!(true, node.is_some());
                assert_eq!(true, node.unwrap().entry);
            }

            #[test]
            fn double_insert() {
                let entry = &Entry("appealing delicacy");

                let mut trie = Potrie::new();
                let res = trie.ins(&entry);
                assert_eq!(true, res);
                assert_eq!(1, trie.cnt);

                let res = trie.ins(&entry);
                assert_eq!(false, res);
                assert_eq!(1, trie.cnt);

                let links = &trie.root.links.as_ref();
                assert_eq!(true, links.is_some());
                let mut links = links.unwrap();

                let last_ix = entry.len() - 1;
                for (ix, c) in entry.chars().enumerate() {
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

            use crate::{Entry, Potrie};

            #[test]
            fn member() {
                let e = &Entry("Keyword");
                let mut trie = Potrie::new();
                _ = trie.ins(e);

                let res = trie.en(e);
                assert_eq!(true, res);
            }

            #[test]
            fn not_member() {
                let e = &Entry("Keyword");
                let mut trie = Potrie::new();
                _ = trie.ins(e);

                for e in ["Key", "Opener"] {
                    let e = Entry(e);
                    let res = trie.en(&e);
                    assert_eq!(false, res);
                }
            }
        }

        mod rem {
            use crate::{Entry, Potrie};

            #[test]
            fn known_unknown() {
                let known = &Entry("safe-hideaway");
                let unknown = &Entry("grave-monition");

                let mut trie = Potrie::new();
                _ = trie.ins(known);

                assert_eq!(false, trie.rem(unknown));
                assert_eq!(0, trie.btr.len());
                assert_eq!(1, trie.cnt);

                assert_eq!(true, trie.rem(known));
                assert_eq!(0, trie.btr.len());
                assert_eq!(0, trie.cnt);
                assert_eq!(false, trie.en(known));
            }
        }

        // node in path to entry being deleted cannot
        // be deleted if and only if participates in
        // path to another entry where path len varies 0…m
        mod rem_actual {

            use crate::{Entry, Potrie};

            #[test]
            fn basic_test() {
                let entry = &Entry("abcxyz");

                let mut trie = Potrie::new();
                _ = trie.ins(entry);
                _ = trie.track(entry, true);

                _ = trie.rem_actual(&mut 0);
                assert_eq!(false, trie.en(entry));
            }

            #[test]
            fn inner_entry() {
                let mut trie = Potrie::new();

                let outer = &Entry("Keyword");
                _ = trie.ins(outer);

                let inner = &Entry("Key");
                _ = trie.ins(inner);

                let mut esc_code = 0;
                _ = trie.track(inner, true);

                _ = trie.rem_actual(&mut esc_code);
                assert_eq!(1, esc_code);

                assert_eq!(false, trie.en(inner));
                assert_eq!(true, trie.en(outer));
            }

            #[test]
            fn links_removal() {
                let entry = &Entry("Keyword");
                let mut trie = Potrie::new();
                _ = trie.ins(entry);

                let mut esc_code = 0;
                _ = trie.track(entry, true);
                _ = trie.rem_actual(&mut esc_code);
                assert_eq!(4, esc_code);

                assert_eq!(false, trie.en(entry));
                assert_eq!(None, trie.root.links);
            }

            #[test]
            fn node_composing_path() {
                let dissimilar = &Entry("Dissimilar");
                let keyword = &Entry("Keyword");

                let mut trie = Potrie::new();
                _ = trie.ins(dissimilar);
                _ = trie.ins(keyword);

                let mut esc_code = 0;
                _ = trie.track(keyword, true);
                _ = trie.rem_actual(&mut esc_code);
                assert_eq!(2, esc_code);

                assert_eq!(false, trie.en(keyword));
                assert_eq!(true, trie.en(dissimilar));
            }

            #[test]
            fn entry_under_entry() {
                let above = &Entry("keyworder");
                let under = &Entry("keyworders");
                let mut trie = Potrie::new();
                _ = trie.ins(above);
                _ = trie.ins(under);

                let mut esc_code = 0;
                _ = trie.track(under, true);
                _ = trie.rem_actual(&mut esc_code);
                assert_eq!(3, esc_code);

                assert_eq!(false, trie.en(under));
                assert_eq!(true, trie.en(above));

                _ = trie.track(above, true);
                let btr = &trie.btr;
                let last = btr[btr.len() - 1];
                assert_eq!('r', last.0);
                let node = unsafe { last.1.as_ref() }.unwrap();
                assert_eq!(false, node.links());
            }
        }

        mod track {

            use crate::{Entry, NULL, Potrie, TraRes};

            #[test]
            fn tracing() {
                let mut trie = Potrie::new();

                let entries = ["k", "key", "keyword"];
                for e in entries {
                    _ = trie.ins(&Entry(e));
                }

                let keyword = entries[2];

                _ = trie.track(&Entry(keyword), true);

                let trace = trie.btr;
                let proof = format!("{}{}", NULL, keyword);
                for (ix, c) in proof.chars().enumerate() {
                    let d = trace[ix];
                    assert_eq!(c, d.0);
                }

                for e in entries {
                    let (_, node) = trace[e.len()];
                    let node = unsafe { node.as_ref() }.unwrap();
                    assert_eq!(true, node.entry);
                }
            }

            #[test]
            fn ok() {
                let entry = &Entry("información meteorológica");

                let mut trie = Potrie::new();
                _ = trie.ins(entry);
                let res = trie.track(entry, false);

                assert_eq!(TraRes::Ok, res);
            }

            #[test]
            fn unknown_not_path() {
                let entry = &Entry("wordbook");
                let bad_entry = &Entry("wordbooks");

                let mut trie = Potrie::new();
                _ = trie.ins(entry);
                let res = trie.track(bad_entry, false);
                assert_eq!(TraRes::UnknownForAbsentPathLinks, res);
            }

            #[test]
            fn unknown_not_path2() {
                let entry = &Entry("wordbookz");
                let bad_entry = &Entry("wordbooks");

                let mut trie = Potrie::new();
                _ = trie.ins(entry);
                let res = trie.track(bad_entry, false);
                assert_eq!(TraRes::UnknownForAbsentPathNode, res);
            }

            #[test]
            fn unknown_not_entry() {
                let entry = &Entry("wordbooks");
                let bad_entry = &Entry("wordbook");

                let mut trie = Potrie::new();
                _ = trie.ins(entry);

                let res = trie.track(bad_entry, false);
                assert_eq!(TraRes::UnknownForNotEntry, res);
            }
        }

        #[test]
        fn ct() {
            let test = 3;
            let mut trie = Potrie::new();
            assert_eq!(0, trie.ct());
            trie.cnt = test;
            assert_eq!(test, trie.ct());
        }

        mod ext {
            use crate::{Entry, Potrie};

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

                let mut trie = Potrie::new();
                for e in entries.clone() {
                    _ = trie.ins(&e);
                }

                let ext = trie.ext();
                assert_eq!(true, ext.is_some());
                let mut ext = ext.unwrap();

                assert_eq!(proof.len(), ext.len());

                ext.sort();
                assert_eq!(proof, ext);

                assert_eq!(true, ext.capacity() >= 1000);

                for e in entries.clone() {
                    assert_eq!(true, trie.en(&e));
                }
            }

            #[test]
            fn empty_tree() {
                let mut trie = Potrie::new();
                let ext = trie.ext();

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

// cargo test --release
