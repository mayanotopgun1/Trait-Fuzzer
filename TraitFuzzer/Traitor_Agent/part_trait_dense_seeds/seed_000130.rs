#![feature(return_position_impl_trait_in_trait)]
#![feature(generic_associated_types)]

use std::thread;

trait Task<'a> {
    type Item;
    fn execute(self);
}

trait TaskExt<'a>: Task<'a> {
    fn execute_twice(&self) -> impl core::fmt::Debug;
}

impl<'a, T: Task<'a> + Clone> TaskExt<'a> for T {
    fn execute_twice(&self) -> impl core::fmt::Debug {
        self.clone().execute();
        self.clone().execute()
    }
}

impl<'a> Task<'a> for isize {
    type Item = &'a isize;
    fn execute(self) {
        println!("{}", self);
        assert_eq!(self, 10);
    }
}

pub fn main() {
    thread::spawn(move || (10 as isize).execute_twice()).join().ok().unwrap();
}

fn child(i: isize) {
    i.execute_twice();
}