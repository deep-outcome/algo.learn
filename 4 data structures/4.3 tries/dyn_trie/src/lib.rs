use std::collections::hash_map::HashMap;

type Links<T> = HashMap<char, Node<T>>;
type Path<'a, T> = Vec<PathNode<'a, T>>;
type PathNode<'a, T> = (char, &'a Node<T>);
type Key = str;

fn entry_path_node<'a, T>(path: &Path<'a, T>, key: &Key) -> Option<PathNode<'a, T>> {
    let key_len = key.len();
    if path.len() < key_len + 1 {
        None
    } else {
        let epn = path[key_len];
        if epn.1.entry() {
            Some(epn)
        } else {
            None
        }
    }
}

pub struct Trie<T> {
    root: Node<T>,
}

const NULL: char = '\0';
impl<T> Trie<T> {
    pub fn new() -> Trie<T> {
        Trie {
            root: Node::<T>::empty(),
        }
    }

    pub fn insert(&mut self, entry: T, key: &Key) {
        let mut node = &mut self.root;
        for c in key.chars() {
            let links = node.links.get_or_insert_with(|| Links::new());
            node = links.entry(c).or_insert(Node::<T>::empty());
        }

        node.entry = Some(entry);
    }

    pub fn member(&self, key: &Key) -> Option<&T> {
        let path = self.path(key);

        let epn = entry_path_node(&path, key);

        if let Some(epn) = epn {
            epn.1.entry.as_ref()
        } else {
            None
        }
    }

    pub fn delete(&mut self, key: &Key) -> Result<(), ()> {
        let path = self.path(key);
        let entry_pn = match entry_path_node(&path, key) {
            Some(epn) => epn,
            _ => return Err(()),
        };

        let entry_n = entry_pn.1;
        if entry_n.links() {
            unsafe { as_mut(entry_n) }.entry = None; // Sounds.
            return Ok(());
        }

        let mut path_rev = path.iter().rev();
        _ = path_rev.next();

        let mut subnode_key = entry_pn.0;
        while let Some((c, n)) = path_rev.next() {
            let n_mut = unsafe { as_mut(*n) }; // Sounds.

            let n_links = n_mut.links.as_mut().unwrap();
            _ = n_links.remove(&subnode_key);

            let c = *c;
            if n_links.len() == 0 {
                n_mut.links = None;
            } else {
                return Ok(());
            }

            if n.entry() {
                return Ok(());
            }

            subnode_key = c;
        }

        return Ok(());

        unsafe fn as_mut<T>(node: &Node<T>) -> &mut Node<T> {
            let ptr: *const Node<T> = node;
            let mut_ptr: *mut Node<T> = std::mem::transmute(ptr);
            mut_ptr.as_mut().unwrap()
        }
    }

    fn path(&self, key: &Key) -> Vec<PathNode<'_, T>> {
        let root = &self.root;
        let mut links = root.links.as_ref();

        let mut path = Vec::with_capacity(key.len() + 1);
        path.push((NULL, root));

        for c in key.chars() {
            if let Some(l) = links {
                if let Some(node) = l.get(&c) {
                    path.push((c, node));
                    links = node.links.as_ref();

                    continue;
                }
            }

            break;
        }

        path
    }
}

#[derive(PartialEq, Clone)]
struct Node<T> {
    links: Option<Links<T>>,
    entry: Option<T>,
}

impl<T> Node<T> {
    fn entry(&self) -> bool {
        self.entry.is_some()
    }

    fn links(&self) -> bool {
        self.links.is_some()
    }

    fn empty() -> Self {
        Node {
            links: None,
            entry: None,
        }
    }
}

use std::fmt::{Debug, Formatter};

