use super::datamanner::Item;
use std::fmt::Display;

pub fn sort<T>(items: &mut Vec<Item<T>>, print_encounterments: bool)
where
    T: Copy + PartialOrd + Display,
{
    let len = items.len();

    if len < 2 {
        return;
    }

    let mut inner_loop_counter = 0usize;
    let mut outter_loop_counter = 0usize;

    let mut stability_indexes: Vec<usize> = Vec::with_capacity(len / 2);

    let corrected_len = len - 1usize;

    let mut left_index = 0usize;
    while left_index < corrected_len {
        let mut min = items[left_index];
        let mut min_index = left_index;

        let original = min;

        let mut right_index = left_index + 1usize;
        while right_index < len {
            let some = items[right_index];

            if some < min {
                min = some;
                min_index = right_index;
            }

            if some == original {
                stability_indexes.push(right_index);
            }

            right_index = right_index + 1usize;
            inner_loop_counter = inner_loop_counter + 1usize;
        }

        if min_index != left_index {
            if stability_indexes.len() > 0 {
                //                 datamanner::debug_pring(&items);
                //                 println!("{:?}", stability_indexes);

                stability_indexes.push(min_index);
                //             stability_indexes.push(left_index);
                let iter = stability_indexes.iter().filter(|x| **x <= min_index);

                let mut outter_swap = items[left_index];
                for index_ref in iter {
                    let index = *index_ref;
                    let swap = items[index];
                    items[index] = outter_swap;
                    outter_swap = swap;

                    //                     super::datamanner::debug_pring(&items);
                }
            //                    println!();
            } else {
                items[min_index] = items[left_index];

                //                             super::datamanner::debug_pring(&items);
            }
            items[left_index] = min;
        }

        left_index = left_index + 1usize;
        stability_indexes.clear();

        outter_loop_counter = outter_loop_counter + 1usize;
    }

    if print_encounterments {
        println!("    SELECTSORT encounterements");
        println!("      inner loops  | {}", inner_loop_counter);
        println!("      outter loops | {}", outter_loop_counter);
    }
}
