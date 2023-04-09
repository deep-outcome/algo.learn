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

    let corrected_len = len - 1usize;

    let mut left_index = 0usize;
    while left_index < corrected_len {
        let mut min = items[left_index];
        let mut min_index = left_index;

        let mut right_index = left_index + 1usize;
        while right_index < len {
            let some = items[right_index];

            if some < min {
                min = some;
                min_index = right_index;
            }

            right_index = right_index + 1usize;
            inner_loop_counter = inner_loop_counter + 1usize;
        }

        let mut move_index = min_index;
        while left_index < move_index {
            let previous_index = move_index- 1usize;
            items[move_index] = items[previous_index];
            move_index = previous_index;
        }
        
        items[left_index] = min;
        
        left_index = left_index + 1usize;
        
        outter_loop_counter = outter_loop_counter + 1usize;
    }

    if print_encounterments {
        println!("    SELECTSORT encounterements");
        println!("      inner loops  | {}", inner_loop_counter);
        println!("      outter loops | {}", outter_loop_counter);
    }
}
