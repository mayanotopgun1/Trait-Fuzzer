static mut FLAGS: u64 = 0;

struct AddFlags {
    bits: u64,
}

impl Drop for AddFlags {
    fn drop(&mut self) {
        unsafe {
            FLAGS += self.bits;
        }
    }
}

trait FlagChecker {
    fn check_flags(expected: u64);
}

impl FlagChecker for () {
    fn check_flags(expected: u64) {
        unsafe {
            let actual = FLAGS;
            FLAGS = 0;
            assert_eq!(actual, expected, "flags {}, expected {}", actual, expected);
        }
    }
}

fn main() {
    for _ in &[AddFlags { bits: 1 }] {
        <()>::check_flags(0);
    }
    <()>::check_flags(1);
}