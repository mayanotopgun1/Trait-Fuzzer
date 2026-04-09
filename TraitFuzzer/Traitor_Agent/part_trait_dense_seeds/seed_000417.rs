trait ItemMaker { fn item(&self); }
struct I;
impl ItemMaker for I { fn item(&self) {} }

macro_rules! make_item {
    () => { fn f() { let _i = I; _i.item(); } }
}

trait StmtMaker { fn stmt(&self); }
struct S;
impl StmtMaker for S { fn stmt(&self) { let x = 0; } }

macro_rules! make_stmt {
    () => { fn g() { let _s = S; _s.stmt(); } }
}

fn f() {
    make_item! {}
}

fn g() {
    make_stmt! {}
}

fn main() {}