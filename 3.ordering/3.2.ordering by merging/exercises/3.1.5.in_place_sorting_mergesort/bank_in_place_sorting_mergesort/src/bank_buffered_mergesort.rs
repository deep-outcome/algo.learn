use std::cmp::min;
use std::fmt::{Debug, Display, Error, Formatter};
use std::result::Result;

pub fn sort(arr: &mut [i32]) {
    let len = arr.len();

    let mut block_size = 1;

    while block_size < len {
        println!("\nBlock sizing {}", block_size);

        let normalized_merged_block_size = block_size * 2;

        let mut left_index = 0;
        let mut right_index = block_size;

        while right_index < len {
            let right_exclusive_end = min(right_index + block_size, len);

            let indent = String::from("  ");
            print!("{}", indent);
            merge(left_index, right_index, right_exclusive_end, arr, indent);

            left_index = left_index + normalized_merged_block_size;
            right_index = right_index + normalized_merged_block_size;
        }

        block_size = block_size * 2;
    }
}

fn merge(
    l_index: usize,
    r_index: usize,
    r_exclusor: usize,
    arr: &mut [i32],
    indent: String,
) -> MergeResult {
    println!("MERGE START {:?}", arr);

    let mut l_reader: usize = l_index;
    let mut r_reader: usize = r_index;

    let mut arr_writer: usize = l_index;

    let mut l_displaced_reader = l_reader;

    let l_exclusor: usize = r_index;

    let mut swap;

    let mut tail_finmerge_start_index = r_index;

    while l_reader < l_exclusor && r_reader < r_exclusor {
        println!("{}{}-- MAIN loop", indent, indent);

        let left = arr[l_displaced_reader];
        let right = arr[r_reader];

        if left < right {
            if r_reader > r_index {
                swap = arr[arr_writer];
                arr[arr_writer] = left;

                arr[l_displaced_reader] = swap;

                if l_displaced_reader == r_reader - 1usize {
                    l_displaced_reader = r_index - 1usize;
                }
            }

            l_displaced_reader = l_displaced_reader + 1usize;
        } else {
            swap = arr[arr_writer];
            arr[arr_writer] = right;
            arr[r_reader] = swap;

            if l_displaced_reader == arr_writer {
                l_displaced_reader = r_reader;
            }

            tail_finmerge_start_index = arr_writer + 1usize;

            r_reader = r_reader + 1usize;
        }

        l_reader = l_reader + 1usize;
        arr_writer = arr_writer + 1usize;
    }

    if r_index == r_reader {
        println!("{}MERGE RESULT >>> {}\n", indent, MergeResult::NoAction);
        return MergeResult::NoAction;
    }

    let balanced_blocks: bool = r_exclusor - r_index == l_exclusor - l_index;

    if r_reader == r_exclusor && balanced_blocks {
        println!("{}MERGE RESULT >>> {}\n", indent, MergeResult::SwapAction);
        return MergeResult::SwapAction;
    }

    let new_indent = format!("{}{}", indent, indent);

    print!("{}BANK ", new_indent);
    merge(
        r_index,
        l_displaced_reader,
        r_reader,
        arr,
        new_indent.clone(),
    );

    if r_reader != r_exclusor {
        print!("{}RIGHT BLOCK ACCOMPLISHING ", new_indent);
        merge(r_index, r_reader, r_exclusor, arr, new_indent.clone());
    }

    if !balanced_blocks {
        print!("{}JOINT BLOCK ACCOMPLISHING ", new_indent);
        merge(
            tail_finmerge_start_index,
            r_index,
            r_reader,
            arr,
            new_indent,
        );
    }

    MergeResult::NonSpecific
}

#[derive(Copy, Clone)]
#[repr(usize)]
enum MergeResult {
    NonSpecific = 1,
    NoAction = 2,
    SwapAction = 3,
}

impl PartialEq for MergeResult {
    fn eq(&self, other: &MergeResult) -> bool {
        *self as usize == *other as usize
    }
}

impl Display for MergeResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let output = match self {
            MergeResult::NonSpecific => "MERGED",
            MergeResult::NoAction => "MUTUALLY SORTED BY MERGE START",
            MergeResult::SwapAction => "BLOCK SWAP",
        };

        _ = f.write_str(output);
        Ok(())
    }
}

impl Debug for MergeResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let output = match self {
            MergeResult::NonSpecific => "NonSpecific",
            MergeResult::NoAction => "NoAction",
            MergeResult::SwapAction => "SwapAction",
        };

        _ = f.write_str(output);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{merge, MergeResult};

    #[test]
    fn block_swap() {
        let mut subject = [3, 4, 1, 2];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 2usize, 4usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::SwapAction, result, "{}", result);
    }

    #[test]
    fn no_action() {
        let mut subject = [1, 2, 3, 4];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 2usize, 4usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::NoAction, result, "{}", result);
    }

    #[test]
    fn right_rest_merging() {
        let mut subject = [5, 7, 8, 10, 1, 2, 6, 9];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 4usize, 8usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::NonSpecific, result, "{}", result);
    }

    #[test]
    fn bank_merging() {
        let mut subject = [5, 7, 8, 9, 1, 2, 6, 10];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 4usize, 8usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::NonSpecific, result, "{}", result);
    }

    #[test]
    fn right_rest_merging_unbalanced() {
        let mut subject = [3, 4, 999, 1001, 1, 2, 1000];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 4usize, 7usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::NonSpecific, result, "{}", result);
    }

    #[test]
    fn bank_merging_unbalanced() {
        let mut subject = [3, 5, 999, 1000, 1, 2, 4];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 4usize, 7usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::NonSpecific, result, "{}", result);
    }

    #[test]
    fn balancing() {
        let mut subject = [3, 6, 7, 8, 4, 5];
        let mut criterion = subject.to_vec();
        criterion.sort();

        let result = merge(0usize, 4usize, 6usize, &mut subject, String::from("_"));

        assert_eq!(criterion, subject);
        assert_eq!(MergeResult::NonSpecific, result, "{}", result);
    }
}
