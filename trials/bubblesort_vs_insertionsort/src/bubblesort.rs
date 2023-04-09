pub fn sort(arr: &mut [i32]) {
    let len = arr.len();

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
            let number = arr[left_index];
            let next = arr[right_index];
            if next < number {
                arr[left_index] = next;
                arr[right_index] = number;

                if accomodated_len == 0usize {
                    // OPT on start index
                    if left_index > 0usize {
                        accomodated_left_index = left_index - 1;
                    }
                }

                // OPT on end index
                accomodated_len = right_index;
            }

            left_index = left_index + 1usize;
            right_index = right_index + 1usize;

            inner_loop_counter = inner_loop_counter + 1usize;
        }

        outter_loop_counter = outter_loop_counter + 1usize;
    }

    println!("    BUBBLESORT encounterements");
    println!("      inner loops  | {}", inner_loop_counter);
    println!("      outter loops | {}", outter_loop_counter);
}
