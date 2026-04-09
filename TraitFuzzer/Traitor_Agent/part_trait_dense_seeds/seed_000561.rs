#![allow(non_camel_case_types)]
#![allow(stable_features)]



#![feature(core)]

struct ncint { v: isize }
fn ncint(v: isize) -> ncint { ncint { v: v } }

struct NoFoo { copied: isize, nocopy: ncint, }
impl NoFoo {
    fn new(x:isize,y:isize) -> NoFoo { NoFoo { copied: x, nocopy: ncint(y) } }
}

trait NoFooExt {
    fn update_copied(&mut self, value: isize);
    fn update_nocopy(&mut self, value: isize);
}

impl NoFooExt for NoFoo {
    fn update_copied(&mut self, value: isize) {
        self.copied = value;
    }

    fn update_nocopy(&mut self, value: isize) {
        self.nocopy = ncint(value);
    }
}

struct MoveFoo { copied: isize, moved: Box<isize>, }
impl MoveFoo {
    fn new(x:isize,y:isize) -> MoveFoo { MoveFoo { copied: x, moved: Box::new(y) } }
}

trait MoveFooExt {
    fn update_copied(&mut self, value: isize);
    fn update_moved(&mut self, value: isize);
}

impl MoveFooExt for MoveFoo {
    fn update_copied(&mut self, value: isize) {
        self.copied = value;
    }

    fn update_moved(&mut self, value: isize) {
        self.moved = Box::new(value);
    }
}

struct DropNoFoo { inner: NoFoo }
impl DropNoFoo {
    fn new(x:isize,y:isize) -> DropNoFoo { DropNoFoo { inner: NoFoo::new(x,y) } }
}
impl Drop for DropNoFoo { fn drop(&mut self) { } }

struct DropMoveFoo { inner: MoveFoo }
impl DropMoveFoo {
    fn new(x:isize,y:isize) -> DropMoveFoo { DropMoveFoo { inner: MoveFoo::new(x,y) } }
}
impl Drop for DropMoveFoo { fn drop(&mut self) { } }


fn test0() {





    let mut f = DropNoFoo::new(1, 2);
    let b = DropNoFoo { inner: NoFoo { nocopy: ncint(3), ..f.inner }};
    let c = DropNoFoo { inner: NoFoo { nocopy: ncint(4), ..f.inner }};
    assert_eq!(f.inner.copied,    1);
    assert_eq!(f.inner.nocopy.v, 2);

    assert_eq!(b.inner.copied,    1);
    assert_eq!(b.inner.nocopy.v, 3);

    assert_eq!(c.inner.copied,    1);
    assert_eq!(c.inner.nocopy.v, 4);


    let mut f = DropMoveFoo::new(5, 6);
    let b = DropMoveFoo { inner: MoveFoo { moved: Box::new(7), ..f.inner }};
    let c = DropMoveFoo { inner: MoveFoo { moved: Box::new(8), ..f.inner }};
    assert_eq!(f.inner.copied,    5);
    assert_eq!(*f.inner.moved,    6);

    assert_eq!(b.inner.copied,    5);
    assert_eq!(*b.inner.moved,    7);

    assert_eq!(c.inner.copied,    5);
    assert_eq!(*c.inner.moved,    8);
}

fn test1() {

    let mut f = MoveFoo::new(11, 12);

    let b = MoveFoo {moved: Box::new(13), ..f};
    let c = MoveFoo {copied: 14, ..f};
    assert_eq!(b.copied,    11);
    assert_eq!(*b.moved,    13);
    assert_eq!(c.copied,    14);
    assert_eq!(*c.moved,    12);
}

fn test2() {

    let mut f = NoFoo::new(21, 22);
    let b = NoFoo {nocopy: ncint(23), ..f};
    let c = NoFoo {copied: 24, ..f};
    assert_eq!(b.copied,    21);
    assert_eq!(b.nocopy.v, 23);
    assert_eq!(c.copied,    24);
    assert_eq!(c.nocopy.v, 22);
}

pub fn main() {
    test0();
    test1();
    test2();
}