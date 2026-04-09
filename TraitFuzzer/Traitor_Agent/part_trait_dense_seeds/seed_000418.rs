trait ItemMaker { fn item(&self); }
#[derive(Debug)]
struct I;
impl ItemMaker for I { fn item(&self) {} }

macro_rules! make_item {
    () => { fn f() -> impl core::fmt::Debug { let _i = I; _i.item(); _i } }
}

trait StmtMaker { fn stmt(&self); }
#[derive(Debug)]
struct S;
impl StmtMaker for S { fn stmt(&self) { let x = 0; } }

macro_rules! make_stmt {
    () => { fn g() -> impl core::fmt::Debug { let _s = S; _s.stmt(); _s } }
}

fn f() -> impl core::fmt::Debug {
    make_item! {}
}

fn g() -> impl core::fmt::Debug {
    make_stmt! {}
}

fn main() {}