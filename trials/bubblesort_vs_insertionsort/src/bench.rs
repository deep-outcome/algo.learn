#![allow(dead_code)]

extern crate test;

use super::bubblesort;
use super::insertionsort;

use test::bench::Bencher;

#[bench]
fn bubblesort_bench_data(b: &mut Bencher) {
    bubblesort_bench(&mut [2, 1], b);
    bubblesort_bench(&mut [1, 2, 3, 4, 5, 6], b);
    bubblesort_bench(&mut [5, 4, 3, 2], b);

    bubblesort_bench(&mut [89, 8, 1, -23, 13, 22, 76], b);
    bubblesort_bench(&mut [89, -23, 1, 8, 22, 76], b);
    bubblesort_bench(&mut [-101, -102, 13, 2, 1, 56, 55], b);
    bubblesort_bench(&mut [100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102], b);
}

#[bench]
fn insertionsort_bench_data(b: &mut Bencher) {
    insertionsort_bench(&mut [2, 1], b);
    insertionsort_bench(&mut [1, 2, 3, 4, 5, 6], b);
    insertionsort_bench(&mut [5, 4, 3, 2], b);

    insertionsort_bench(&mut [89, 8, 1, -23, 13, 22, 76], b);
    insertionsort_bench(&mut [89, -23, 1, 8, 22, 76], b);
    insertionsort_bench(&mut [-101, -102, 13, 2, 1, 56, 55], b);
    insertionsort_bench(&mut [100, 1, 3, 5, 5, 6, 2, -2, 8, 101, 102], b);
}

fn bubblesort_bench(arr: &mut [i32], b: &mut Bencher) {
    b.iter(|| bubblesort::sort(arr));
}

fn insertionsort_bench(arr: &mut [i32], b: &mut Bencher) {
    b.iter(|| insertionsort::sort(arr));
}