impl<T> Debug for Node<T>
where
    T: Debug,
{
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

    mod entry_path_node {
        use crate::{entry_path_node, Node, NULL};

        fn replacement_key(n: usize) -> String {
            const REPLACEMENT: char = '\u{001A}';

            REPLACEMENT.to_string().repeat(n)
        }

        /// Longer key means it is not traced by path.
        #[test]
        fn longer_key() {
            const PATH_LEN: usize = 4;

            let node: Node<usize> = Node::empty();
            let path = vec![(NULL, &node); PATH_LEN];
            let key = replacement_key(PATH_LEN);

            assert_eq!(None, entry_path_node(&path, &key));
        }

        #[test]
        fn not_entry() {
            const PATH_LEN: usize = 5;

            let node: Node<usize> = Node::empty();
            let path = vec![(NULL, &node); PATH_LEN];
            let key = replacement_key(PATH_LEN - 1);

            assert_eq!(None, entry_path_node(&path, &key));
        }

        #[test]
        fn entry() {
            let empty_n: Node<usize> = Node::empty();

            let mut entry_n = empty_n.clone();
            entry_n.entry = Some(0);

            let mut path = vec![(NULL, &empty_n); 4];
            path.push(('a', &entry_n));

            let key = replacement_key(4);

            let epn = entry_path_node(&path, &key);
            assert!(epn.is_some());
            let epn = epn.unwrap();
            assert_eq!('a', epn.0);
            assert!(epn.1.entry());
        }
    }

    mod trie {
        use crate::Trie;

        #[test]
        fn new() {
            let trie = Trie::<usize>::new();

            let root = trie.root;
            assert!(!root.entry());

            let links = &root.links;
            assert!(links.is_none());
        }

        mod insert {
            use crate::Trie;

            #[test]
            fn basic_test() {
                const KEY: &str = "touchstone";

                let mut trie = Trie::new();
                trie.insert(3usize, KEY);

                let last_node_ix = KEY.len() - 1;

                let mut links = trie.root.links.as_ref().unwrap();

                for (ix, c) in KEY.chars().enumerate() {
                    let node = &links.get(&c);

                    assert!(node.is_some());
                    let node = node.unwrap();

                    if ix < last_node_ix {
                        let temp = &node.links;
                        assert!(!node.entry());
                        assert!(temp.is_some());
                        links = temp.as_ref().unwrap();
                    } else {
                        assert!(!node.links());

                        let entry = node.entry;
                        assert!(entry.is_some());
                        assert_eq!(3usize, entry.unwrap());
                    }
                }
            }

            #[test]
            fn existing_path_insert() {
                const EXISTING: &str = "touchstone";
                const NEW: &str = "touch";

                let mut trie = Trie::new();
                trie.insert(3usize, EXISTING);
                trie.insert(4usize, NEW);

                assert!(trie.member(EXISTING).is_some());
                assert!(trie.member(NEW).is_some());
            }
        }

        mod member {

            use crate::Trie;

            #[test]
            fn member() {
                const KEY: &str = "Keyword";
                let mut trie = Trie::new();
                trie.insert(27usize, KEY);

                let member = trie.member(KEY);
                assert!(member.is_some());
                assert_eq!(27, *member.unwrap());
            }

            #[test]
            fn not_member() {
                const KEY: &str = "Keyword";
                let mut trie = Trie::new();
                trie.insert(0usize, KEY);

                for key in ["Key", "Opener"] {
                    let member = trie.member(key);
                    assert!(member.is_none());
                }
            }
        }

        /// Node in path to entry being deleted
        /// cannot be deleted if and only if participates
        /// in path to another entry. Path len varies 0…m.        
        mod delete {

            use crate::Trie;

            #[test]
            fn not_member() {
                const KEY: &str = "Keyword";
                let mut trie = Trie::new();
                trie.insert(0usize, KEY);

                for bad_key in ["Key", "Opener"] {
                    let err = trie.delete(bad_key);
                    assert!(err.is_err());
                    assert!(trie.member(KEY).is_some());
                }
            }

            #[test]
            fn inner_entry() {
                let mut trie = Trie::new();

                const OUTER: &str = "Keyword";
                trie.insert(0usize, OUTER);

                const INNER: &str = "Key";
                trie.insert(0usize, INNER);

                assert!(trie.delete(INNER).is_ok());
                assert!(trie.member(INNER).is_none());
                assert!(trie.member(OUTER).is_some());
            }

            #[test]
            fn links_removal() {
                const KEY: &str = "Keyword";
                let mut trie = Trie::new();
                trie.insert(0usize, KEY);

                assert!(trie.delete(KEY).is_ok());
                assert!(trie.member(KEY).is_none());
                let links = trie.root.links;
                assert!(links.is_none());
            }

            #[test]
            fn node_composing_path() {
                const DISSIMILAR: &str = "Dissimilar";
                const KEYWORD: &str = "Keyword";
                let mut trie = Trie::new();
                trie.insert(0usize, DISSIMILAR);
                trie.insert(0usize, KEYWORD);

                assert!(trie.delete(KEYWORD).is_ok());
                assert!(trie.member(KEYWORD).is_none());
                assert!(trie.member(DISSIMILAR).is_some());
            }

            #[test]
            fn node_being_entry() {
                const KEY1: &str = "Keyword";
                const KEY2: &str = "K";
                let mut trie = Trie::new();
                trie.insert(0usize, KEY1);
                trie.insert(0usize, KEY2);

                assert!(trie.delete(KEY1).is_ok());
                assert!(trie.member(KEY1).is_none());
                assert!(trie.member(KEY2).is_some());

                let k = trie.root.links.as_ref().unwrap().get(&'K');
                assert!(!k.unwrap().links());
            }
        }

        mod path {

            use crate::{Trie, NULL};

            #[test]
            fn path() {
                let mut trie = Trie::<usize>::new();

                let kvs = [("k", 12), ("keyw", 22), ("keyword", 45)];
                for (k, v) in kvs {
                    trie.insert(v, k);
                }

                let keyword = kvs[2].0;
                let proof = format!("{}{}", NULL, keyword);

                let path = trie.path(keyword);
                assert_eq!(proof.len(), path.len());

                let mut ix = 0;
                for c in proof.chars() {
                    let p = path[ix];
                    assert_eq!(c, p.0);
                    ix += 1;
                }

                for (k, v) in kvs {
                    let (_, node) = path[k.len()];
                    assert!(node.entry());
                    assert_eq!(v, node.entry.unwrap());
                }
            }

            #[test]
            fn no_branch() {
                let mut trie = Trie::<usize>::new();

                const KEYBOARD: &str = "Keyboard";
                const KEYWORD: &str = "Keyword";
                trie.insert(0usize, KEYWORD);

                let path = trie.path(KEYBOARD);
                let proof = format!("{}Key", NULL);
                assert_eq!(proof.len(), path.len());

                let mut ix = 0;
                for c in proof.chars() {
                    let p = path[ix];
                    assert_eq!(c, p.0);
                    ix += 1;
                }
            }

            #[test]
            fn no_branches() {
                const KEY: &str = "Key";
                let trie = Trie::<usize>::new();

                let path = trie.path(KEY);
                assert_eq!(1, path.len());
                assert_eq!(NULL, path[0].0);
            }
        }
    }

    mod node {

        use crate::{Links, Node};

        #[test]
        fn entry() {
            let mut node = Node::<usize>::empty();

            assert!(!node.entry());
            node.entry = Some(1);
            assert!(node.entry());
        }

        #[test]
        fn links() {
            let mut node = Node::<usize>::empty();

            assert!(!node.links());
            node.links = Some(Links::new());
            assert!(node.links());
        }

        #[test]
        fn empty() {
            let node = Node::<usize>::empty();

            assert!(node.links.is_none());
            assert!(node.entry.is_none());
        }
    }
}
