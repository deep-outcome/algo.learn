pub fn sort(arr: &mut [i32]) {
    let len = arr.len();

    if len < 2usize {
        return;
    }

    let mut inner_loop_counter = 0usize;
    let mut outter_loop_counter = 0usize;

    let mut global = 1usize;

    while global < len {
        let mut local = global - 1usize;

        loop {
            inner_loop_counter = inner_loop_counter + 1usize;

            let next_to_local = local + 1usize;

            let left = arr[local];
            let right = arr[next_to_local];

            if left < right {
                break;
            }

            arr[next_to_local] = left;
            arr[local] = right;

            if local == 0 {
                break;
            }

            local = local - 1usize;
        }

        global = global + 1usize;

        outter_loop_counter = outter_loop_counter + 1usize;
    }

    println!("    INSERTIONSORT encounterements");
    println!("      inner loops  | {}", inner_loop_counter);
    println!("      outter loops | {}", outter_loop_counter);
}
