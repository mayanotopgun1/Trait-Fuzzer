#![deny(break_with_label_and_loop)]

unsafe trait FooTrait { fn foo(&self) -> i32; }
unsafe impl FooTrait for () { fn foo(&self) -> i32 { 42 } }

fn main () {
    'label: loop {
        break 'label unsafe { <() as FooTrait>::foo(&()) };
    };
}