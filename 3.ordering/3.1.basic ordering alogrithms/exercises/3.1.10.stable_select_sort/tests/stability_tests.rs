use stable_select_sort::datamanner::Item;
use stable_select_sort::datamanner;
use stable_select_sort::selectsort;
use stable_select_sort::selectsort2;
use stable_select_sort::verify_original_order;

#[test]
fn simple_shift_test() {
    let arr = [2, 2, 1];
    let mut items = datamanner::into_items(&arr);
    sort(&mut items);

    let mut criterion = arr.to_vec();
    criterion.sort();

    assert_eq!(criterion, datamanner::extract_values(&items));
    verify_original_order(&items);
}

#[test]
// little doubtful
fn shift_stops_on_hole() {
    // hole emerges from minimum being swapped to front
    let arr = [2, 2, 1, 2, 2];
    let mut items = datamanner::into_items(&arr);
    sort(&mut items);

    let mut criterion = arr.to_vec();
    criterion.sort();

    assert_eq!(criterion, datamanner::extract_values(&items));
    verify_original_order(&items);

    assert_eq!(items[3].original_order(), 3);
    assert_eq!(items[4].original_order(), 4);
}

#[test]
fn more_complexified_shift_test() {
    let arr = [2, 3, 4, 3, 2, 1];
    let mut items = datamanner::into_items(&arr);
    sort(&mut items);

    let mut criterion = arr.to_vec();
    criterion.sort();

    assert_eq!(criterion, datamanner::extract_values(&items));
    verify_original_order(&items);
}

fn sort (items: &mut Vec<Item<i32>>) {
//     selectsort::sort(items, false);
    selectsort2::sort(items, false);
}
