#![feature(allocator_api, specialization)]

trait NodeMut {
    fn set_first_byte(&mut self, value: u8);
}

default impl<T> NodeMut for T {
    default fn set_first_byte(&mut self, _value: u8) {}
}

impl<'a> NodeMut for Box<[u8; 1], &'a std::alloc::Global> {
    fn set_first_byte(&mut self, value: u8) {
        self[0] = value;
    }
}

#[inline(never)]
pub fn by_ref(node: &mut dyn NodeMut) {
    node.set_first_byte(9u8);
}

pub fn main() {
    let mut node = Box::new_in([5u8], &std::alloc::Global);
    node.set_first_byte(7u8);

    std::hint::black_box(&node);

    let mut node = Box::new_in([5u8], &std::alloc::Global);

    by_ref(&mut node);

    std::hint::black_box(&node);
}