#![allow(irrefutable_let_patterns, dropping_references)]
struct M;

trait DropTrait {
    fn drop_self(&self);
}

impl DropTrait for M {
    fn drop_self(&self) {}
}

trait BorrowTrait {
    fn borrow_self(&mut self) -> &mut Self;
}

impl BorrowTrait for M {
    fn borrow_self(&mut self) -> &mut Self {
        self
    }
}

fn let_wild_gets_moved_expr() {
    let m = M;
    m.drop_self();
    let _ = m;

    let mm = (M, M);
    let (_x, _) = mm;
    let (_, _y) = mm;
    let (_, _) = mm;
}

fn match_moved_expr_to_wild() {
    let m = M;
    m.drop_self();
    match m { _ => {} }

    let mm = (M, M);
    match mm { (_x, _) => {} }
    match mm { (_, _y) => {} }
    match mm { (_, _) => {} }
}

fn if_let_moved_expr_to_wild() {
    let m = M;
    m.drop_self();
    if let _ = m {}

    let mm = (M, M);
    if let (_x, _) = mm {}
    if let (_, _y) = mm {}
    if let (_, _) = mm {}
}

fn let_wild_gets_borrowed_expr() {
    let mut m = M;
    let r = m.borrow_self();
    let _ = m;

    drop(r);

    let mut mm = (M, M);
    let (r1, r2) = (mm.0.borrow_self(), mm.1.borrow_self());
    let (_, _) = mm;
    drop((r1, r2));
}

fn match_borrowed_expr_to_wild() {
    let mut m = M;
    let r = m.borrow_self();
    match m { _ => {} }
    drop(r);

    let mut mm = (M, M);
    let (r1, r2) = (mm.0.borrow_self(), mm.1.borrow_self());
    match mm { (_, _) => {} }
    drop((r1, r2));
}

fn if_let_borrowed_expr_to_wild() {
    let mut m = M;
    let r = m.borrow_self();
    if let _ = m {}
    drop(r);

    let mut mm = (M, M);
    let (r1, r2) = (mm.0.borrow_self(), mm.1.borrow_self());
    if let (_, _) = mm {}
    drop((r1, r2));
}

fn main() {}