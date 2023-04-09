fn main() {
    test();
}

pub fn test() {
    wrappy_test(&mut [2, 1]);
    wrappy_test(&mut [1, 2, 3, 4, 5, 6]);
    wrappy_test(&mut [5, 4, 3, 2]);

    wrappy_test(&mut [89, 8, 1, -23, 13, 22, 76]);
    wrappy_test(&mut [89, -23, 1, 8, 22, 76]);
    wrappy_test(&mut [-101, -102, 13, 2, 1, 56, 55]);
    wrappy_test(&mut [100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102]);

    wrappy_test(&mut [5, 8, 9, 5, 3, 9, 8, 3, 7]);
    wrappy_test(&mut [5, 8, 9, 5, 3, 5, 9, 5, 8, 7]);
}

fn wrappy_test(arr: &mut [i32]) {
    stable_select_sort::wrappy_test(arr, true);
}
