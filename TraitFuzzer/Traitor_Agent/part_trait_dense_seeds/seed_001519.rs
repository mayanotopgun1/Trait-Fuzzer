#![feature(return_position_impl_trait_in_trait)]

trait SplitLast { fn split_last(&self) -> Option<(&i32, &i32)>; }
impl SplitLast for () { fn split_last(&self) -> Option<(&i32, &i32)> { None } }

fn assign_twice() {
    loop {
        match () {
            #[allow(irrefutable_let_patterns)]
            _ if let _ = <() as SplitLast>::split_last(&()) => {}
            _ => {}
        }
    }
}

fn main() {}