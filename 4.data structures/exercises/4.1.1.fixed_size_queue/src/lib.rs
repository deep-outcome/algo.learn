#![allow(dead_code)]

struct FixedSizeQueue<T, const I: usize> {
    arr: [T; I],
    rix: usize,
    wix: usize,
    count: usize,
}

impl<T, const CAP: usize> FixedSizeQueue<T, CAP>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        FixedSizeQueue {
            arr: [T::default(); CAP],
            rix: 0,
            wix: 0,
            count: 0,
        }
    }

    pub fn enque(&mut self, t: T) -> bool {
        let count = self.count;
        if count == CAP {
            return false;
        }

        let wix = self.wix;
        self.arr[wix] = t;

        self.wix = (wix + 1) % CAP;
        self.count = count + 1;

        true
    }

    pub fn deque(&mut self) -> Option<T> {
        let count = self.count;
        if count == 0 {
            return None;
        }

        let rix = self.rix;
        let t = self.arr[rix];

        self.rix = (rix + 1) % CAP;
        self.count = count - 1;

        Some(t)
    }
}

#[cfg(test)]
mod tests_of_units {
    use super::FixedSizeQueue;

    // new()
    #[test]
    fn new() {
        let queue = FixedSizeQueue::<u8, 17>::new();
        assert_eq!([0u8; 17], queue.arr);
        assert_eq!(0, queue.rix);
        assert_eq!(0, queue.wix);
        assert_eq!(0, queue.count);
    }

    // enque()
    #[test]
    fn enque_basic_test() {
        let mut queue = FixedSizeQueue::<u8, 2>::new();

        const ENQ: u8 = 3;

        assert!(queue.enque(ENQ));
        assert_eq!(1, queue.wix);
        assert_eq!(1, queue.count);
        assert_eq!(ENQ, queue.arr[0]);
    }

    #[test]
    fn enque_full_test1() {
        let mut queue = FixedSizeQueue::<u8, 0>::new();

        let deq = queue.deque();
        assert!(deq.is_none());
    }

    #[test]
    fn enque_full_test2() {
        let mut queue = FixedSizeQueue::<u8, 1>::new();

        _ = queue.enque(1);
        assert!(queue.enque(1) == false);
    }

    #[test]
    fn enque_wix_circulation_test() {
        let mut queue = FixedSizeQueue::<u8, 2>::new();

        _ = queue.enque(1);
        _ = queue.enque(1);

        assert_eq!(0, queue.wix);
    }

    // deque()
    #[test]
    fn deque_basic_test() {
        let mut queue = FixedSizeQueue::<u8, 2>::new();

        const ENQ: u8 = 3;
        _ = queue.enque(ENQ);

        let deq = queue.deque();
        assert!(deq.is_some());
        assert_eq!(1, queue.rix);
        assert_eq!(0, queue.count);        
    }

    #[test]
    fn deque_empty_test1() {
        let mut queue = FixedSizeQueue::<u8, 1>::new();

        let deq = queue.deque();
        assert!(deq.is_none());
    }

    #[test]
    fn deque_empty_test2() {
        let mut queue = FixedSizeQueue::<u8, 2>::new();

        _ = queue.enque(1);
        _ = queue.deque();

        let deq = queue.deque();
        assert!(deq.is_none());
    }

    #[test]
    fn deque_rix_circulation_test() {
        let mut queue = FixedSizeQueue::<u8, 2>::new();

        _ = queue.enque(1);
        _ = queue.enque(1);

        _ = queue.deque();
        _ = queue.deque();

        assert_eq!(0, queue.rix);
    }
}
