#[test]
pub fn test() {
    wrappy_test(&mut [6, 5, 8, 7, 2, 1, 4, 3]);
    wrappy_test(&mut [8, 7, 6, 5, 4, 3, 2, 1]);

    wrappy_test(&mut [1, 2, 3, 4, 5, 6]);

    wrappy_test(&mut [89, -23, 1, 8, 22, 76]);
    wrappy_test(&mut [-101, -102, 13, 2, 1, 56, 55]);
    wrappy_test(&mut [100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102]);
    wrappy_test(&mut [
        100, 1, 100, 5, 5, 6, 2, 8, -2, 8, 100, 100, 3, 3, 101, 102, 44, 43,
    ]);
    wrappy_test(&mut [
        100, 1, 100, 5, 5, 6, 2, 8, -2, 8, 100, 100, 3, 3, 101, 102, 44, 43, 11, 55, 99, 18, 33,
        23, 14, 87, 90, 1, 5, 6, 8, 94,
    ]);
}

fn wrappy_test(arr: &mut [i32]) {
    println!("\n>> MERGESORT << INITIATED --");

    let mut criterion = arr.to_vec();
    criterion.sort();

    println!("len {}", arr.len());
    println!("{:?}\n", arr);
    in_place_sorting_mergesort::bank_buffered_mergesort::sort(arr);
    println!("{:?}", arr);

    assert_eq!(criterion, arr)
}
