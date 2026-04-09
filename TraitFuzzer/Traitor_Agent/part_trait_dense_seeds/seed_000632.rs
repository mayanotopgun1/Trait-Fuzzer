static mut S: *const u8 = unsafe { &S as *const *const u8 as *const u8 };

struct StaticDoubleLinked {
    prev: &'static StaticDoubleLinked,
    next: &'static StaticDoubleLinked,
    data: i32,
    head: bool,
}

trait LinkedListTraversal {
    fn traverse_next(&self) -> Option<&'static Self>;
    fn traverse_prev(&self) -> Option<&'static Self>;
    fn is_head(&self) -> bool;
}

impl LinkedListTraversal for StaticDoubleLinked {
    fn traverse_next(&self) -> Option<&'static Self> {
        if self.head {
            None
        } else {
            Some(self.next)
        }
    }

    fn traverse_prev(&self) -> Option<&'static Self> {
        if self.head {
            None
        } else {
            Some(self.prev)
        }
    }

    fn is_head(&self) -> bool {
        self.head
    }
}

static L1: StaticDoubleLinked = StaticDoubleLinked { prev: &L3, next: &L2, data: 1, head: true };
static L2: StaticDoubleLinked = StaticDoubleLinked { prev: &L1, next: &L3, data: 2, head: false };
static L3: StaticDoubleLinked = StaticDoubleLinked { prev: &L2, next: &L1, data: 3, head: false };

pub fn main() {
    unsafe {
        assert_eq!(S, *(S as *const *const u8));
    }

    let mut test_vec = Vec::new();
    let mut cur = &L1;
    loop {
        test_vec.push(cur.data);
        if let Some(next) = cur.traverse_next() {
            cur = next;
        } else {
            break;
        }
    }
    assert_eq!(&test_vec, &[1, 2, 3]);

    let mut test_vec = Vec::new();
    let mut cur = &L1;
    loop {
        if let Some(prev) = cur.traverse_prev() {
            cur = prev;
            test_vec.push(cur.data);
        } else {
            break;
        }
    }
    assert_eq!(&test_vec, &[3, 2, 1]);
}