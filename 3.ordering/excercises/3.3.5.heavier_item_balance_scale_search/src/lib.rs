use std::collections::HashMap;
use std::iter::Map;
use std::mem::transmute;
use std::result::Result;
use std::slice::Iter;

pub struct Find {
    index: usize,
}

impl Find {
    pub fn get_index(&self) -> usize {
        self.index
    }
}

// #[derive(Debug)]
pub struct Item {
    weight: f32,
}

impl Item {
    pub fn new(weight: f32) -> Item {
        Item { weight }
    }

    pub fn create_items_from_u32(source: &[u32]) -> Vec<Item> {
        let map = source.iter().map(|x| *x as f32);

        Self::create_items_from_f32(map)
    }

    pub fn create_items_from_f32<'a, T: 'a, F>(source: Map<Iter<'a, T>, F>) -> Vec<Item>
    where
        F: FnMut(&'a T) -> f32,
    {
        source.map(|x| Item { weight: x }).collect::<Vec<Item>>()
    }
}

fn validate_input(items: &[Item]) -> Result<(), &'static str> {
    let len = items.len();

    if len < 2usize {
        return Err("Expected at least 2 items.");
    }

    let mut mapping: HashMap<u32, usize> = HashMap::new();

    for i in items {
        unsafe {
            let key: u32 = transmute(i.weight);
            if let Some(val) = mapping.get_mut(&key) {
                *val += 1usize;
            } else {
                mapping.insert(key, 1usize);
            }
        }
    }

    let mut keys: Vec<u32> = mapping.keys().map(|x| *x).collect();
    let keys_len = keys.len();

    if keys_len == 1usize {
        return Err("No heavier item found.");
    }

    if keys_len > 2usize {
        return Err("More than 2 weight kinds found.");
    }

    /* must: keys_len == 2usize here, so     *
     * if keys_len == len, there are exactly *
     * 2 different weights, 1 per key       */
    if keys_len == len {
        return Ok(());
    }

    keys.sort();
    let count2 = *mapping.get(&keys[1]).unwrap();

    if count2 == 1usize {
        return Ok(());
    }

    Err("Expected exactly 1 heavier item amond rest.")
}

pub fn find_heavier_one(items: &[Item]) -> Result<Find, &'static str> {
    if let Err(err) = validate_input(items) {
        return Err(err);
    }

    let len = items.len();

    // shortcut obvious cases to lighten branch
    // inspection (special case results handling)
    let mut end_index = len - 1usize;

    if len < 4usize {
        // ie 2 or 3 items
        return Ok(Find {
            index: compare(items, 0, end_index),
        });
    }

    let mut start_index = 0usize;

    // items balance occurs after branch inspection so if
    // get odd input items, it must be balanced beforehand
    if len % 2 == 1 {
        end_index = end_index - 1usize;
    }

    loop {
        match follow_heavier_branch(items, &mut start_index, &mut end_index) {
            Ok(()) => {
                let diff = end_index - start_index;

                if diff % 2usize == 0 {
                    // when last follow return final sized
                    // branch of size 3, this runs in vain
                    // ;nano overhead, no care
                    end_index = end_index - 1usize;
                }

                if diff > 2usize {
                    continue;
                }

                return Ok(Find {
                    index: compare(items, start_index, end_index),
                });
            }
            Err(()) => {
                return Ok(Find {
                    index: end_index + 1usize,
                })
            }
        };
    }
}

fn compare(items: &[Item], left_index: usize, right_index: usize) -> usize {
    let left = items[left_index].weight;
    let right = items[right_index].weight;

    if left < right {
        right_index
    } else if left > right {
        left_index
    } else {
        (left_index + right_index) / 2usize
    }
}

fn follow_heavier_branch(
    items: &[Item],
    start_index_ref: &mut usize,
    end_index_ref: &mut usize,
) -> Result<(), ()> {
    let start_index = *start_index_ref;
    let end_index = *end_index_ref;

    let left_half = (start_index + end_index) / 2usize;
    let right_half = left_half + 1usize;

    let left = &items[start_index..=left_half];
    let right = &items[right_half..=end_index];

    let left_sum: f32 = sum(left);
    let right_sum: f32 = sum(right);

    if left_sum < right_sum {
        *start_index_ref = right_half;
        *end_index_ref = end_index;
        return Ok(());
    }

    if left_sum > right_sum {
        *start_index_ref = start_index;
        *end_index_ref = left_half;
        return Ok(());
    }

    // when odd items were corrected, possibly odd
    // out one is heavier, thus branches are equal
    return Err(());
}

fn sum(items: &[Item]) -> f32 {
    items.iter().map(|x| x.weight).sum()
}

#[cfg(test)]
mod unit_test {
    use super::*;

    fn create_items(source: &[u32]) -> Vec<Item> {
        Item::create_items_from_u32(source)
    }

    #[test]
    fn sum_test() {
        let nums = create_items(&[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(28f32, sum(nums.as_slice()));
    }

    #[test]
    fn compare_last_is_heavier() {
        let items = create_items(&[1, 2, 3]);
        assert_eq!(2usize, compare(&items, 0usize, 2usize));
    }

    #[test]
    fn compare_first_is_heavier() {
        let items = create_items(&[3, 2, 1]);
        assert_eq!(0usize, compare(&items, 0usize, 2usize));
    }

    #[test]
    fn compare_middle_is_predicated() {
        let items = create_items(&[1, 1, 1]);
        assert_eq!(1usize, compare(&items, 0usize, 2usize));
    }

    #[test]
    fn follow_heavier_branch_test1() {
        let items = create_items(&[1, 1, 1, 2]);

        let mut left = 0usize;
        let mut right = 3usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Ok(()), result);
        assert_eq!(2usize, left);
        assert_eq!(3usize, right);
    }

    #[test]
    fn follow_heavier_branch_test2() {
        let items = create_items(&[1, 1, 2, 1]);

        let mut left = 0usize;
        let mut right = 3usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Ok(()), result);
        assert_eq!(2usize, left);
        assert_eq!(3usize, right);
    }

    #[test]
    fn follow_heavier_branch_test3() {
        let items = create_items(&[1, 2, 1, 1]);

        let mut left = 0usize;
        let mut right = 3usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Ok(()), result);
        assert_eq!(0usize, left);
        assert_eq!(1usize, right);
    }

    #[test]
    fn follow_heavier_branch_test4() {
        let items = create_items(&[2, 1, 1, 1]);

        let mut left = 0usize;
        let mut right = 3usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Ok(()), result);
        assert_eq!(0usize, left);
        assert_eq!(1usize, right);
    }

    #[test]
    fn follow_heavier_branch_test5() {
        let items = create_items(&[1, 2, 1, 1, 1, 1]);

        let mut left = 0usize;
        let mut right = 5usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Ok(()), result);
        assert_eq!(0usize, left);
        assert_eq!(2usize, right);
    }

    #[test]
    fn follow_heavier_branch_test6() {
        let items = create_items(&[1, 1, 1, 1, 2, 1]);

        let mut left = 0usize;
        let mut right = 5usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Ok(()), result);
        assert_eq!(3usize, left);
        assert_eq!(5usize, right);
    }

    #[test]
    fn follow_heavier_branch_nothing_to_follow() {
        let items = create_items(&[1, 1, 1, 1, 1, 1]);

        let mut left = 0usize;
        let mut right = 5usize;

        let result = follow_heavier_branch(&items, &mut left, &mut right);

        assert_eq!(Err(()), result);
        assert_eq!(0usize, left);
        assert_eq!(5usize, right);
    }
}
