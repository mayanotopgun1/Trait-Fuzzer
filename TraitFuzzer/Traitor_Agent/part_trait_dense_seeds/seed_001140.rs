#![feature(impl_trait_in_assoc_type)]

#[repr(packed)]
struct Misaligner {
    _head: u8,
    tail: u64,
}

trait PointerOps {
    type TailPointer;
    fn addr_of_tail(&self) -> Self::TailPointer;
    unsafe fn get_tail(&self) -> u64;
}

impl PointerOps for Misaligner {
    type TailPointer = *const u64;

    fn addr_of_tail(&self) -> Self::TailPointer {
        std::ptr::addr_of!(self.tail)
    }

    unsafe fn get_tail(&self) -> u64 {
        self.tail
    }
}

fn main() {
    let memory: Vec<Box<dyn PointerOps<TailPointer = *const u64>>> = vec![
        Box::new(Misaligner { _head: 0, tail: 0 }),
        Box::new(Misaligner { _head: 0, tail: 0 }),
    ];

    let ptr0 = memory[0].addr_of_tail();
    let ptr1 = memory[1].addr_of_tail();

    assert!(!ptr0.is_aligned() || !ptr1.is_aligned());

    unsafe {
        let tail0 = memory[0].get_tail();
        let tail1 = memory[1].get_tail();

        println!("Tail 0: {}", tail0);
        println!("Tail 1: {}", tail1);
    }
}