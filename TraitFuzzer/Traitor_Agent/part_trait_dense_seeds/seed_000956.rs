macro_rules! m1 {
    ($(#[$meta:meta])* { $e:expr }) => {
        m1! { expr: { $e }, unprocessed: [$(#[$meta])*] }
    };

    (expr: { $e:expr }, unprocessed: [ #[$meta:meta] $($metas:tt)* ]) => {
        m1! { expr: { $e }, unprocessed: [ $($metas)* ] }
    };

    (expr: { $e:expr }, unprocessed: []) => {
        { $e }
    }
}

macro_rules! m2 {
    ($(#[$meta:meta])* { $e:stmt }) => {
        m2! { stmt: { $e }, unprocessed: [$(#[$meta])*] }
    };

    (stmt: { $e:stmt }, unprocessed: [ #[$meta:meta] $($metas:tt)* ]) => {
        m2! { stmt: { $e }, unprocessed: [ $($metas)* ] }
    };

    (stmt: { $e:stmt }, unprocessed: []) => {
        { $e }
    }
}

macro_rules! m3 {
    ($(#[$meta:meta])* { $e:item }) => {
        m3! { item: { $e }, unprocessed: [$(#[$meta])*] }
    };

    (item: { $e:item }, unprocessed: [ #[$meta:meta] $($metas:tt)* ]) => {
        m3! { item: { $e }, unprocessed: [ $($metas)* ] }
    };

    (item: { $e:item }, unprocessed: []) => {
        { $e }
    }
}

trait Executable {
    fn execute(&self);
}

impl Executable for i32 {
    fn execute(&self) {}
}

trait Statement {
    fn run(&self);
}

impl Statement for () {
    fn run(&self) {}
}

trait Item {
    fn define(&self);
}

impl Item for () {
    fn define(&self) {}
}

fn main() {

    m1!(




















        {
            #[allow(deprecated)] 0
        }
    ).execute();

    m2!(




















        {
            #[allow(deprecated)] let x = 5
        }
    ).run();

    m3!(




















        {
            #[allow(deprecated)] struct S;
        }
    ).define();
}