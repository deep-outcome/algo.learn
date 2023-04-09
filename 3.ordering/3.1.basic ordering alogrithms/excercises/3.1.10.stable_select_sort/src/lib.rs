use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use datamanner::Item;

pub mod datamanner;
pub mod selectsort;
pub mod selectsort2;

pub fn wrappy_test(arr: &mut [i32], print_encounterments: bool) {
    let mut criterion = arr.to_vec();

    criterion.sort();

    print!("\nWRAPPY TEST");

    let mut items = datamanner::into_items(arr);

    println!(" | orig   {:?}\n", items);

    selectsort::sort(&mut items, print_encounterments);
//     selectsort2::sort(&mut items, print_encounterments);

    println!("WRAPPY TEST | sorted {:?}\n", items);

    verify_original_order(&items);
    assert_eq!(criterion, datamanner::extract_values(&items));

    print!("------------------------------------");
}

pub fn verify_original_order<T>(items: &Vec<Item<T>>)
where
    T: Copy + Display + Eq + Hash,
{
    let mut value_stability_mapping: HashMap<T, Vec<datamanner::Item<T>>> =
        HashMap::with_capacity(items.len());

    for i in items {
        let i = *i;
        let key = i.item();
        match value_stability_mapping.get_mut(&key) {
            Some(list) => list.push(i),
            None => {
                let mut list = Vec::new();
                list.push(i);
                value_stability_mapping.insert(key, list);
            }
        }
    }

    for mapping in value_stability_mapping {
        let values = mapping.1;

        if values.len() == 1 {
            continue;
        }

        let mut order_aux = 0usize;
        for val in values {
            let val = val.original_order();
            assert!(val + 1 > order_aux, "On key {}!", mapping.0);
            order_aux = val;
        }
    }
}
