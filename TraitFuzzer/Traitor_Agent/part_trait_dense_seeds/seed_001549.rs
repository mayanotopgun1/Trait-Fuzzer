#![allow(static_mut_refs)]
#![feature(specialization)]

use std::mem;

static mut DROP_COUNT: usize = 0;

struct Fragment;

impl Drop for Fragment {
    fn drop(&mut self) {
        unsafe {
            DROP_COUNT += 1;
        }
    }
}

trait DropCounter {
    fn increment_drop_count();
}

default impl<T> DropCounter for T {
    default fn increment_drop_count() {}
}

impl DropCounter for Fragment {
    fn increment_drop_count() {
        unsafe {
            DROP_COUNT += 1;
        }
    }
}

fn main() {
    {
        let mut fragments = vec![Fragment, Fragment, Fragment];
        let _new_fragments: Vec<Fragment> = mem::replace(&mut fragments, vec![])
            .into_iter()
            .skip_while(|_fragment| {
                true
            }).collect();
    }
    unsafe {
        assert_eq!(DROP_COUNT, 3);
    }
}