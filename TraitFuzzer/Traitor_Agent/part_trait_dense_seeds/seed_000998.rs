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
    let m = Box::new(M);
    m.drop_self();
    let _ = m;

    let mm = (Box::new(M), Box::new(M));
    let (_x, _) = mm;
    let (_, _y) = mm;
    let (_, _) = mm;
}

fn match_moved_expr_to_wild() {
    let m = Box::new(M);
    m.drop_self();
    match *m { _ => {} }

    let mm = (Box::new(M), Box::new(M));
    match *(mm.0) { _x => {} }
    match *(mm.1) { _y => {} }
    match *(mm.0) { _ => {} }
}

fn if_let_moved_expr_to_wild() {
    let m = Box::new(M);
    m.drop_self();
    if let _ = *m {}

    let mm = (Box::new(M), Box::new(M));
    if let _x = *(mm.0) {}
    if let _y = *(mm.1) {}
    if let _ = *(mm.0) {}
}

fn let_wild_gets_borrowed_expr() {
    let mut m = Box::new(M);
    let r = m.borrow_self();
    let _ = m;

    drop(r);

    let mut mm = (Box::new(M), Box::new(M));
    let (r1, r2) = (mm.0.borrow_self(), mm.1.borrow_self());
    let (_, _) = mm;
    drop((r1, r2));
}

fn match_borrowed_expr_to_wild() {
    let mut m = Box::new(M);
    let r = m.borrow_self();
    match *m { _ => {} }
    drop(r);

    let mut mm = (Box::new(M), Box::new(M));
    let (r1, r2) = (mm.0.borrow_self(), mm.1.borrow_self());
    match *(mm.0) { _ => {} }
    drop((r1, r2));
}

fn if_let_borrowed_expr_to_wild() {
    let mut m = Box::new(M);
    let r = m.borrow_self();
    if let _ = *m {}
    drop(r);

    let mut mm = (Box::new(M), Box::new(M));
    let (r1, r2) = (mm.0.borrow_self(), mm.1.borrow_self());
    if let _ = *(mm.0) {}
    drop((r1, r2));
}

fn main() {}