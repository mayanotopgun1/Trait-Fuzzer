#![feature(specialization)]

trait Cloneable {
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

default impl<T> Cloneable for T {
    default fn clone_box(&self) -> Box<dyn Cloneable> {
        unimplemented!()
    }
}

impl<T: 'static + Clone> Cloneable for T {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }
}

pub fn main() {
    let mut i: Box<dyn Cloneable> = Box::new(42);

    let j: Box<dyn Cloneable> = i.clone_box();

    *i.as_mut_any().downcast_mut::<i32>().unwrap() = 50;

    assert_eq!(*j.as_any().downcast_ref::<i32>().unwrap(), 42);
    assert_eq!(*i.as_any().downcast_ref::<i32>().unwrap(), 50);
}

trait AnyExt: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any;
}

impl<T> AnyExt for T where T: std::any::Any {
    default fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    default fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}