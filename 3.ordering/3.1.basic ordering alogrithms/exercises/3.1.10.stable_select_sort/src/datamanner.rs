use std::cmp::Ordering;
use std::fmt::Result as fResult;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone)]
pub struct Item<T> {
    item: T,
    original_order: usize,
}

impl<T> Item<T>
where
    T: Copy,
{
    pub fn item(&self) -> T {
        self.item
    }

    pub fn original_order(&self) -> usize {
        self.original_order
    }
}

impl<T> PartialEq for Item<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Item<T>) -> bool {
        self.item.eq(&other.item)
    }

    fn ne(&self, other: &Item<T>) -> bool {
        self.item.ne(&other.item)
    }
}

impl<T> PartialOrd for Item<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Item<T>) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

impl<T> Eq for Item<T> where T: Eq {}

impl<T> Ord for Item<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Item<T>) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl<T> Debug for Item<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        f.write_str("{")?;
        f.write_str(&self.item.to_string())?;
        f.write_str("(")?;
        f.write_str(&self.original_order.to_string())?;
        f.write_str(")}")?;

        Ok(())
    }
}

pub fn into_items<T>(source: &[T]) -> Vec<Item<T>>
where
    T: Copy,
{
    let len = source.len();

    let mut items = Vec::with_capacity(len);

    if len == 0 {
        return items;
    }

    let mut index = 0;
    for t in source {
        items.push(Item {
            item: *t,
            original_order: index,
        });

        index = index + 1;
    }

    items
}

pub fn extract_values<T>(source: &Vec<Item<T>>) -> Vec<T>
where
    T: Copy,
{
    let len = source.len();
    let mut result = Vec::with_capacity(len);

    if len == 0 {
        return result;
    }

    for i in source {
        result.push(i.item);
    }

    result
}

pub fn debug_pring<T: Display>(items: &Vec<Item<T>>) {
    let mut str = String::from("[");

    let mut index = -1i32;
    for i in items {
        index = index + 1i32;
        str = str + &format!("[{}]{:?}, ", index, i);
    }

    println!("{}]", str.trim_end().trim_end_matches(','));
}
