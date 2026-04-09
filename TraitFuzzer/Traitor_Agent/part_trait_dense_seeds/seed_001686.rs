#![feature(const_trait_impl, const_ops, const_cmp)]

const trait IntTrait {
    fn get(&self) -> i32;
}

impl const IntTrait for Int {
    fn get(&self) -> i32 {
        self.0
    }
}

struct Int(i32);

impl const std::ops::Add for Int {
    type Output = Int;

    fn add(self, rhs: Self) -> Self {
        Int(self.get() + rhs.get())
    }
}

impl const PartialEq for Int {
    fn eq(&self, rhs: &Self) -> bool {
        self.get() == rhs.get()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub const trait Plus {
    fn plus(self, rhs: Self) -> Self;
}

impl const Plus for i32 {
    fn plus(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub const fn add_i32(a: i32, b: i32) -> i32 {
    a.plus(b)
}

const ADD_INT: Int = Int(1i32) + Int(2i32);

fn main() {
    assert!(ADD_INT == Int(3i32));
}