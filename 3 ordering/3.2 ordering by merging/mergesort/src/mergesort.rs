use std::cmp::{min, Ordering};

pub fn test() {
    //     wrappy_test(&mut [1, 2, 3, 4, 5, 6]);
    //     wrappy_test(&mut [5, 4, 3, 2]);

    wrappy_test(&mut [89, -23, 1, 8, 22, 76]);
    wrappy_test(&mut [-101, -102, 13, 2, 1, 56, 55]);
    wrappy_test(&mut [100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102]);
}

fn wrappy_test(array: &mut [i32]) {
    let mut test = array.to_vec();
    test.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    println!();
    println!("len {}", array.len());
    println!("{:?}", array);
    sort(array);
    println!("{:?}", array);

    assert_eq!(test, array)
}

pub fn sort(array: &mut [i32]) {
    let len = array.len();
    let mut aux: Vec<i32> = Vec::with_capacity(len);

    let mut block_size = 1;

    while block_size < len {
        println!("block size {}", block_size);

        let normalized_merged_block_size = block_size * 2;

        let mut left_index = 0;
        let mut right_index = block_size;

        while right_index < len {
            let right_exclusive_end = min(right_index + block_size, len);

            merge(
                left_index,
                right_index,
                right_exclusive_end,
                &array,
                &mut aux,
            );

            let aux_len = aux.len();
            for i in (left_index..left_index + aux_len).zip(aux.drain(0..aux_len)) {
                println!("i0 {}, i1 {}", i.0, i.1);
                array[i.0] = i.1;
            }

            println!();

            left_index = left_index + normalized_merged_block_size;
            right_index = right_index + normalized_merged_block_size;
        }

        block_size = block_size * 2;
    }
}

fn merge(
    left_index: usize,
    right_index: usize,
    right_exclusive_end: usize,
    array: &[i32],
    aux: &mut Vec<i32>,
) {
    let mut left_index: usize = left_index;
    let mut right_index: usize = right_index;

    let left_exclusive_end: usize = right_index;

    while left_index < left_exclusive_end && right_index < right_exclusive_end {
        let left = array[left_index];
        let right = array[right_index];

        if right < left {
            push_item(right, &mut right_index, aux);
        } else {
            push_item(left, &mut left_index, aux);
        }
    }

    while left_index < left_exclusive_end {
        push_item_of_index(&mut left_index, &array, aux);
    }

    while right_index < right_exclusive_end {
        push_item_of_index(&mut right_index, &array, aux);
    }
}

fn push_item_of_index(index: &mut usize, array: &[i32], aux: &mut Vec<i32>) {
    let index_value = *index;
    let item = array[index_value];
    aux.push(item);
    *index = index_value + 1;
}

fn push_item(item: i32, index: &mut usize, aux: &mut Vec<i32>) {
    aux.push(item);
    *index = *index + 1;
}
