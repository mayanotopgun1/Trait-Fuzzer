trait ArrayMaker {
    type Out;
    fn make_array() -> Self::Out;
}

impl ArrayMaker for () {
    type Out = [(); { |x: u32| x; 4 }];
    fn make_array() -> Self::Out {
        todo!()
    }
}

fn foo() -> <() as ArrayMaker>::Out {
    <() as ArrayMaker>::make_array()
}

fn bar() {
    let _: <() as ArrayMaker>::Out;
}

unsafe fn unsf() {}

trait UnsafeArrayMaker {
    type Out;
    unsafe fn make_unsafe_array() -> Self::Out;
}

impl UnsafeArrayMaker for () {
    type Out = [(); { unsafe { || { unsf() } }; 4 }];
    unsafe fn make_unsafe_array() -> Self::Out {
        todo!()
    }
}

fn bad2<T>() -> Box<dyn Iterator<Item = <T as UnsafeArrayMaker>::Out>>
where
    T: UnsafeArrayMaker,
{
    todo!()
}

fn main() {}