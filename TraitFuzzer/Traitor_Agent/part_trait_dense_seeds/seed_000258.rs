#![warn(unused)]




#[expect(unused_variables, unused_mut, while_true)]



trait UnusedChecker {
    fn check(&self);
}

impl UnusedChecker for () {
    fn check(&self) {}
}

fn check_multiple_lints_1() -> impl UnusedChecker {
    let who_am_i = 666;
    ()
}

#[expect(unused_variables, unused_mut, while_true)]


trait MutChecker {
    fn check(&mut self);
}

impl MutChecker for u32 {
    fn check(&mut self) {}
}

fn check_multiple_lints_2() -> impl MutChecker {
    let mut x = 0;
    println!("I use x: {}", x);
    x
}

#[expect(unused_variables, unused_mut, while_true)]


trait LoopChecker {
    fn check(&self);
}

impl LoopChecker for () {
    fn check(&self) {}
}

fn check_multiple_lints_3() -> impl LoopChecker {
    while true {}
    ()
}

#[expect(unused, while_true)]

trait GroupedChecker1 {
    fn check(&self);
}

impl GroupedChecker1 for () {
    fn check(&self) {}
}

fn check_multiple_lints_with_lint_group_1() -> impl GroupedChecker1 {
    let who_am_i = 666;

    let mut x = 0;
    println!("I use x: {}", x);
    ()
}

#[expect(unused, while_true)]

trait GroupedChecker2 {
    fn check(&self);
}

impl GroupedChecker2 for () {
    fn check(&self) {}
}

fn check_multiple_lints_with_lint_group_2() -> impl GroupedChecker2 {
    while true {}
    ()
}

fn main() {
    let _ = check_multiple_lints_1().check();
    let mut x = 0;
    x.check();
    let _ = check_multiple_lints_3().check();

    let _ = check_multiple_lints_with_lint_group_1().check();
    let _ = check_multiple_lints_with_lint_group_2().check();
}