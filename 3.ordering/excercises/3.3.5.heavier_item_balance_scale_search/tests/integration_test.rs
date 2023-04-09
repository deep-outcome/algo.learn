#![allow(non_snake_case)]

use heavier_item_balance_scale_search::find_heavier_one;
use heavier_item_balance_scale_search::Item;

#[test]
fn validation_test__not_enough_items() {
    let source: Vec<Vec<u32>> = vec![vec![], vec![0]];

    for vec in source {
        let items = Item::create_items_from_u32(&vec);
        let result = find_heavier_one(&items);

        match result {
            Err(err) => assert_eq!("Expected at least 2 items.", err),
            Ok(_) => assert!(false),
        };
    }
}

#[test]
fn validation_test__same_weight_items_only() {
    let source = [1.1, 1.1].iter().map(|x| *x);
    let items = Item::create_items_from_f32(source);

    let result = find_heavier_one(&items);

    match result {
        Err(err) => assert_eq!("No heavier item found.", err),
        Ok(_) => assert!(false),
    };
}

#[test]
fn validation_test__more_than_2_different_weight_kinds() {
    let source = [1.0, 1.1, 1.2].iter().map(|x| *x);
    let items = Item::create_items_from_f32(source);

    let result = find_heavier_one(&items);

    match result {
        Err(err) => assert_eq!("More than 2 weight kinds found.", err),
        Ok(_) => assert!(false),
    };
}

#[test]
// same logic tests whether lighter item is provided
// because in turn normal ones get heavier
fn validation_test__more_than_1_heavier_item() {
    let source = [1.0, 1.2, 1.2].iter().map(|x| *x);
    let items = Item::create_items_from_f32(source);

    let result = find_heavier_one(&items);

    match result {
        Err(err) => assert_eq!("Expected exactly 1 heavier item amond rest.", err),
        Ok(_) => assert!(false),
    };
}

#[test]
fn search_test__shortcut_sized_items() {
    let source = vec![
        (vec![1.2, 1.1, 1.1], 0), // first heavier
        (vec![1.1, 1.2, 1.1], 1), // middle heavier
        (vec![1.1, 1.1, 1.2], 2), // last heavier
    ];

    for kv in source {
        let source = kv.0.iter().map(|x| *x);

        let items = Item::create_items_from_f32(source);
        let result = find_heavier_one(&items);

        match result {
            Err(_) => assert!(false),
            Ok(f) => assert_eq!(kv.1, f.get_index()),
        };
    }
}

use rand::Rng;
use std::iter;
#[test]
fn search_test__pow2_sized_items1() {
    let size = 2i32.pow(12);
    let mut counter = size;

    let normals = iter::repeat(1)
        .take_while(|_| {
            counter = counter - 1i32;
            counter > -1i32
        })
        .collect::<Vec<u32>>();

    let mut source = normals;

    let rnd_ix = rand::thread_rng().gen_range(0..size) as usize;

    source[rnd_ix] = 2u32;

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(rnd_ix, f.get_index()),
    };
}

#[test]
fn search_test__pow2_sized_items2() {
    let source = [
        2, 1, 1, 1, /**/ 1, 1, 1, 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, //
    ];

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(0, f.get_index()),
    };
}

#[test]
fn search_test__even_not_pow2_sized_items1() {
    let size = 36i32;
    let mut counter = size;

    let normals = iter::repeat(1)
        .take_while(|_| {
            counter = counter - 1i32;
            counter > -1i32
        })
        .collect::<Vec<u32>>();

    let mut source = normals;

    let rnd_ix = rand::thread_rng().gen_range(0..size) as usize;

    source[rnd_ix] = 2;

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(rnd_ix, f.get_index()),
    };
}

#[test]
fn search_test__even_divided_to_odd_sized_items() {
    let source = [
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 2, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
    ];

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(17, f.get_index()),
    };
}

#[test]
fn search_test__final_compare_2items() {
    let source = [
        2, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
        1, 1, 1, 1, /**/ 1, 1, 1, 1, /**/ 1, //
    ];

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(0usize, f.get_index()),
    };
}

#[test]
fn search_test__final_compare_3items() {
    let source = [
        2, 1, 1, /**/ 1, 1, 1, //
        1, 1, 1, /**/ 1, 1, 1, //
    ];

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(0usize, f.get_index()),
    };
}

#[test]
fn search_test__odd_input_with_last_heavier() {
    let source = [
        1, 1, 1, /**/ 1, 1, 1, //
        1, 1, 1, /**/ 1, 2, //
    ];

    let items = Item::create_items_from_u32(&source);
    let result = find_heavier_one(&items);

    match result {
        Err(_) => assert!(false),
        Ok(f) => assert_eq!(10, f.get_index()),
    };
}
