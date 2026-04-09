use std::mem;

#[repr(packed)]
#[allow(dead_code)]
struct S4(u8,[u8; 3]);

#[repr(packed)]
#[allow(dead_code)]
struct S5(u8,u32);

trait TransmuteTo<T> {
    unsafe fn transmute_to(self) -> T;
}

impl TransmuteTo<[u8; 4]> for S4 {
    unsafe fn transmute_to(self) -> [u8; 4] {
        mem::transmute(self)
    }
}

impl TransmuteTo<[u8; 5]> for S5 {
    unsafe fn transmute_to(self) -> [u8; 5] {
        mem::transmute(self)
    }
}

pub fn main() {
    unsafe {
        let s4 = S4(1, [2,3,4]);
        let transd : [u8; 4] = s4.transmute_to();
        assert_eq!(transd, [1, 2, 3, 4]);

        let s5 = S5(1, 0xff_00_00_ff);
        let transd : [u8; 5] = s5.transmute_to();

        assert_eq!(transd, [1, 0xff, 0, 0, 0xff]);
    }
}