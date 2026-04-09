pub trait MyTrait<'a> {
    type Output: 'a;
    fn gimme_value(&self) -> Self::Output;
}

trait MyTraitExt<'a>: MyTrait<'a>
where
    Self::Output: std::fmt::Debug,
{
    fn print_value(&self) {
        let value = self.gimme_value();
        println!("{:?}", value);
    }
}

impl<'a, T> MyTraitExt<'a> for T where T: MyTrait<'a>, <T as MyTrait<'a>>::Output: std::fmt::Debug {}

pub struct MyStruct;

impl<'a> MyTrait<'a> for MyStruct {
    type Output = &'a usize;
    fn gimme_value(&self) -> Self::Output {
        unimplemented!()
    }
}

fn meow<T, F>(t: &T, f: F)
where
    T: for<'any> MyTrait<'any>,
    F: for<'any2> Fn(<T as MyTrait<'any2>>::Output),
{
    let v = t.gimme_value();
    f(v);
}

fn main() {
    let struc = MyStruct;
    meow(&struc, |foo| {
        println!("{:?}", foo);
    });

    struc.print_value();
}