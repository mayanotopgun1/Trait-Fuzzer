#![allow(dead_code)]

struct SomeStruct<T>(T);

trait StructTransform<'min, 'max> {
    fn transform(v: Self) -> SomeStruct<&'min ()>
    where
        'max: 'min;
}

impl<'min, 'max> StructTransform<'min, 'max> for SomeStruct<&'max ()> {
    fn transform(v: Self) -> SomeStruct<&'min ()>
    where
        'max: 'min,
    {
        v
    }
}

fn main() {}