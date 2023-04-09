#![feature(test)]

use std::cmp::Ordering;

mod bubblesort;
mod insertionsort;

mod bench;

fn main() {
    test();
}

pub fn test() {
    wrappy_test(&[2, 1]);
    wrappy_test(&[1, 2, 3, 4, 5, 6]);
    wrappy_test(&[5, 4, 3, 2]);

    wrappy_test(&[89, 8, 1, -23, 13, 22, 76]);
    wrappy_test(&[89, -23, 1, 8, 22, 76]);
    wrappy_test(&[-101, -102, 13, 2, 1, 56, 55]);
    wrappy_test(&[100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102]);
}

fn wrappy_test(arr: &[i32]) {
    let for_bubblesort = arr.iter().map(|x| *x).collect::<Vec<i32>>();
    let mut for_bubblesort = for_bubblesort.into_boxed_slice();
    let for_insertionsort = arr.iter().map(|x| *x).collect::<Vec<i32>>();
    let mut for_insertionsort = for_insertionsort.into_boxed_slice();

    let mut criterion = arr.to_vec();

    criterion.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    print!("\nWRAPPY TEST");
    println!(" | orig {:?}\n", arr);

    bubblesort::sort(for_bubblesort.as_mut());
    insertionsort::sort(for_insertionsort.as_mut());

    assert_eq!(criterion, for_bubblesort.to_vec());
    assert_eq!(criterion, for_insertionsort.to_vec());

    println!("------------------------------------------------------------");
}
