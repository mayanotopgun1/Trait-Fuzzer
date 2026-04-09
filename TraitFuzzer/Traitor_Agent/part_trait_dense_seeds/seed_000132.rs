use std::mem;

#[derive(Copy, Clone)]
enum Never { }

trait UnionInspect {
    unsafe fn inspect_u64(&self) -> u64;
}

union Foo {
    a: u64,
    _b: Never
}

impl UnionInspect for Foo {
    unsafe fn inspect_u64(&self) -> u64 {
        self.a
    }
}

union Bar {
    _a: (Never, u64),
    _b: (u64, Never)
}

impl UnionInspect for Bar {
    unsafe fn inspect_u64(&self) -> u64 {
        match *self {
            Bar { _a: (_, a) } => a,
            Bar { _b: (b, _) } => b,
        }
    }
}

fn main() {
    assert_eq!(mem::size_of::<Foo>(), 8);

    assert_eq!(mem::size_of::<Bar>(), 8);

    let f = vec![Box::new(Foo { a: 42 }), Box::new(Foo { a: 10 })];
    println!("{}", unsafe { f[0].inspect_u64() });
    assert_eq!(unsafe { f[1].inspect_u64() }, 10);
}