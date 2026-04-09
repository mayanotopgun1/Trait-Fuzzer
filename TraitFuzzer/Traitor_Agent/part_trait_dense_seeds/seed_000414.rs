#![feature(impl_trait_in_assoc_type)]

trait UnionAccess {
    type FieldA<'a> where Self: 'a;
    type FieldB<'a> where Self: 'a;

    fn get_field_a<'a>(&'a self) -> Self::FieldA<'a>;
    fn set_field_a<'a>(&'a mut self, value: Self::FieldA<'a>);
    fn get_field_b<'a>(&'a self) -> Self::FieldB<'a>;
    fn set_field_b<'a>(&'a mut self, value: Self::FieldB<'a>);
}

union U {
    a: (u8, u8),
    b: u16,
}

impl UnionAccess for U {
    type FieldA<'a> = &'a (u8, u8);
    type FieldB<'a> = &'a u16;

    fn get_field_a<'a>(&'a self) -> Self::FieldA<'a> {
        unsafe { &self.a }
    }

    fn set_field_a<'a>(&'a mut self, value: Self::FieldA<'a>) {
        unsafe { self.a = *value; }
    }

    fn get_field_b<'a>(&'a self) -> Self::FieldB<'a> {
        unsafe { &self.b }
    }

    fn set_field_b<'a>(&'a mut self, value: Self::FieldB<'a>) {
        unsafe { self.b = *value; }
    }
}

union V {
    c: u32,
    d: f64,
}

impl UnionAccess for V {
    type FieldA<'a> = &'a u32;
    type FieldB<'a> = &'a f64;

    fn get_field_a<'a>(&'a self) -> Self::FieldA<'a> {
        unsafe { &self.c }
    }

    fn set_field_a<'a>(&'a mut self, value: Self::FieldA<'a>) {
        unsafe { self.c = *value; }
    }

    fn get_field_b<'a>(&'a self) -> Self::FieldB<'a> {
        unsafe { &self.d }
    }

    fn set_field_b<'a>(&'a mut self, value: Self::FieldB<'a>) {
        unsafe { self.d = *value; }
    }
}

fn main() {
    let mut u = U { a: (1, 2) };
    println!("Before setting field b: {:?}", u.get_field_a());
    u.set_field_b(&5);
    println!("After setting field b: {:?}", u.get_field_a());

    let mut v = V { c: 42 };
    println!("Before setting field d: {:?}", v.get_field_a());
    v.set_field_b(&3.14);
    println!("After setting field d: {:?}", v.get_field_a());
}