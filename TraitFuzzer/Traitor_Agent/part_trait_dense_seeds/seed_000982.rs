#[inline(never)]
fn cmp(a: usize, b: usize) -> bool {
    a == b
}

trait CmpExt {
    fn cmp_ext(&self, other: &Self) -> bool;
}

impl CmpExt for usize {
    fn cmp_ext(&self, other: &Self) -> bool {
        self == other
    }
}

#[inline(always)]
fn cmp_in(a: usize, b: usize) -> bool {
    a == b
}

trait CmpInExt {
    fn cmp_in_ext(&self, other: &Self) -> bool;
}

impl CmpInExt for usize {
    fn cmp_in_ext(&self, other: &Self) -> bool {
        self == other
    }
}

fn main() {
    let a = {
        let v = 0;
        &v as *const _ as usize
    };
    let b = {
        let v = 0;
        &v as *const _ as usize
    };
    assert_eq!(format!("{a}"), format!("{b}"));
    assert_eq!(format!("{}", a == b), "true");
    assert_eq!(format!("{}", CmpInExt::cmp_in_ext(&a, &b)), "true");
    assert_eq!(format!("{}", CmpExt::cmp_ext(&a, &b)), "true");
}