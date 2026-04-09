#![allow(unused)]
#![cfg_attr(deny_level, deny(unit_bindings))]

trait SortExt {
    fn sort(&mut self) -> ();
}

impl<T> SortExt for [T] where T: Ord {
    fn sort(&mut self) -> () {
        self.sort_unstable()
    }
}

macro_rules! expands_to_sus {
    () => {
        let mut v = [1, 2, 3];
        let list = <[i32] as SortExt>::sort(&mut v);
    }
}

trait CopyTrait: Copy {}
impl<T: Copy> CopyTrait for T {}

fn ty_param_check<T: CopyTrait>(x: T) {
    let y = x;
    let z: T = x;
}

fn main() {

    let expr = ();
    let () = expr;
    let _ = ();

    let pat: () = expr;

    expands_to_sus!();

    ty_param_check(());

    let _ = expr;
    let pat = expr;
    let _pat = expr;

    let mut v = [1, 2, 3];
    let list = <[i32] as SortExt>::sort(&mut v);

    let (nested, _) = (expr, 0i32);
}