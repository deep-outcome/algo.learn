#![allow(dead_code)]

use std::mem;
use std::ptr;

#[cfg(test)]
mod test {
    use super::lexbucketsort;

    #[test]
    fn basic_test() {
        let mut items: [(&[usize; 4], char); 6] = [
            (&[9, 2, 8, 7], 'x'),
            (&[9, 2, 8, 1], 'z'),
            (&[3, 2, 8, 7], 'a'),
            (&[3, 2, 7, 8], 'x'),
            (&[3, 2, 7, 8], 'a'),
            (&[2, 3, 9, 10], 'x'),
        ];

        let mut criterion = items.to_vec();
        criterion.sort_by(|x, y| x.0.cmp(y.0));

        lexbucketsort(&mut items, 10);

        assert_eq!(criterion, items.to_vec());
    }
}

type LexKeyAnyVal<'a, const N: usize, T> = (&'a [usize; N], T);

fn lexbucketsort<T, const N: usize>(items: &mut [LexKeyAnyVal<N, T>], max: usize) {
    let items_len = items.len();
    if items_len < 2 {
        return;
    }

    let mut key = items[0].0.len();
    while key > 0 {
        key -= 1;
        bucketsort(items, max, key);
    }
}

fn bucketsort<T, const N: usize>(items: &mut [LexKeyAnyVal<N, T>], max: usize, key: usize) {
    let buckets_len = max + 1;
    let mut buckets = Vec::with_capacity(buckets_len);
    let buckets_populator = buckets.spare_capacity_mut();

    for i in 0..=max {
        buckets_populator[i].write(Vec::with_capacity(0));
    }

    unsafe {
        buckets.set_len(buckets_len);
    }

    let mut items_ptr = items.as_ptr();

    let items_len = items.len();
    for _ in 0..items_len {
        let item: LexKeyAnyVal<N, T>;
        unsafe {
            item = items_ptr.read(); // let assume that LexKeyAnyVal is somewhat small in bytes
        }
        let bucket_key = item.0[key];

        if bucket_key < buckets_len {
            buckets[bucket_key].push(item);
            unsafe {
                items_ptr = items_ptr.add(1);
            }
        } else {
            free_duplicities(buckets);
            panic!("Key not in available buckets.")
        }
    }

    let mut items_ptr = items.as_mut_ptr();

    let mut writes_count = 0;
    for buc in buckets.iter() {
        let buc_len = buc.len();

        if buc_len > 0 {
            let mut buc_ptr = buc.as_ptr();

            for _ in 0..buc_len {
                unsafe {
                    ptr::write(items_ptr, buc_ptr.read());
                    buc_ptr = buc_ptr.add(1);
                    items_ptr = items_ptr.add(1);
                }

                writes_count += 1;
            }

            if writes_count == items_len {
                break;
            }
        }
    }

    free_duplicities(buckets);

    fn free_duplicities<const N: usize, T>(mut bucs: Vec<Vec<LexKeyAnyVal<N, T>>>) {
        while let Some(mut b) = bucs.pop() {
            while let Some(i) = b.pop() {
                mem::forget(i);
            }
        }
    }
}
