use crate::linked_list::serie::Serie;
use std::cmp::Ordering;
use std::panic;

pub fn test() {
    wrappy_test(&mut [2, 1]);
    wrappy_test(&mut [1, 2, 3, 4, 5, 6]);
    wrappy_test(&mut [5, 4, 3, 2]);

    wrappy_test(&mut [89, 8, 1, -23, 13, 22, 76]);
    wrappy_test(&mut [89, -23, 1, 8, 22, 76]);
    wrappy_test(&mut [-101, -102, 13, 2, 1, 56, 55]);
    wrappy_test(&mut [100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102]);
}

fn wrappy_test(unordered: &[i32]) {
    let mut criterion = unordered.to_vec();

    criterion.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    println!();
    print!("WRAPPY TEST : len {}", unordered.len());
    println!(" | orig {:?}", unordered);

    let mut serie = Serie::from(unordered);

    println!("{:?}", serie);
    println!();

    serie.check_counts(&unordered, false);

    unsafe {
        let serie: *mut Serie = &mut serie;
        sort(&mut *serie);
    }

    println!("resulted {:?}", serie);
    let test: Vec<i32> = serie.into();
    assert_eq!(criterion, test)
}

pub fn sort<'a>(serie: &mut Serie<'a>) {
    let len = serie.len();

    if len < 2usize {
        return;
    }

    let mut inner_loop_counter = 0usize;
    let mut outter_loop_counter = 0usize;

    let mut accomodated_len = len;
    let mut accomodated_left_index = 0usize;

    loop {
        if accomodated_len == 0usize {
            break;
        }

        let mut left_index = accomodated_left_index;
        let mut right_index = left_index + 1usize;

        let end = accomodated_len;

        accomodated_len = 0usize;

        while right_index < end {
            let number = serie[left_index];
            let next = serie[right_index];
            if next < number {
                if let Err(e) = serie.swap_with_next(left_index) {
                    panic!("{}", e);
                }

                if accomodated_len == 0usize {
                    // OPT on start index
                    if left_index > 0usize {
                        accomodated_left_index = left_index - 1;
                    }
                }

                // OPT on end index
                accomodated_len = right_index;
                println!("swapped {}, {:?}", number, serie);
            }

            left_index = left_index + 1usize;
            right_index = right_index + 1usize;

            inner_loop_counter = inner_loop_counter + 1usize;
        }

        outter_loop_counter = outter_loop_counter + 1usize;
    }

    println!("inner loops encountered  | {}", inner_loop_counter);
    println!("outter loops encountered | {}", outter_loop_counter);
}
