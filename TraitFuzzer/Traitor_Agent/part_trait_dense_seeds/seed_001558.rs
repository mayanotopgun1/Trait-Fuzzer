#[derive(Debug)]
struct Dropable(&'static str);

impl Drop for Dropable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}

trait DroppableTrait {
    fn print_drop(&self);
}

impl DroppableTrait for Dropable {
    fn print_drop(&self) {
        println!("Dropping {}", self.0)
    }
}

#[derive(Debug)]
struct A {
    x: Dropable,
    y: Dropable,
}

impl A {
    fn print_fields(&self) {
        println!("{:?} {:?}", self.y, self.x);
    }
}

trait ATrait {
    fn print_fields(&self);
}

impl ATrait for A {
    fn print_fields(&self) {
        println!("{:?} {:?}", self.y, self.x);
    }
}

#[derive(Debug)]
struct B {
    c: A,
    d: A,
}

impl B {
    fn print_fields(&self) {
        println!("{:?} {:?} {:?} {:?}", self.d.y, self.d.x, self.c.y, self.c.x);
    }
}

trait BTrait {
    fn print_fields(&self);
}

impl BTrait for B {
    fn print_fields(&self) {
        println!("{:?} {:?} {:?} {:?}", self.d.y, self.d.x, self.c.y, self.c.x);
    }
}

#[derive(Debug)]
struct R<'a> {
    c: &'a A,
    d: &'a A,
}

impl<'a> R<'a> {
    fn print_fields(&self) {
        println!("{:?} {:?} {:?} {:?}", self.d.y, self.d.x, self.c.y, self.c.x);
    }
}

trait RTrait<'a> {
    fn print_fields(&self);
}

impl<'a> RTrait<'a> for R<'a> {
    fn print_fields(&self) {
        println!("{:?} {:?} {:?} {:?}", self.d.y, self.d.x, self.c.y, self.c.x);
    }
}

fn main() {
    let a = A { x: Dropable("x"), y: Dropable("y") };

    let c = move || a.print_fields();

    c();

    let b = B {
        c: A { x: Dropable("b.c.x"), y: Dropable("b.c.y") },
        d: A { x: Dropable("b.d.x"), y: Dropable("b.d.y") },
    };

    let d = move || b.print_fields();

    d();

    let r = R {
        c: &A { x: Dropable("r.c.x"), y: Dropable("r.c.y") },
        d: &A { x: Dropable("r.d.x"), y: Dropable("r.d.y") },
    };

    let e = move || r.print_fields();

    e();
}