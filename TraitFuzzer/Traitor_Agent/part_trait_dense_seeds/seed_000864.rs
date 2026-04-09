#![feature(generic_associated_types)]
#![allow(unused_unsafe)]
#![allow(while_true)]

trait Looper {
    type Iter<'a> where Self: 'a;
    fn loop_once(&self);
}

impl Looper for i32 {
    type Iter<'a> = &'a i32;

    fn loop_once(&self) {
        if *self == 1 {
            loop { break; }
        } else if *self == 2 {
            while true { break; }
        }
    }
}

trait Conditional {
    type Cond<'a> where Self: 'a;
    fn conditional_action(&self);
}

impl Conditional for i32 {
    type Cond<'a> = &'a bool;

    fn conditional_action(&self) {
        match self {
            3 => (),
            4 => if true { () } else { () },
            _ => ()
        }
    }
}

trait Matcher {
    type Match<'a> where Self: 'a;
    fn match_action(&self);
}

impl Matcher for i32 {
    type Match<'a> = &'a i32;

    fn match_action(&self) {
        match self {
            5 => match () { () => () },
            6 => (),
            7 => unsafe { () },
            _ => ()
        }
    }
}

trait ActionExt: Looper + Conditional + Matcher {}

impl<T> ActionExt for T where T: Looper + Conditional + Matcher {}

fn main() {
    let x = 1;
    x.loop_once();
    x.conditional_action();
    x.match_action();

    let r: &i32 = &x;

    match r {
        &1 => if true { () } else { () },
        &2 => (),
        _ => ()
    }
}